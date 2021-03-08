use std::cell::RefCell;

use lazy_static::lazy_static;
use parking_lot::{ReentrantMutex, ReentrantMutexGuard};

use vex_sys;

pub use vex_sys::{Addr, VexArch as Arch, VexEndness};

pub mod ir;
mod logger;

// arch specific data:
pub mod x86;
pub mod amd64;
pub mod ppc32;
pub mod ppc64;
pub mod arm;
pub mod arm64;
pub mod s390x;
pub mod mips32;
pub mod mips64;

unsafe extern "C" fn failure_exit() {
    panic!("LibVEX encountered a critical error.")
}

unsafe extern "C" fn log_bytes(bytes: *const libc::c_char, nbytes: u64) {
    let bytes = std::slice::from_raw_parts(bytes as *const u8, nbytes as usize);
    let log = logger::VEX_LOG.lock();
    let _ = std::io::Write::write(&mut *log.borrow_mut(), bytes);
}

fn init() {
    use std::sync::Once;
    use vex_sys::LibVEX_Init;

    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let mut vcon = crate::VexControl::default();
        unsafe {
            LibVEX_Init(Some(failure_exit), Some(log_bytes), 3, &mut vcon.0);
        }
    })
}

pub struct ArchInfo(pub vex_sys::VexArchInfo);

impl Default for ArchInfo {
    fn default() -> Self {
        let mut this = std::mem::MaybeUninit::uninit();
        Self(unsafe {
            vex_sys::LibVEX_default_VexArchInfo(this.as_mut_ptr());
            this.assume_init()
        })
    }
}

pub struct AbiInfo(pub vex_sys::VexAbiInfo);

impl Default for AbiInfo {
    fn default() -> Self {
        let mut this = std::mem::MaybeUninit::uninit();
        Self(unsafe {
            vex_sys::LibVEX_default_VexAbiInfo(this.as_mut_ptr());
            this.assume_init()
        })
    }
}

pub struct VexControl(pub vex_sys::VexControl);

impl Default for VexControl {
    fn default() -> Self {
        let mut this = std::mem::MaybeUninit::uninit();
        Self(unsafe {
            vex_sys::LibVEX_default_VexControl(this.as_mut_ptr());
            this.assume_init()
        })
    }
}

use libc::c_void;
unsafe extern "C" fn return_0(
    _cb: *mut c_void,
    _px_control: *mut vex_sys::VexRegisterUpdates,
    _vge: *const vex_sys::VexGuestExtents,
) -> u32 {
    0
}

unsafe extern "C" fn return_false(_cb: *mut c_void, _addr: u64) -> u8 {
    0
}

unsafe extern "C" fn failure_disp() {
    panic!("LibVEX called the display function.")
}

pub struct TranslateArgs(pub vex_sys::VexTranslateArgs);

impl TranslateArgs {
    pub fn new(arch_guest: Arch, arch_host: Arch, endness: VexEndness) -> Self {
        let abiinfo_both = AbiInfo::default();
        let archinfo_guest = ArchInfo::default();

        Self(vex_sys::VexTranslateArgs {
            abiinfo_both: vex_sys::VexAbiInfo {
                // Use some values that makes AMD64 happy.
                guest_stack_redzone_size: 128,
                ..abiinfo_both.0
            },

            // Prepare first for a translation where guest == host
            // We will translate the sanity test function
            arch_guest: arch_guest.into(),
            arch_host: arch_host.into(),

            archinfo_guest: vex_sys::VexArchInfo {
                // Use some values that makes ARM64 happy.
                arm64_dMinLine_lg2_szB: 6,
                arm64_iMinLine_lg2_szB: 6,
                endness,
                hwcaps: 0,
                ..archinfo_guest.0
            },
            archinfo_host: vex_sys::VexArchInfo {
                endness,
                hwcaps: 0,
                ..archinfo_guest.0
            },
            callback_opaque: std::ptr::null_mut(),
            guest_bytes: std::ptr::null(),
            guest_bytes_addr: 0,
            guest_extents: std::ptr::null_mut(),
            chase_into_ok: Some(return_false),
            host_bytes: std::ptr::null_mut(),
            host_bytes_size: 0,
            host_bytes_used: std::ptr::null_mut(),
            instrument1: None,
            instrument2: None,
            finaltidy: None,
            needs_self_check: Some(return_0),
            preamble_function: None,
            traceflags: 0,
            sigill_diag: 0,
            addProfInc: 0,
            // When only calling the FrontEnd, these can be null
            disp_cp_chain_me_to_slowEP: std::ptr::null(),
            disp_cp_chain_me_to_fastEP: std::ptr::null(),
            disp_cp_xindir: std::ptr::null(),
            disp_cp_xassisted: failure_disp as *const _,
        })
    }

    /// Call VEX's front-end method, LibVEX_FrontEnd.
    ///
    /// The IRSB returned doesn't actually need the same lifetime as `self`,
    /// but this helps prevent calling the front-end in a way that would
    /// invalidate IRSBs that are still in use, with a compile time check.
    pub fn front_end(
        &mut self,
        guest_bytes: *const u8,
        guest_bytes_addr: u64,
    ) -> Result<ir::IRSB, ()> {
        use std::mem::MaybeUninit;
        init();

        let mut vtr = MaybeUninit::<vex_sys::VexTranslateResult>::uninit();
        let mut ge = MaybeUninit::<vex_sys::VexGuestExtents>::uninit();
        self.0.guest_extents = ge.as_mut_ptr();
        let mut host_bytes: [u8; 100] = [0; 100];
        let mut host_bytes_used = 0;
        self.0.host_bytes = host_bytes.as_mut_ptr();
        self.0.host_bytes_size = host_bytes.len() as i32;
        self.0.host_bytes_used = &mut host_bytes_used;
        self.0.guest_bytes = guest_bytes;
        self.0.guest_bytes_addr = guest_bytes_addr;

        let _lock = LIFT_LOCK.lock();
        let irsb = unsafe {
            vex_sys::LibVEX_FrontEnd(
                &mut self.0,
                vtr.as_mut_ptr(),
                #[allow(const_item_mutation)]
                &mut vex_sys::VexRegisterUpdates::VexRegUpd_INVALID)
        };
        let vtr = unsafe { vtr.assume_init() };

        match vtr.status {
            vex_sys::VexTranslateResult_VexTransOK => Ok(ir::IRSB {
                inner: unsafe { &*irsb },
                _lock,
            }),
            vex_sys::VexTranslateResult_VexTransAccessFail => Err(()),
            vex_sys::VexTranslateResult_VexTransOutputFull => Err(()),
        }
    }

    /// Call VEX's translate method, LibVEX_Translate.
    pub fn translate(
        &mut self,
        guest_bytes: *const u8,
        guest_bytes_addr: u64,
        host_bytes: &mut [u8],
    ) -> Result<i32, ()> {
        use std::mem::MaybeUninit;
        init();

        let mut ge = MaybeUninit::<vex_sys::VexGuestExtents>::uninit();
        self.0.guest_extents = ge.as_mut_ptr();
        let mut host_bytes_used = 0;
        self.0.host_bytes = host_bytes.as_mut_ptr();
        self.0.host_bytes_size = host_bytes.len() as i32;
        self.0.host_bytes_used = &mut host_bytes_used;
        self.0.guest_bytes = guest_bytes;
        self.0.guest_bytes_addr = guest_bytes_addr;

        let _lock = LIFT_LOCK.lock();
        let vtr = unsafe { vex_sys::LibVEX_Translate(&mut self.0) };

        match vtr.status {
            vex_sys::VexTranslateResult_VexTransOK => Ok(host_bytes_used),
            vex_sys::VexTranslateResult_VexTransAccessFail => Err(()),
            vex_sys::VexTranslateResult_VexTransOutputFull => Err(()),
        }
    }
}

// VEX uses a static buffer (named `temporary`, in main_globals.c) for the
// allocation of all IR objects. It is cleared at the begining/end of every
// *translateion*. This means an IRSB is only valid until the next call to
// `front_end` or `translate`. We use a Mutex to ensure that these methods are not
// called while an IRSB is still active.
struct LiftLock(ReentrantMutex<RefCell<bool>>);

impl LiftLock {
    fn new() -> Self {
        Self(ReentrantMutex::new(RefCell::new(false)))
    }

    fn lock(&self) -> LiftGuard {
        let guard = self.0.lock();
        assert!(!*guard.borrow(), "Already lifting");
        *guard.borrow_mut() = true;
        LiftGuard(guard)
    }
}

struct LiftGuard<'a>(ReentrantMutexGuard<'a, RefCell<bool>>);

impl Drop for LiftGuard<'_> {
    fn drop(&mut self) {
        *self.0.borrow_mut() = false;
    }
}

lazy_static! {
    static ref LIFT_LOCK: LiftLock = LiftLock::new();
}

#[cfg(test)]
mod test {
    use super::{Arch, VexEndness, TranslateArgs};

    #[test]
    fn sanity() {
        let mut vta = TranslateArgs::new(
            Arch::VexArchAMD64,
            Arch::VexArchAMD64,
            VexEndness::VexEndnessLE,
        );

        let irsb = vta.front_end(sanity as *const _, sanity as _).unwrap();

        println!("{}", irsb);

        for mut stmt in irsb.stmts() {
            if let super::ir::StmtEnum::Put(put) = stmt.as_enum() {
                println!("Got put with data: {}", put.data());
            }
        }
    }

    #[test]
    #[should_panic]
    fn double_lift() {
        let mut vta = TranslateArgs::new(
            Arch::VexArchAMD64,
            Arch::VexArchAMD64,
            VexEndness::VexEndnessLE,
        );

        let irsb = vta.front_end(sanity as *const _, sanity as _).unwrap();

        // get another irsb
        let next = match irsb.next().as_enum() {
            super::ir::ExprEnum::Const(c) => match c.as_enum() {
                super::ir::ConstEnum::U64(addr) => addr,
                _ => panic!(),
            }
            _ => panic!(),
        };

        let mut vta2 = TranslateArgs::new(
            Arch::VexArchAMD64,
            Arch::VexArchAMD64,
            VexEndness::VexEndnessLE,
        );
        let _irsb2 = vta2.front_end(next as *const _, next as _).unwrap();
    }

    #[test]
    fn translate() {
        let mut vta = TranslateArgs::new(
            Arch::VexArchAMD64,
            Arch::VexArchAMD64,
            VexEndness::VexEndnessLE,
        );

        let mut buf = [0; 1000];

        let size = vta.translate(
            translate as *const _,
            translate as _,
            &mut buf,
        ).unwrap();

        assert!(size > 300);
    }
}

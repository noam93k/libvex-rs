pub use vex_sys::{
    OFFSET_x86_EAX,
    OFFSET_x86_EBX,
    OFFSET_x86_ECX,
    OFFSET_x86_EDX,
    OFFSET_x86_ESI,
    OFFSET_x86_EDI,
    OFFSET_x86_EBP,
    OFFSET_x86_ESP,
    OFFSET_x86_EIP,
    OFFSET_x86_CS,
    OFFSET_x86_DS,
    OFFSET_x86_ES,
    OFFSET_x86_FS,
    OFFSET_x86_GS,
    OFFSET_x86_SS,

    VEX_HWCAPS_X86_MMXEXT,
    VEX_HWCAPS_X86_SSE1,
    VEX_HWCAPS_X86_SSE2,
    VEX_HWCAPS_X86_SSE3,
    VEX_HWCAPS_X86_LZCNT,
};

pub struct State(pub vex_sys::VexGuestX86State);

impl Default for State {
    fn default() -> Self {
        let mut this = std::mem::MaybeUninit::uninit();
        Self(unsafe {
            vex_sys::LibVEX_GuestX86_initialise(this.as_mut_ptr());
            this.assume_init()
        })
    }
}

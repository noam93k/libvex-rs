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

pub mod offset {
    use vex_sys::Int;

    pub const EAX: Int = vex_sys::OFFSET_x86_EAX as Int;
    pub const EBX: Int = vex_sys::OFFSET_x86_EBX as Int;
    pub const ECX: Int = vex_sys::OFFSET_x86_ECX as Int;
    pub const EDX: Int = vex_sys::OFFSET_x86_EDX as Int;
    pub const ESI: Int = vex_sys::OFFSET_x86_ESI as Int;
    pub const EDI: Int = vex_sys::OFFSET_x86_EDI as Int;
    pub const EBP: Int = vex_sys::OFFSET_x86_EBP as Int;
    pub const ESP: Int = vex_sys::OFFSET_x86_ESP as Int;
    pub const EIP: Int = vex_sys::OFFSET_x86_EIP as Int;
    pub const CS: Int = vex_sys::OFFSET_x86_CS as Int;
    pub const DS: Int = vex_sys::OFFSET_x86_DS as Int;
    pub const ES: Int = vex_sys::OFFSET_x86_ES as Int;
    pub const FS: Int = vex_sys::OFFSET_x86_FS as Int;
    pub const GS: Int = vex_sys::OFFSET_x86_GS as Int;
    pub const SS: Int = vex_sys::OFFSET_x86_SS as Int;
}

pub mod hw_caps {
    pub use vex_sys::{
        VEX_HWCAPS_X86_MMXEXT as MMXEXT,
        VEX_HWCAPS_X86_SSE1 as SSE1,
        VEX_HWCAPS_X86_SSE2 as SSE2,
        VEX_HWCAPS_X86_SSE3 as SSE3,
        VEX_HWCAPS_X86_LZCNT as LZCNT,
    };
}

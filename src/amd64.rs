
pub use vex_sys::{
    OFFSET_amd64_RAX,
    OFFSET_amd64_RBX,
    OFFSET_amd64_RCX,
    OFFSET_amd64_RDX,
    OFFSET_amd64_RSI,
    OFFSET_amd64_RDI,
    OFFSET_amd64_RSP,
    OFFSET_amd64_RBP,
    OFFSET_amd64_R8,
    OFFSET_amd64_R9,
    OFFSET_amd64_R10,
    OFFSET_amd64_R11,
    OFFSET_amd64_R12,
    OFFSET_amd64_R13,
    OFFSET_amd64_R14,
    OFFSET_amd64_R15,
    OFFSET_amd64_RIP,

    VEX_HWCAPS_AMD64_SSE3,
    VEX_HWCAPS_AMD64_SSSE3,
    VEX_HWCAPS_AMD64_CX16,
    VEX_HWCAPS_AMD64_LZCNT,
    VEX_HWCAPS_AMD64_AVX,
    VEX_HWCAPS_AMD64_RDTSCP,
    VEX_HWCAPS_AMD64_BMI,
    VEX_HWCAPS_AMD64_AVX2,
    VEX_HWCAPS_AMD64_RDRAND,
    VEX_HWCAPS_AMD64_F16C,
    VEX_HWCAPS_AMD64_RDSEED,
};
pub struct State(pub vex_sys::VexGuestAMD64State);

impl Default for State {
    fn default() -> Self {
        let mut this = std::mem::MaybeUninit::uninit();
        Self(unsafe {
            vex_sys::LibVEX_GuestAMD64_initialise(this.as_mut_ptr());
            this.assume_init()
        })
    }
}

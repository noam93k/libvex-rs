
pub use vex_sys::{
    OFFSET_arm64_X0,
    OFFSET_arm64_X1,
    OFFSET_arm64_X2,
    OFFSET_arm64_X3,
    OFFSET_arm64_X4,
    OFFSET_arm64_X5,
    OFFSET_arm64_X6,
    OFFSET_arm64_X7,
    OFFSET_arm64_X8,
    OFFSET_arm64_XSP,
    OFFSET_arm64_PC,

    VEX_HWCAPS_ARM64_FHM,
    VEX_HWCAPS_ARM64_DPBCVAP,
    VEX_HWCAPS_ARM64_DPBCVADP,
    VEX_HWCAPS_ARM64_SM3,
    VEX_HWCAPS_ARM64_SM4,
    VEX_HWCAPS_ARM64_SHA3,
    VEX_HWCAPS_ARM64_RDM,
    VEX_HWCAPS_ARM64_ATOMICS,
    VEX_HWCAPS_ARM64_I8MM,
    VEX_HWCAPS_ARM64_BF16,
    VEX_HWCAPS_ARM64_FP16,
    VEX_HWCAPS_ARM64_VFP16,
};

pub struct State(pub vex_sys::VexGuestARM64State);

impl Default for State {
    fn default() -> Self {
        let mut this = std::mem::MaybeUninit::uninit();
        Self(unsafe {
            vex_sys::LibVEX_GuestARM64_initialise(this.as_mut_ptr());
            this.assume_init()
        })
    }
}

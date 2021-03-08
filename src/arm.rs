
pub use vex_sys::{
    OFFSET_arm_R0,
    OFFSET_arm_R1,
    OFFSET_arm_R2,
    OFFSET_arm_R3,
    OFFSET_arm_R4,
    OFFSET_arm_R5,
    OFFSET_arm_R7,
    OFFSET_arm_R13,
    OFFSET_arm_R14,
    OFFSET_arm_R15T,

    VEX_HWCAPS_ARM_VFP,
    VEX_HWCAPS_ARM_VFP2,
    VEX_HWCAPS_ARM_VFP3,
    VEX_HWCAPS_ARM_NEON,
};

pub struct State(pub vex_sys::VexGuestARMState);

impl Default for State {
    fn default() -> Self {
        let mut this = std::mem::MaybeUninit::uninit();
        Self(unsafe {
            vex_sys::LibVEX_GuestARM_initialise(this.as_mut_ptr());
            this.assume_init()
        })
    }
}


pub use vex_sys::{
    OFFSET_ppc64_GPR0,
    OFFSET_ppc64_GPR1,
    OFFSET_ppc64_GPR2,
    OFFSET_ppc64_GPR3,
    OFFSET_ppc64_GPR4,
    OFFSET_ppc64_GPR5,
    OFFSET_ppc64_GPR6,
    OFFSET_ppc64_GPR7,
    OFFSET_ppc64_GPR8,
    OFFSET_ppc64_GPR9,
    OFFSET_ppc64_GPR10,
    OFFSET_ppc64_CIA,
    OFFSET_ppc64_CR0_0,

    VEX_HWCAPS_PPC64_V,
    VEX_HWCAPS_PPC64_FX,
    VEX_HWCAPS_PPC64_GX,
    VEX_HWCAPS_PPC64_VX,
    VEX_HWCAPS_PPC64_DFP,
    VEX_HWCAPS_PPC64_ISA2_07,
    VEX_HWCAPS_PPC64_ISA3_0,
    VEX_HWCAPS_PPC64_ISA3_1,
};

pub struct State(pub vex_sys::VexGuestPPC64State);

impl Default for State {
    fn default() -> Self {
        let mut this = std::mem::MaybeUninit::uninit();
        Self(unsafe {
            vex_sys::LibVEX_GuestPPC64_initialise(this.as_mut_ptr());
            this.assume_init()
        })
    }
}

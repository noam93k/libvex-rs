
pub use vex_sys::{
    OFFSET_ppc32_GPR0,
    OFFSET_ppc32_GPR1,
    OFFSET_ppc32_GPR2,
    OFFSET_ppc32_GPR3,
    OFFSET_ppc32_GPR4,
    OFFSET_ppc32_GPR5,
    OFFSET_ppc32_GPR6,
    OFFSET_ppc32_GPR7,
    OFFSET_ppc32_GPR8,
    OFFSET_ppc32_GPR9,
    OFFSET_ppc32_GPR10,
    OFFSET_ppc32_CIA,
    OFFSET_ppc32_CR0_0,

    VEX_HWCAPS_PPC32_F,
    VEX_HWCAPS_PPC32_V,
    VEX_HWCAPS_PPC32_FX,
    VEX_HWCAPS_PPC32_GX,
    VEX_HWCAPS_PPC32_VX,
    VEX_HWCAPS_PPC32_DFP,
    VEX_HWCAPS_PPC32_ISA2_07,
    VEX_HWCAPS_PPC32_ISA3_0,
    VEX_HWCAPS_PPC32_ISA3_1,
};

pub struct State(pub vex_sys::VexGuestPPC32State);

impl Default for State {
    fn default() -> Self {
        let mut this = std::mem::MaybeUninit::uninit();
        Self(unsafe {
            vex_sys::LibVEX_GuestPPC32_initialise(this.as_mut_ptr());
            this.assume_init()
        })
    }
}

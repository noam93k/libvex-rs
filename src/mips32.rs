
pub use vex_sys::{
    OFFSET_mips32_r0,
    OFFSET_mips32_r1,
    OFFSET_mips32_r2,
    OFFSET_mips32_r3,
    OFFSET_mips32_r4,
    OFFSET_mips32_r5,
    OFFSET_mips32_r6,
    OFFSET_mips32_r7,
    OFFSET_mips32_r8,
    OFFSET_mips32_r9,
    OFFSET_mips32_r10,
    OFFSET_mips32_r11,
    OFFSET_mips32_r12,
    OFFSET_mips32_r13,
    OFFSET_mips32_r14,
    OFFSET_mips32_r15,
    OFFSET_mips32_r17,
    OFFSET_mips32_r18,
    OFFSET_mips32_r19,
    OFFSET_mips32_r20,
    OFFSET_mips32_r21,
    OFFSET_mips32_r22,
    OFFSET_mips32_r23,
    OFFSET_mips32_r24,
    OFFSET_mips32_r25,
    OFFSET_mips32_r26,
    OFFSET_mips32_r27,
    OFFSET_mips32_r28,
    OFFSET_mips32_r29,
    OFFSET_mips32_r30,
    OFFSET_mips32_r31,
    OFFSET_mips32_PC,
    OFFSET_mips32_HI,
    OFFSET_mips32_LO,
};

pub struct State(pub vex_sys::VexGuestMIPS32State);

impl Default for State {
    fn default() -> Self {
        let mut this = std::mem::MaybeUninit::uninit();
        Self(unsafe {
            vex_sys::LibVEX_GuestMIPS32_initialise(this.as_mut_ptr());
            this.assume_init()
        })
    }
}

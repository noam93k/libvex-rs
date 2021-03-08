
pub use vex_sys::{
    OFFSET_mips64_r0,
    OFFSET_mips64_r1,
    OFFSET_mips64_r2,
    OFFSET_mips64_r3,
    OFFSET_mips64_r4,
    OFFSET_mips64_r5,
    OFFSET_mips64_r6,
    OFFSET_mips64_r7,
    OFFSET_mips64_r8,
    OFFSET_mips64_r9,
    OFFSET_mips64_r10,
    OFFSET_mips64_r11,
    OFFSET_mips64_r12,
    OFFSET_mips64_r13,
    OFFSET_mips64_r14,
    OFFSET_mips64_r15,
    OFFSET_mips64_r17,
    OFFSET_mips64_r18,
    OFFSET_mips64_r19,
    OFFSET_mips64_r20,
    OFFSET_mips64_r21,
    OFFSET_mips64_r22,
    OFFSET_mips64_r23,
    OFFSET_mips64_r24,
    OFFSET_mips64_r25,
    OFFSET_mips64_r26,
    OFFSET_mips64_r27,
    OFFSET_mips64_r28,
    OFFSET_mips64_r29,
    OFFSET_mips64_r30,
    OFFSET_mips64_r31,
    OFFSET_mips64_PC,
    OFFSET_mips64_HI,
    OFFSET_mips64_LO,
};

pub struct State(pub vex_sys::VexGuestMIPS64State);

impl Default for State {
    fn default() -> Self {
        let mut this = std::mem::MaybeUninit::uninit();
        Self(unsafe {
            vex_sys::LibVEX_GuestMIPS64_initialise(this.as_mut_ptr());
            this.assume_init()
        })
    }
}

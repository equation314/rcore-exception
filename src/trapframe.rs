use core::fmt::{Debug, Error, Formatter};
use riscv::register::{scause::Scause, sstatus::Sstatus};

#[repr(C)]
#[derive(Clone)]
pub struct TrapFrame {
    pub x: [usize; 32],   // General registers
    pub sstatus: Sstatus, // Supervisor Status Register
    pub sepc: usize,      // Supervisor exception program counter
    pub stval: usize,     // Supervisor trap value
    pub scause: Scause,   // Scause register: record the cause of exception/interrupt/trap
}

impl Debug for TrapFrame {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        struct Regs<'a>(&'a [usize; 32]);
        impl<'a> Debug for Regs<'a> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                const REG_NAME: [&str; 32] = [
                    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2",
                    "a3", "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9",
                    "s10", "s11", "t3", "t4", "t5", "t6",
                ];
                f.debug_map().entries(REG_NAME.iter().zip(self.0)).finish()
            }
        }
        f.debug_struct("TrapFrame")
            .field("regs", &Regs(&self.x))
            .field("sstatus", &self.sstatus)
            .field("sepc", &self.sepc)
            .field("stval", &self.stval)
            .field("scause", &self.scause.cause())
            .finish()
    }
}

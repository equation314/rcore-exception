use crate::TrapFrame;

pub trait Handler {
    fn debug(_tf: &TrapFrame) {}

    fn handle_timer() {}

    fn handle_external() {}

    fn handle_syscall(_tf: &mut TrapFrame) {}

    fn handle_breakpoint(_tf: &mut TrapFrame) {}

    fn handle_page_fault(_tf: &mut TrapFrame) {}

    fn handle_other(_tf: &mut TrapFrame) {
        panic!("unexpected trap");
    }

    extern "C" fn handle(tf: &mut TrapFrame) {
        use riscv::register::scause::{Exception as E, Interrupt as I, Trap};
        Self::debug(tf);
        match tf.scause.cause() {
            Trap::Interrupt(I::SupervisorExternal) => Self::handle_external(),
            Trap::Interrupt(I::SupervisorTimer) => Self::handle_timer(),
            Trap::Exception(E::Breakpoint) => Self::handle_breakpoint(tf),
            Trap::Exception(E::UserEnvCall) => Self::handle_syscall(tf),
            Trap::Exception(E::LoadPageFault) => Self::handle_page_fault(tf),
            Trap::Exception(E::StorePageFault) => Self::handle_page_fault(tf),
            Trap::Exception(E::InstructionPageFault) => Self::handle_page_fault(tf),
            _ => Self::handle_other(tf),
        }
    }
}

pub struct DefaultHandler;

impl Handler for DefaultHandler {}

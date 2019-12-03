use crate::TrapFrame;

pub trait RvHandler {
    fn debug(_tf: &TrapFrame) {}

    fn handle_timer() {}

    fn handle_external() {}

    fn handle_syscall(_tf: &mut TrapFrame) {}

    fn handle_breakpoint(_tf: &mut TrapFrame) {}

    fn handle_page_fault(_tf: &mut TrapFrame) {}

    fn handle_other(_tf: &mut TrapFrame) {
        panic!("unexpected trap");
    }
}

pub struct DefaultRvHandler;

impl RvHandler for DefaultRvHandler {}

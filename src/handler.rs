use crate::TrapFrame;

pub trait RvHandler {
    fn handle_timer() {}

    fn handle_external() {}

    fn handle_syscall(_tf: &TrapFrame) {}

    fn handle_breakpoint(_tf: &TrapFrame) {}

    fn handle_page_fault(_tf: &TrapFrame) {}

    fn handle_other(_tf: &TrapFrame) {
        panic!("unexpected trap");
    }
}

pub struct DefaultRvHandler;

impl RvHandler for DefaultRvHandler {}

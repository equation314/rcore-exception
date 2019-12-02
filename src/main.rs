use rcore_exception::{RvHandler, TrapFrame};

struct MyHandler;

impl RvHandler for MyHandler {
    fn handle_timer() {
        println!("handle_super_timer");
    }

    fn handle_external() {
        println!("handle_super_external");
    }

    fn handle_syscall(_tf: &TrapFrame) {
        println!("handle_syscall");
    }

    fn handle_page_fault(_tf: &TrapFrame) {
        println!("handle_page_fault");
    }

    fn handle_other(_tf: &TrapFrame) {
        println!("handle_other");
    }
}

pub fn main() {
    rcore_exception::init::<MyHandler>();
    rcore_exception::trap();
}

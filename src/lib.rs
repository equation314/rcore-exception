#![feature(global_asm)]

pub mod handler;
pub mod trapframe;

pub use handler::{DefaultRvHandler, RvHandler};
pub use trapframe::TrapFrame;

global_asm!(include_str!("trap.S"));

type RustTrapFn = extern "C" fn(tf: &TrapFrame);

#[no_mangle]
static mut RUST_TRAP_FN: RustTrapFn = rust_trap::<DefaultRvHandler>;
static mut VEC: usize = 0;

extern "C" fn rust_trap<H: RvHandler>(tf: &TrapFrame) {
    println!("rust_trap");
    H::handle_timer();
    H::handle_external();
    H::handle_syscall(tf);
    H::handle_breakpoint(tf);
    H::handle_page_fault(tf);
}

pub fn init<H: RvHandler>() {
    extern "C" {
        fn alltraps();
    }
    unsafe {
        VEC = alltraps as usize;
        RUST_TRAP_FN = rust_trap::<H>;
    }
}

pub fn trap() {
    use core::mem::transmute;
    let f: fn() = unsafe { transmute(VEC) };
    f();
}

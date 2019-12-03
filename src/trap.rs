use crate::{DefaultRvHandler, RvHandler, TrapFrame};
use riscv::register::{sie, sscratch, stvec};

#[cfg(target_arch = "riscv32")]
global_asm!(
    r"
    .equ XLENB,     4
    .equ XLENb,     32
    .macro LOAD_TF a1, a2
        lw \a1, \a2*XLENB(sp)
    .endm
    .macro STORE_TF a1, a2
        sw \a1, \a2*XLENB(sp)
    .endm
    .macro LOAD a1, a2
        lw \a1, \a2
    .endm
    .macro STORE a1, a2
        sw \a1, \a2
    .endm
"
);
#[cfg(target_arch = "riscv64")]
global_asm!(
    r"
    .equ XLENB,     8
    .equ XLENb,     64
    .macro LOAD_TF a1, a2
        ld \a1, \a2*XLENB(sp)
    .endm
    .macro STORE_TF a1, a2
        sd \a1, \a2*XLENB(sp)
    .endm
    .macro LOAD a1, a2
        ld \a1, \a2
    .endm
    .macro STORE a1, a2
        sd \a1, \a2
    .endm
"
);

global_asm!(include_str!("trap.S"));

type TrapHandlerFn = extern "C" fn(tf: &mut TrapFrame);

#[no_mangle]
static mut TRAP_HANDLER_FN: TrapHandlerFn = trap_handler::<DefaultRvHandler>;

pub fn init<H: RvHandler>() {
    unsafe {
        // Set sscratch register to 0, indicating to exception vector that we are
        // presently executing in the kernel
        sscratch::write(0);
        // Set the exception vector address
        stvec::write(trap_entry as usize, stvec::TrapMode::Direct);
        // Enable external interrupt
        sie::set_sext();
        // Set the actual function address called from asm according to the handler
        TRAP_HANDLER_FN = trap_handler::<H>;
    }
}

extern "C" fn trap_handler<H: RvHandler>(tf: &mut TrapFrame) {
    use riscv::register::scause::{Exception as E, Interrupt as I, Trap};
    H::debug(tf);
    match tf.scause.cause() {
        Trap::Interrupt(I::SupervisorExternal) => H::handle_external(),
        Trap::Interrupt(I::SupervisorTimer) => H::handle_timer(),
        Trap::Exception(E::Breakpoint) => H::handle_breakpoint(tf),
        Trap::Exception(E::UserEnvCall) => H::handle_syscall(tf),
        Trap::Exception(E::LoadPageFault) => H::handle_page_fault(tf),
        Trap::Exception(E::StorePageFault) => H::handle_page_fault(tf),
        Trap::Exception(E::InstructionPageFault) => H::handle_page_fault(tf),
        _ => H::handle_other(tf),
    }
}

extern "C" {
    fn trap_entry();
}

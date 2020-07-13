use core::ptr::write_volatile;
use alloc::collections::VecDeque;

pub struct Processes {
    process: VecDeque<Process>
}

impl Processes {
    pub fn with_capacity(capacity: usize) -> Self {
        let v: VecDeque<Process> = VecDeque::with_capacity(capacity);
        Self {
            process: v
        }
    }

    pub fn push_front(&mut self, p: Process) {
        self.process.push_front(p);
    }

    pub fn push_back(&mut self, p: Process) {
        self.process.push_back(p);
    }

    pub fn pop_front(&mut self) -> Process {
        self.process.pop_front().unwrap()
    }

    pub fn pop_back(&mut self) -> Process {
        self.process.pop_back().unwrap()
    }

    pub fn run(&mut self) -> ! {
        loop {
            let mut p = self.pop_front();
            p.run();
            self.push_back(p);
        }
    }
}

pub struct Process {
    stack_ptr: *mut usize,
    // R4 - R11
    states: [usize; 8],
}

impl Process {
    /// Initialize stack frame of task
    pub unsafe fn new(stack_ptr: *mut usize, callback: fn() -> !) -> Self {
        Self {
            stack_ptr: push_function_call(stack_ptr, callback),
            states: [0; 8],
        }
    }

    /// Switch context from kernel to task
    pub fn run(&mut self) {
        unsafe { self.stack_ptr = switch(self.stack_ptr, &mut self.states) }
    }
}

/// Set initial register of the context of task
///
/// The processor will automatically load the top 8 words(u32)
/// from the stack frame of task into register when switching to context.
pub unsafe fn push_function_call(user_stack: *mut usize, callback: fn() -> !) -> *mut usize {
    write_volatile(user_stack.offset(0), 0); // R0
    write_volatile(user_stack.offset(1), 0); // R1
    write_volatile(user_stack.offset(2), 0); // R2
    write_volatile(user_stack.offset(3), 0); // R3
    write_volatile(user_stack.offset(5), 0 | 0x1); // LR
    write_volatile(user_stack.offset(6), callback as usize | 1); // PC
    write_volatile(user_stack.offset(7), 0x01000000); //xPSR
    user_stack
}

/// Toggle context between kernel and task
///
/// SVC interrupt can only be fired by instruction `svc`.
///
/// SVC handler is an interrupt handler, which means it will
/// be executed in handler mode, and because of that, it could
/// choose the execution context when it returns by loading special
/// EXC_RETURN value into pc register.
///
/// EXC_RETURN variants:
/// - 0xfffffff9 : return to msp (thread mode) - switch to kernel
/// - 0xfffffffd : return to psp (thread mode) - switch to task
/// - 0xfffffff1 : return to msp (handler mode) - return to another interrupt handler
///
/// `msp` means the Main Stack Pointer and
/// `psp` means the Process Stack Pointer.
#[no_mangle]
#[naked]
pub unsafe extern "C" fn svc_handler() {
    llvm_asm!("
    cmp lr, #0xfffffff9
    bne to_kernel

    mov r0, #1
    msr CONTROL, r0
    isb
    movw lr, #0xfffd
    movt lr, #0xffff
    bx lr

    to_kernel:
    mov r0, #0
    msr CONTROL, r0
    isb
    movw lr, #0xfff9
    movt lr, #0xffff
    bx lr"
    :::: "volatile" );
}

/// Switch context to kernel in fixed period
///
/// It's is important for the kernel to get the control back sometimes
/// so as to dispatch to other tasks.
#[no_mangle]
#[naked]
pub unsafe extern "C" fn systick_handler() {
    llvm_asm!("
    mov r0, #0
    msr CONTROL, r0
    isb
    movw lr, #0xfff9
    movt lr, #0xffff"
    :::: "volatile"  );
}

/// Setup task context and switch to it
///
/// This function is doing these few steps:
/// 1. Saves registers {r4-r12, lr} into msp (by compiler ABI).
/// 2. Load task stack address into psp.
/// 3. Restore the register states of task from `process_regs` into {r4-r11}.
/// 4. Invoke SVC exception in order to jump into svc_handler,
///    therefore we switched to task context.
/// 5. Saves registers states {r4-r11} into `process_regs`
///    when switched back to kernel (by systick_handler or svc_handler),
/// 6. Restore new psp into `user_stack`.
/// 7. Restore kernel registers states {r4-r12, lr->pc} from msp (by compiler ABI).
///
/// The first step and last step is performed by function call ABI convention,
/// so we have to ensure this function is never inlined.
#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn switch(
    mut user_stack: *mut usize,
    process_regs: &mut [usize; 8],
) -> *mut usize {
    llvm_asm!("
    msr psp, $0
    ldmia $2, {r4-r11}

    svc 0xff

    stmia $2, {r4-r11}
    mrs $0, psp
    "
    : "={r0}"(user_stack)
    : "{r0}"(user_stack), "{r1}"(process_regs as *mut _ as *mut _)
    : "r4","r5","r6","r8","r9","r10","r11" : "volatile" );

    user_stack
}

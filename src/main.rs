#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(naked_functions)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

mod rcc;
mod led;
mod usb_ttl;
mod tim;
mod switch_context;
mod esp;
mod interrupt;
mod sdcard;
mod handle;
mod config;
mod wifi;
mod systick;
mod startup;
mod debug;

extern crate alloc;

use stm32f4xx_hal::stm32;
use core::panic::PanicInfo;
use core::fmt::Write;
use core::alloc::Layout;
use alloc_cortex_m::CortexMHeap;
use crate::usb_ttl::USART1;
use crate::switch_context::{Process, Processes};

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const TASK_NUM: usize = 3;
const TASK_STACK_SIZE: usize = 200;
static mut TASK_STACKS: [[usize; TASK_STACK_SIZE]; TASK_NUM] = [[0; TASK_STACK_SIZE]; TASK_NUM];

#[no_mangle]
#[inline(never)]
fn main() -> ! {
    unsafe { ALLOCATOR.init(0x20000000, 1024) }
    unsafe { offset_interrupt(); }
    let mut ps = Processes::with_capacity(2);

    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = stm32::Peripherals::take().unwrap();

    rcc::clock_init(&mut dp.RCC, &mut dp.FLASH);
    systick::systick_start(&mut cp.SYST);
    usb_ttl::init(&mut dp.RCC, &mut dp.GPIOA, &mut dp.USART1);
    esp::serial_init(&mut dp.RCC, &mut dp.GPIOA, &mut dp.USART2);
    esp::dma_init(&mut dp.RCC, &mut dp.DMA1);
    tim::init(&mut dp.RCC, &mut dp.TIM2);
    led::init(&mut dp.RCC, &mut dp.GPIOF);
    sdcard::init(&mut dp.RCC, &mut dp.GPIOC, &mut dp.GPIOD);
    interrupt::nvic_enable();

    led::green_dark();
    led::red_dark();
    wifi::init();

    ps.run();
}

unsafe fn offset_interrupt() {
    let ptr = &*cortex_m::peripheral::SCB::ptr();
    ptr.vtor.write(0x20000);
}

#[panic_handler]
pub unsafe extern "C" fn panic_fmt(info: &PanicInfo) -> ! {
    writeln!(USART1, "{}, {}", info.message().unwrap(), info.location().unwrap()).unwrap();
    loop {}
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}
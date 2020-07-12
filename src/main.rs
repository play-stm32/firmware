#![feature(panic_info_message)]
#![no_std]
#![no_main]
#![feature(type_ascription)]
#![feature(llvm_asm)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]

mod rcc;
mod led;
mod usb_ttl;
mod tim;
mod switch_context;
mod esp;
mod sdcard;
mod handle;
mod config;
mod net;
mod systick;
mod startup;
mod debug;

use stm32f4xx_hal::stm32;
use core::panic::PanicInfo;
use core::fmt::Write;
use crate::usb_ttl::USART1;
use crate::switch_context::Process;

const TASK_NUM: usize = 2;
const TASK_STACK_SIZE: usize = 200;
static mut TASK_STACKS: [[usize; TASK_STACK_SIZE]; TASK_NUM] = [[0; TASK_STACK_SIZE]; TASK_NUM];

#[no_mangle]
#[inline(never)]
fn main() -> ! {
    offset_interrupt();

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

    led::green_dark();
    led::red_dark();
    // net::init();


    let mut t1 = unsafe {
        Process::new(TASK_STACKS[0].as_mut_ptr(), task1)
    };

    let mut t2 = unsafe {
        Process::new(TASK_STACKS[1].as_mut_ptr(), task2)
    };

    loop {
        t1.switch_to_task();
        t2.switch_to_task();
    }
}

#[no_mangle]
fn task1() -> ! {
    loop {
        writeln!(USART1, "task1").unwrap();
    }
}

#[no_mangle]
fn task2() -> ! {
    loop {
        writeln!(USART1, "task2").unwrap();
    }
}

fn offset_interrupt() {
    unsafe {
        let ptr = &*cortex_m::peripheral::SCB::ptr();
        ptr.vtor.write(0x20000);
    }
}

#[panic_handler]
pub unsafe extern "C" fn panic_fmt(info: &PanicInfo) -> ! {
    writeln!(USART1, "{}, {}", info.message().unwrap(), info.location().unwrap()).unwrap();
    loop {}
}
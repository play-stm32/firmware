#![feature(panic_info_message)]
#![no_std]
#![no_main]
#![feature(type_ascription)]

mod rcc;
mod led;
mod usb_ttl;
mod tim;
mod interrupt;
mod esp;
mod sdcard;
mod handle;
mod config;
mod net;

use stm32f4xx_hal::stm32;
use core::panic::PanicInfo;
use core::fmt::Write;
use crate::usb_ttl::USART1;

#[no_mangle]
#[inline(never)]
fn main() -> ! {
    offset_interrupt();

    let mut dp = stm32::Peripherals::take().unwrap();
    rcc::clock_init(&mut dp.RCC, &mut dp.FLASH);
    usb_ttl::init(&mut dp.RCC, &mut dp.GPIOA, &mut dp.USART1);
    esp::serial_init(&mut dp.RCC, &mut dp.GPIOA, &mut dp.USART2);
    esp::dma_init(&mut dp.RCC, &mut dp.DMA1);
    tim::init(&mut dp.RCC, &mut dp.TIM2);
    led::init(&mut dp.RCC, &mut dp.GPIOF);
    sdcard::init(&mut dp.RCC, &mut dp.GPIOC, &mut dp.GPIOD);
    interrupt::nvic_enable();
    led::green_dark();
    led::red_dark();
    net::init();

    loop {}
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
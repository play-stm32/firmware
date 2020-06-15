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

use stm32f4xx_hal::stm32;
use core::panic::PanicInfo;
use core::fmt::Write;
use esp8266::command::AT;
use crate::esp::{USART2, RX_STATE, MSG_LEN, BUFFER};
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

    let mut buf = [0; 512];
    let (ssid, passwd) = sdcard::get_wifi_config(&mut buf);

    let at = unsafe {
        AT::new(USART2,
                tim::delay,
                &RX_STATE,
                &BUFFER,
                &MSG_LEN,
                1)
    };

    at.wait_ready(1).unwrap().
        set_wifi_mode_no_save(1, 1).unwrap().
        connect_wifi(ssid, passwd, 10).unwrap().
        connect_tcp("10.10.10.184", 1122, 2).unwrap().
        send_msg_to_server("8b71ba1e-d6c2-46bc-9f34-6664bd3d9c19", 2).unwrap();

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
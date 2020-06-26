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

use stm32f4xx_hal::stm32;
use core::panic::PanicInfo;
use core::fmt::Write;
use esp8266::command::{AT, ATError};
use crate::esp::{USART2, RX_STATE, MSG_LEN, BUFFER};
use crate::usb_ttl::USART1;
use crate::config::{Config, CONFIG_BUF};

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

    let config = unsafe {
        Config::get_config(&mut CONFIG_BUF).unwrap()
    };

    let at = unsafe {
        AT::new(USART2,
                tim::delay,
                &RX_STATE,
                &BUFFER,
                &MSG_LEN,
                1)
    };

    let op = || -> Result<(), ATError> {
        Ok(at.wait_ready(1)?.
            set_wifi_mode_no_save(1, 1)?.
            connect_wifi(config.wifi_ssid, config.wifi_pwd, 10)?.
            connect_tcp(config.server, config.port, 2)?.
            send_msg_to_server(config.token, 2)?)
    };

    for i in 1..11 {
        writeln!(USART1, "try to init net service {} times", i).unwrap();
        if let Err(_) = op() {
            writeln!(USART1, "try {} times error", i).unwrap();
        } else {
            writeln!(USART1, "init net service successfully").unwrap();
            break;
        };
    }

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
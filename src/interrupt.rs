use cortex_m::peripheral::NVIC;
use stm32f4xx_hal::interrupt;
use stm32f4xx_hal::stm32;
use crate::{tim, esp, handle};
use crate::tim::{SECOND, SECOND_VALUE};
use crate::esp::{MSG_LEN, BUFFER, RX_STATE, BUFFER_LEN};
use core::sync::atomic::Ordering;

/// NVIC enable
pub fn nvic_enable() {
    unsafe {
        NVIC::unmask(stm32::interrupt::TIM2);
        NVIC::unmask(stm32::interrupt::USART2);
    }
}

/// NVIC disable
pub fn nvic_disable() {
    NVIC::mask(stm32::interrupt::TIM2);
    NVIC::mask(stm32::interrupt::USART2);
}

/// handle TIM2 interrupt
#[interrupt]
fn TIM2() {
    unsafe {
        tim::clean_interrupt_flag();
        SECOND += 1;
        if SECOND == SECOND_VALUE { tim::disable_count(); }
    }
}

#[interrupt]
fn USART2() {
    esp::usart_clear_idle();
    esp::dma_disble();
    let remain_size = esp::dma_get_size();
    let len = BUFFER_LEN - remain_size;
    unsafe {
        MSG_LEN = len;
        RX_STATE.store(true, Ordering::SeqCst);
        if let Ok(msg) = core::str::from_utf8(&BUFFER[2..6]) {
            if msg.contains("+IPD") {
                esp::usart_disable_idle();
                handle::handle_request();
                esp::usart_enable_idle();
                RX_STATE.store(false, Ordering::SeqCst);
            }
        }
    }
    esp::dma_resize();
    esp::dma_enable();
}

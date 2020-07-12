use core::sync::atomic::Ordering;
use crate::{tim, esp, handle};
use crate::tim::{SECOND, SECOND_VALUE};
use crate::esp::{MSG_LEN, BUFFER, RX_STATE, BUFFER_LEN};

/// handle TIM2 interrupt
pub unsafe extern "C" fn tim2_handler() {
    tim::clean_interrupt_flag();
    SECOND += 1;
    if SECOND == SECOND_VALUE { tim::disable_count(); }
}

/// handle USART2 interrupt
pub unsafe extern "C" fn usart2_handler() {
    esp::usart_clear_idle();
    esp::dma_disble();
    let remain_size = esp::dma_get_size();
    let len = BUFFER_LEN - remain_size;
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
    esp::dma_resize();
    esp::dma_enable();
}

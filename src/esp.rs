use stm32f4xx_hal::stm32;
use core::fmt::Error;
use core::str;
use core::sync::atomic::AtomicBool;
use embedded_hal::serial::Write as SerialWrite;
use core::fmt::Write;

const DMA_STREAM: usize = 5;
pub const BUFFER_LEN: usize = 1000;
pub static mut BUFFER: [u8; BUFFER_LEN] = [0; BUFFER_LEN];
pub static mut RX_STATE: AtomicBool = AtomicBool::new(false);
pub static mut MSG_LEN: usize = 0;

pub fn serial_init(rcc: &mut stm32::RCC, gpioa: &mut stm32::GPIOA, usart2: &mut stm32::USART2) {
    rcc.apb1enr.modify(|_r, w| w.usart2en().set_bit());
    rcc.ahb1enr.modify(|_r, w| w.gpioaen().set_bit());

    // PA2(Tx) alternate push
    gpioa.afrl.modify(|_r, w| w.afrl2().af7());
    gpioa.moder.modify(|_r, w| w.moder2().alternate());
    gpioa.ospeedr.modify(|_r, w| w.ospeedr2().high_speed());
    gpioa.pupdr.modify(|_r, w| w.pupdr2().pull_up());
    gpioa.otyper.modify(|_r, w| w.ot2().push_pull());

    // PA3(Rx) alternate push
    gpioa.afrl.modify(|_r, w| w.afrl3().af7());
    gpioa.moder.modify(|_r, w| w.moder3().alternate());
    gpioa.ospeedr.modify(|_r, w| w.ospeedr3().high_speed());
    gpioa.pupdr.modify(|_r, w| w.pupdr3().pull_up());
    gpioa.otyper.modify(|_r, w| w.ot3().push_pull());

    // configurate usart baudrate
    // USARTDIV = FCLK (PCLK1 for USART2) / baudrate / 16
    //          = 42M / 115200 / 16 = 22.75
    // DIV_MANTISSA = USARTDIV (integer part)
    //              = 22
    // DIV_FRACTION = USARTDIV (fraction part) * 16
    //              = 0.75 * 16 = 12
    usart2.brr.write(|w| w.div_mantissa().bits(22).div_fraction().bits(12));
    usart2.cr1.write(|w| w.ue().set_bit().te().set_bit().re().set_bit().idleie().set_bit());
    usart2.cr3.write(|w| w.dmar().set_bit());
}

pub fn send_msg_to_server(msg: &str) {
    let len = msg.len();
    writeln!(USART2, "AT+CIPSEND={}\r", len).unwrap();
    for _ in 0..50000 {}
    write!(USART2, "{}", msg).unwrap();
    usart_clear_idle();
}

pub fn usart_disable_idle() {
    let ptr = unsafe { &*stm32::USART2::ptr() };
    ptr.cr1.modify(|_r, w| w.idleie().clear_bit());
}

pub fn usart_enable_idle() {
    let ptr = unsafe { &*stm32::USART2::ptr() };
    ptr.cr1.modify(|_r, w| w.idleie().set_bit());
}

pub fn usart_clear_idle() {
    let ptr = unsafe { &*stm32::USART2::ptr() };
    ptr.sr.read().bits();
    ptr.dr.read().bits();
}

pub fn dma_init(rcc: &mut stm32::RCC, dma: &mut stm32::DMA1) {
    rcc.ahb1enr.modify(|_r, w| w.dma1en().set_bit());

    let usart2_dr_ptr = stm32::USART2::ptr() as u32 + 0x04;
    let buffer_ptr = unsafe {
        BUFFER.as_ptr() as u32
    };

    dma.st[DMA_STREAM].par.write(|w| w.pa().bits(usart2_dr_ptr));
    dma.st[DMA_STREAM].m0ar.write(|w| w.m0a().bits(buffer_ptr));
    dma.st[DMA_STREAM].ndtr.write(|w| w.ndt().bits(BUFFER_LEN as u16));

    dma.st[DMA_STREAM].cr.write(|w|
        w.chsel().bits(4)
            .pl().medium()
            .msize().bits8()
            .psize().bits8()
            .minc().set_bit()
            .pinc().clear_bit()
            .circ().set_bit()
            .dir().peripheral_to_memory()
            .en().set_bit());
}

pub fn dma_enable() {
    let ptr = unsafe {
        &*stm32::DMA1::ptr()
    };
    ptr.st[DMA_STREAM].cr.modify(|_r, w| w.en().set_bit());
}

pub fn dma_disble() {
    let ptr = unsafe {
        &*stm32::DMA1::ptr()
    };
    ptr.st[DMA_STREAM].cr.modify(|_r, w| w.en().clear_bit());
}

pub fn dma_resize() {
    let ptr = unsafe {
        &*stm32::DMA1::ptr()
    };
    ptr.st[DMA_STREAM].ndtr.modify(|_r, w| w.ndt().bits(BUFFER_LEN as u16));
}

pub fn dma_get_size() -> usize {
    let ptr = unsafe { &*stm32::DMA1::ptr() };
    ptr.st[DMA_STREAM].ndtr.read().ndt().bits() as usize
}

pub struct USART2;

#[derive(Debug)]
pub enum NeverError {}

impl embedded_hal::serial::Write<u8> for USART2 {
    type Error = NeverError;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        let ptr = unsafe { &*stm32::USART2::ptr() };
        self.flush().unwrap();
        unsafe {
            ptr.dr.write(|w| w.bits(word as u32));
        }
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        let ptr = unsafe { &*stm32::USART2::ptr() };
        while ptr.sr.read().txe().bit_is_clear() {}
        Ok(())
    }
}

impl core::fmt::Write for USART2 {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for ch in s.bytes() {
            self.write(ch).unwrap();
        }
        Ok(())
    }
}
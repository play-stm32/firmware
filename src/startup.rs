use crate::switch_context::{svc_handler, systick_handler};
use crate::debug::hard_fault_handler;
use crate::interrupt::{tim2_handler, usart2_handler};

extern "C" {
    static mut _sidata: u32;
    static mut _sdata: u32;
    static mut _edata: u32;
    static mut _sbss: u32;
    static mut _ebss: u32;
}

// interrupt vertor that will be linked to the very start of FLASH
#[link_section = ".vector"]
#[used]
pub static ISR_VECTOR: [unsafe extern "C" fn(); 96] = [
    reset_handler,
    unhandled_interrupt, // NMI
    hard_fault_handler,  // Hard Fault
    unhandled_interrupt, // MemManage
    unhandled_interrupt, // BusFault
    unhandled_interrupt, // UsageFault
    unhandled_interrupt,
    unhandled_interrupt,
    unhandled_interrupt,
    unhandled_interrupt,
    svc_handler,         // SVC
    unhandled_interrupt, // DebugMon
    unhandled_interrupt,
    unhandled_interrupt, // PendSV
    systick_handler,     // SysTick
    unimplemented_interrupt, // WWDG,
    unimplemented_interrupt, // PVD
    unimplemented_interrupt, // TAMP_STAMP
    unimplemented_interrupt, // RTC_WKUP
    unimplemented_interrupt, // FLASH
    unimplemented_interrupt, // RCC
    unimplemented_interrupt, // EXTI0
    unimplemented_interrupt, // EXTI1
    unimplemented_interrupt, // EXTI2
    unimplemented_interrupt, // EXTI3
    unimplemented_interrupt, // EXTI4
    unimplemented_interrupt, // DMA1_Stream0
    unimplemented_interrupt, // DMA1_Stream1
    unimplemented_interrupt, // DMA1_Stream2
    unimplemented_interrupt, // DMA1_Stream3
    unimplemented_interrupt, // DMA1_Stream4
    unimplemented_interrupt, // DMA1_Stream5
    unimplemented_interrupt, // DMA1_Stream6
    unimplemented_interrupt, // ADC
    unimplemented_interrupt, // CAN1_TX
    unimplemented_interrupt, // CAN1_RX0
    unimplemented_interrupt, // CAN1_RX1
    unimplemented_interrupt, // CAN1_SCE
    unimplemented_interrupt, // EXTI9_5
    unimplemented_interrupt, // TIM1_BRK_TIM9
    unimplemented_interrupt, // TIM1_UP_TIM10
    unimplemented_interrupt, // TIM1_TRG_COM_TIM1
    unimplemented_interrupt, // TIM1_CC
    tim2_handler, // TIM2
    unimplemented_interrupt, // TIM3
    unimplemented_interrupt, // TIM4
    unimplemented_interrupt, // I2C1_EV
    unimplemented_interrupt, // I2C1_ER
    unimplemented_interrupt, // I2C1_EV
    unimplemented_interrupt, // I2C1_ER
    unimplemented_interrupt, // SPI1
    unimplemented_interrupt, // SPI2
    unimplemented_interrupt, // USART1
    usart2_handler, // USART2
    unimplemented_interrupt, // USART3
    unimplemented_interrupt, // EXTI15_10
    unimplemented_interrupt, // RTC_Alarm
    unimplemented_interrupt, // OTG_FS_WKUP
    unimplemented_interrupt, // TIM8_BRK_TIM12
    unimplemented_interrupt, // TIM8_UP_TIM13
    unimplemented_interrupt, // TIM8_CC
    unimplemented_interrupt, // DMA1_Stream7
    unimplemented_interrupt, // FSMC
    unimplemented_interrupt, // SDIO
    unimplemented_interrupt, // TIM5
    unimplemented_interrupt, // SPI3
    unimplemented_interrupt, // UART4
    unimplemented_interrupt, // UART5
    unimplemented_interrupt, // TIM6_DAC
    unimplemented_interrupt, // TIM7
    unimplemented_interrupt, // DMA2_Stream0
    unimplemented_interrupt, // DMA2_Stream1
    unimplemented_interrupt, // DMA2_Stream2
    unimplemented_interrupt, // DMA2_Stream3
    unimplemented_interrupt, // DMA2_Stream4
    unimplemented_interrupt, // ETH
    unimplemented_interrupt, // ETH_WKUP
    unimplemented_interrupt, // CAN2_TX
    unimplemented_interrupt, // CAN2_RX0
    unimplemented_interrupt, // CAN2_RX1
    unimplemented_interrupt, // CAN2_SCE
    unimplemented_interrupt, // OTG_FS
    unimplemented_interrupt, // DMA2_Stream5
    unimplemented_interrupt, // DMA2_Stream6
    unimplemented_interrupt, // DMA2_Stream7
    unimplemented_interrupt, // USART6
    unimplemented_interrupt, // I2C3_EV
    unimplemented_interrupt, // I2C3_ER
    unimplemented_interrupt, // OTG_HS_EP1_OUT
    unimplemented_interrupt, // OTG_HS_EP1_IN
    unimplemented_interrupt, // OTG_HS_WKUP
    unimplemented_interrupt, // OTG_HS
    unimplemented_interrupt, // DCMI
    unimplemented_interrupt, // CRYP
    unimplemented_interrupt, // HASH_RNG
    unimplemented_interrupt, // FPU
];

/// Main entry
///
/// It's where the whole system starts
#[no_mangle]
pub unsafe extern "C" fn reset_handler() {
    init_data(&mut _sidata, &mut _sdata, &mut _edata);
    zero_bss(&mut _sbss, &mut _ebss);

    crate::main();
}

unsafe fn init_data(mut sidata: *const u32, mut sdata: *mut u32, edata: *mut u32) {
    while sdata < edata {
        sdata.write(sidata.read());
        sdata = sdata.offset(1);
        sidata = sidata.offset(1);
    }
}

unsafe fn zero_bss(mut sbss: *mut u32, ebss: *mut u32) {
    while sbss < ebss {
        sbss.write_volatile(0);
        sbss = sbss.offset(1);
    }
}

#[no_mangle]
unsafe extern "C" fn unhandled_interrupt() {
    loop {}
}

#[no_mangle]
unsafe extern "C" fn unimplemented_interrupt() {
    unimplemented!()
}

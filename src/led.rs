use stm32f4xx_hal::stm32;

pub fn init(rcc: &mut stm32::RCC, gpiof: &mut stm32::GPIOF) {
    // gpiof enable
    rcc.ahb1enr.modify(|_r, w| w.gpiofen().set_bit());

    // set output mode
    gpiof.moder.modify(|_r, w| w.moder9().output().moder10().output());

    // set push pull mode
    gpiof.otyper.modify(|_r, w| w.ot9().push_pull().ot10().push_pull());

    // set speed
    gpiof.ospeedr.modify(|_r, w| w.ospeedr9().high_speed().ospeedr10().high_speed());
}

pub fn red_light() {
    let ptr = unsafe {
        &*stm32::GPIOF::ptr()
    };
    ptr.bsrr.write(|w| w.br9().reset());
}

pub fn red_dark() {
    let ptr = unsafe {
        &*stm32::GPIOF::ptr()
    };
    ptr.bsrr.write(|w| w.bs9().set());
}

pub fn green_light() {
    let ptr = unsafe {
        &*stm32::GPIOF::ptr()
    };
    ptr.bsrr.write(|w| w.br10().reset());
}

pub fn green_dark() {
    let ptr = unsafe {
        &*stm32::GPIOF::ptr()
    };
    ptr.bsrr.write(|w| w.bs10().set());
}
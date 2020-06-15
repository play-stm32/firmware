use stm32f4xx_hal::stm32;
use fat32::base::Volume;
use sdio_sdhc::sdcard::Card;

pub fn init(
    rcc: &mut stm32::RCC,
    gpioc: &mut stm32::GPIOC,
    gpiod: &mut stm32::GPIOD,
) {
    // gpioc gpiod enable
    rcc.ahb1enr.modify(|_r, w| w.gpiocen().set_bit().gpioden().set_bit());

    gpioc.afrh.modify(|_r, w|
        w.afrh8().af12()
            .afrh9().af12()
            .afrh10().af12()
            .afrh11().af12()
            .afrh12().af12());
    gpiod.afrl.modify(|_r, w| w.afrl2().af12());

    gpioc.moder.modify(|_r, w|
        w.moder8().alternate()
            .moder9().alternate()
            .moder10().alternate()
            .moder11().alternate()
            .moder12().alternate());
    gpiod.moder.modify(|_r, w| w.moder2().alternate());

    gpioc.ospeedr.modify(|_r, w|
        w.ospeedr8().high_speed()
            .ospeedr9().high_speed()
            .ospeedr10().high_speed()
            .ospeedr11().high_speed()
            .ospeedr12().high_speed());
    gpiod.ospeedr.modify(|_r, w| w.ospeedr2().high_speed());

    gpioc.otyper.modify(|_r, w|
        w.ot8().push_pull()
            .ot9().push_pull()
            .ot10().push_pull()
            .ot11().push_pull()
            .ot12().push_pull());
    gpiod.otyper.modify(|_r, w| w.ot2().push_pull());

    gpioc.pupdr.modify(|_r, w|
        w.pupdr8().pull_up()
            .pupdr9().pull_up()
            .pupdr10().pull_up()
            .pupdr11().pull_up()
            .pupdr12().pull_up());
    gpiod.pupdr.modify(|_r, w| w.pupdr2().pull_up());
}

pub fn get_wifi_config(buf: &mut [u8]) -> (&str, &str) {
    let card = Card::init().unwrap();
    let volume = Volume::new(card);
    let len =  volume.root_dir().
        load_file("wificonfig").unwrap().
        read(buf).unwrap();
    match core::str::from_utf8(&buf[0..len]) {
        Ok(str) => {
            let index = str.find(',').unwrap();
            (&str[0..index], &str[index + 1..])
        }
        Err(_) => {
            ("", "")
        }
    }
}
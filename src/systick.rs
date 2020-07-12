use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

pub fn systick_start(syst: &mut SYST) {
    // set clock = SYSCLK / 8
    syst.set_clock_source(SystClkSource::External);

    // rvr = SYSCLK(168M) / 8 / TICK_FREQUENCY(100Hz) = 210K
    syst.set_reload(210000);

    // start systick
    syst.clear_current();
    syst.enable_interrupt();
    syst.enable_counter();
}
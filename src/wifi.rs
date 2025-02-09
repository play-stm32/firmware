use esp8266::command::{AT, ATError};
use core::fmt::Write;

use crate::config::{Config, CONFIG_BUF};
use crate::esp::{USART2, RX_STATE, BUFFER, MSG_LEN};
use crate::usb_ttl::USART1;
use crate::tim;

pub fn init() {
    let at = unsafe {
        AT::new(USART2,
                tim::delay,
                &RX_STATE,
                &BUFFER,
                &MSG_LEN,
                1)
    };

    match unsafe { Config::get_config(&mut CONFIG_BUF) } {
        Ok(config) => {
            let op = || -> Result<(), ATError> {
                Ok(at.wait_ready(1)?.
                    set_wifi_mode_no_save(1, 1)?.
                    connect_wifi(config.wifi_ssid, config.wifi_pwd, 10)?.
                    connect_tcp(config.server, config.port, 2)?.
                    send_msg_to_server(config.token, 2)?)
            };

            if let Err(e) = op() {
                writeln!(USART1, "{:?}, please check config", e).unwrap();
            }
        }
        Err(e) => {
            writeln!(USART1, "{:?}, please insert sdcard", e).unwrap();
        }
    }
}

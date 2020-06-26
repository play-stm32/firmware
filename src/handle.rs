use core::str::FromStr;
use protocol::protocol::Command;
use crate::led;
use crate::esp::{BUFFER, send_msg_to_server};

pub fn handle_request() {
    unsafe {
        let mut index = 0;
        for i in 8.. {
            if BUFFER[i] == ':' as u8 {
                index = i + 1;
                break;
            }
        }

        let len = core::str::from_utf8(&BUFFER[7..index - 1]).unwrap();
        let mut len = usize::from_str(len).unwrap();
        if BUFFER[index + len - 1].is_ascii_control() { len -= 1; }

        if let Ok(command) = serde_json_core::from_slice::<Command>(&BUFFER[index..index + len]) {
            match command {
                Command::GreenLEDLight => { led::green_light(); }
                Command::GreenLEDDark => { led::green_dark(); }
                Command::RedLEDLight => { led::red_light(); }
                Command::RedLEDDark => { led::red_dark(); }
                Command::Reboot => {}
                Command::Upgrade => {}
            }
            send_msg_to_server("OK");
        }
    }
}

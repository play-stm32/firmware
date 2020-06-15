use core::str::FromStr;
use protocol::protocol::{Board, Request};
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

        if let Ok(request) = serde_json_core::from_slice::<Request>(&BUFFER[index..index + len]) {
            match request.board {
                Board::GreenLEDLight => { led::green_light(); }
                Board::GreenLEDDark => { led::green_dark(); }
                Board::RedLEDLight => { led::red_light(); }
                Board::RedLEDDark => { led::red_dark(); }
                _ => {}
            }
            send_msg_to_server("OK");
        }
    }
}

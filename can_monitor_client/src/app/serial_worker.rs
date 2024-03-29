use std::io::{self};
use std::thread;
use std::time::Duration;

use std::sync::mpsc::Sender;

use crate::models::CanLine;

pub fn read_serial(port_name: String, baud_rate: u32, writer: Sender<CanLine>, id_to_monitor: u32) {
    let port = serialport::new(&port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open();

    match port {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 1000];
            let mut last: &str = "";
            let mut value_string: String;
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => match std::str::from_utf8(&serial_buf[..t]) {
                        Ok(val) => {
                            value_string = format!("{}{}", last, val);
                            let lines = value_string.split("\r\n").collect::<Vec<&str>>();
                            last = lines.last().unwrap();
                            for line in lines {
                                if &last != &line {
                                    let canline = CanLine::new(line);
                                    if canline.id == id_to_monitor {
                                        writer.send(canline).unwrap();
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }

                let ten_millis = Duration::from_millis(10);
                thread::sleep(ten_millis);
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}

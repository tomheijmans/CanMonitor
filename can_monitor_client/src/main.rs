mod app;
mod models;

use clap::{App, AppSettings, Arg};
use crossterm::event::{self, Event as CEvent, KeyCode};
use std::{io, sync::mpsc, thread, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    let matches = App::new("Can monitor client")
        .about("Reads can data from the serial port and displays it")
        .setting(AppSettings::DisableVersion)
        .arg(
            Arg::with_name("port")
                .help("The device path to a serial port")
                .use_delimiter(false)
                .required(true),
        )
        .arg(
            Arg::with_name("byteIndexStart")
                .help("The start index of bytes to get the combined value from")
                .use_delimiter(false)
                .default_value("5")
                .validator(is_u8),
        )
        .arg(
            Arg::with_name("byteIndexEnd")
                .help("The end index of bytes to get the combined value from")
                .use_delimiter(false)
                .default_value("6")
                .validator(is_u8),
        )
        .arg(
            Arg::with_name("canIdToMonitor")
                .help("The id of the can row to monitor")
                .use_delimiter(false)
                .default_value("128")
                .validator(is_u32),
        )
        .get_matches();
    let port_name = matches.value_of("port").unwrap().clone().to_string();
    let byte_index_start = matches
        .value_of("byteIndexStart")
        .unwrap()
        .parse::<u8>()
        .unwrap();
    let byte_index_end = matches
        .value_of("byteIndexEnd")
        .unwrap()
        .parse::<u8>()
        .unwrap();
    let baud_rate = 115200;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let (tx, rx) = mpsc::channel();
    let id_to_monitor = matches
        .value_of("canIdToMonitor")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    thread::spawn(move || {
        app::serial_worker::read_serial(port_name, baud_rate, tx, id_to_monitor);
    });

    terminal.clear()?;

    let mut chart_model = models::ChartModel::new(id_to_monitor, 100);
    loop {
        match rx.try_recv() {
            Ok(line) => {
                if line.id == id_to_monitor {
                    let value = line.get_value(byte_index_start, byte_index_end);
                    chart_model.add_value(value);
                }
            }
            Err(mpsc::TryRecvError::Empty) => (),
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("Stopping.");
                break;
            }
        }
        terminal.draw(|f| app::ui::draw(f, &chart_model))?;

        if event::poll(Duration::from_secs(0)).unwrap() {
            if let CEvent::Key(key) = event::read().unwrap() {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    terminal.clear()?;
    Ok(())
}

pub fn is_u8(v: String) -> Result<(), String> {
    let val = v.parse::<u8>();
    if val.is_ok() && val.unwrap() < 8 {
        return Ok(());
    }
    Err(format!(
        "{} isn't a valid value between 0 an 7 (inclusive)",
        &*v
    ))
}

pub fn is_u32(v: String) -> Result<(), String> {
    if v.parse::<u32>().is_ok() {
        return Ok(());
    }
    Err(format!("{} isn't a valid u32 value", &*v))
}

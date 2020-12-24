mod app;
mod models;

use crossterm::event::{self, Event as CEvent, KeyCode};
use std::{io, sync::mpsc, thread, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut should_break = false;
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        app::serial_worker::read_serial(&"COM3", 115200, tx);
    });

    terminal.clear()?;

    let id_to_monitor = 128;

    let mut chart_model = models::ChartModel::new(id_to_monitor, 100);
    loop {
        match rx.try_recv() {
            Ok(line) => {
                if line.id == id_to_monitor {
                    let value = line.get_value(5, 6);
                    chart_model.add_value(value);
                }
            }
            Err(mpsc::TryRecvError::Empty) => (),
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("Stopping.");
                break;
            }
        }
        terminal.draw(|f| app::ui::draw_charts(f, &chart_model))?;

        if event::poll(Duration::from_secs(0)).unwrap() {
            if let CEvent::Key(key) = event::read().unwrap() {
                if key.code == KeyCode::Char('q') {
                    should_break = true
                }
            }
        }

        if should_break {
            break;
        }
    }

    terminal.clear()?;
    Ok(())
}
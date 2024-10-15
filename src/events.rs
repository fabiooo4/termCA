pub mod ant_events;
pub mod main_events;

use crossterm::event::poll;
use std::{io, time::Duration};

pub fn is_event_available(speed: Duration) -> io::Result<bool> {
    poll(speed)
}

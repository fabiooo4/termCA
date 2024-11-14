pub mod main_events;
pub mod ant_events;
pub mod elementary_events;
pub mod game_of_life_events;

use crossterm::event::poll;
use std::{io, time::Duration};

pub fn is_event_available(speed: Duration) -> io::Result<bool> {
    poll(speed)
}

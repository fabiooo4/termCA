use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Color;

use crate::{
    app::{App, Screen},
    simulations::ant::AntSim,
};

pub fn main(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.current_screen = Screen::Main;
            app.stop_all();
        }
        KeyCode::Char(' ') => app.is_running = !app.is_running,
        KeyCode::Char('?') => {
            app.help_screen = !app.help_screen;
        }
        KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => {
            // Run simulation once
            AntSim::run_ant_sim(app);
        }
        KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
            // Increase simulation speed
            if app.speed > Duration::from_millis(100) {
                app.speed = app.speed.saturating_sub(Duration::from_millis(100));
            } else if app.speed > Duration::from_millis(10) {
                app.speed = app.speed.saturating_sub(Duration::from_millis(10));
            } else if app.speed > Duration::from_millis(0) {
                app.speed = app.speed.saturating_sub(Duration::from_millis(1));
            } else {
                if app.speed_multiplier < 10 {
                    app.speed_multiplier = app.speed_multiplier.saturating_add(1);
                } else if app.speed_multiplier < 100 {
                    app.speed_multiplier = app.speed_multiplier.saturating_add(10);
                } else if app.speed_multiplier < 1000 {
                    app.speed_multiplier = app.speed_multiplier.saturating_add(100);
                } else {
                    app.speed_multiplier = app.speed_multiplier.saturating_add(1000);
                }
            }
        }
        KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
            // Decrease simulation speed
            if app.speed_multiplier > 1 {
                if app.speed_multiplier > 1000 {
                    app.speed_multiplier = app.speed_multiplier.saturating_sub(1000);
                } else if app.speed_multiplier > 100 {
                    app.speed_multiplier = app.speed_multiplier.saturating_sub(100);
                } else if app.speed_multiplier > 10 {
                    app.speed_multiplier = app.speed_multiplier.saturating_sub(10);
                } else {
                    app.speed_multiplier = app.speed_multiplier.saturating_sub(1);
                }
            } else if app.speed < Duration::from_millis(10) {
                app.speed = app.speed.saturating_add(Duration::from_millis(1));
            } else if app.speed < Duration::from_millis(100) {
                app.speed = app.speed.saturating_add(Duration::from_millis(10));
            } else {
                app.speed = app.speed.saturating_add(Duration::from_millis(100));
            }
        }
        _ => {}
    }
}

pub fn edit(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.editing = None;
            app.ant_sim = None;
        }
        KeyCode::Char(c) => {
            app.ant_sim.as_mut().unwrap().rules_input.push(c);
        }
        _ => {}
    }
}

pub fn resize(new_width: u16, new_height: u16, app: &mut App) {
    let new_width: usize = new_width as usize - 2;
    let new_height: usize = (new_height as usize - 2) * 2;

    // Reposition the ant in the view if it is out of bounds
    for ant in app.ant_sim.as_mut().unwrap().ants.iter_mut() {
        if ant.x >= new_width {
            ant.x = new_width - 1;
        }

        if ant.y >= new_height {
            ant.y = new_height - 1;
        }
    }

    // Resize the grid
    app.ant_sim
        .as_mut()
        .unwrap()
        .grid
        .resize(new_width, new_height, Color::Reset);
}

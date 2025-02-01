mod chemistry;
mod physics;
mod ui;

use crossterm::{
    event,
    event::{Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    time::{Duration, Instant},
};

use chemistry::simulation::Simulation;
use ui::{draw_frame, draw_grid, init_ui};

fn main() -> io::Result<()> {
    // Инициализация TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = init_ui();

    let mut simulation = Simulation::new();
    let mut last_update = Instant::now();
    let dt = 2e-17;
    let mut paused = false;

    // Главный цикл
    loop {
        // Обработка ввода
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => paused = !paused,
                    _ => {}
                }
            }
        }

        // Обновление и отрисовка
        if !paused {
            simulation.update(dt);
        }

        terminal.draw(|f| {
            let size = f.size();
            draw_grid(f, size);
            draw_frame(f, &simulation, size).unwrap();
        })?;
    }

    // Завершение работы
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

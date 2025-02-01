use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction},
};
use ratatui::{
    layout::Layout,
    prelude::{Backend, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;

pub fn init_ui() -> Terminal<CrosstermBackend<io::Stdout>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend).unwrap()
}

pub fn draw_frame<B: Backend>(
    f: &mut Frame<B>,
    simulation: &crate::chemistry::simulation::Simulation,
    area: Rect,
) -> Result<(), io::Error> {
    // Основной блок
    let block = Block::default()
        .title("Atom Collision Simulator")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));

    f.render_widget(block, area);

    // Область для отрисовки частиц
    let main_area = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(area)[0];

    // Отрисовка частиц
    for atom in &simulation.atoms {
        draw_particle(f, &atom.proton, main_area, Color::Red);
        draw_particle(f, &atom.electron, main_area, Color::Blue);
    }

    // Статус бар
    let status = Paragraph::new(format!(
        "Time: {:.2e} s | SPACE - пауза | Q - выход",
        simulation.time
    ))
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::Cyan));

    f.render_widget(status, area);
    Ok(())
}

pub fn draw_particle<B: Backend>(
    f: &mut ratatui::Frame<B>,
    particle: &crate::physics::particle::Particle,
    area: ratatui::layout::Rect,
    color: Color,
) {
    let scale = 5e11; // Увеличили масштаб ещё на 66%
    let area_size = area.width.min(area.height) as f64;
    let max_pos = 2e-10; // Уменьшили область видимости

    let x = ((particle.position[0] / max_pos) * (area_size / 2.0)) as i32 + (area.width / 2) as i32;
    let y =
        ((particle.position[1] / max_pos) * (area_size / 2.0)) as i32 + (area.height / 2) as i32;

    // Отрисовка протона
    if color == Color::Red {
        f.render_widget(
            Paragraph::new("◉").style(Style::default().fg(color)),
            ratatui::layout::Rect::new(x as u16, y as u16, 1, 1),
        );
    }
    // Отрисовка электрона
    else {
        f.render_widget(
            Paragraph::new("◦").style(Style::default().fg(color)),
            ratatui::layout::Rect::new(x as u16, y as u16, 1, 1),
        );
    }

    // Траектория
    for i in 0..15 {
        // Увеличили длину следа
        let trail_x =
            x + ((particle.position[0] - particle.velocity[0] * i as f64 * 5e-18) * scale) as i32;
        let trail_y =
            y + ((particle.position[1] - particle.velocity[1] * i as f64 * 5e-18) * scale) as i32;

        if trail_x >= 0
            && trail_x < area.width as i32
            && trail_y >= 0
            && trail_y < area.height as i32
        {
            f.render_widget(
                Paragraph::new("·").style(Style::default().fg(color)),
                ratatui::layout::Rect {
                    x: trail_x as u16,
                    y: trail_y as u16,
                    width: 1,
                    height: 1,
                },
            );
        }
    }
}

pub fn draw_grid<B: Backend>(frame: &mut Frame<B>, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Горизонтальные линии
    for i in 0..=10 {
        let y = inner.y + (inner.height as f32 * (i as f32 / 10.0)) as u16;
        frame.render_widget(
            Paragraph::new("─".repeat(inner.width as usize)),
            Rect::new(inner.x, y, inner.width, 1),
        );
    }

    // Вертикальные линии
    for i in 0..=10 {
        let x = inner.x + (inner.width as f32 * (i as f32 / 10.0)) as u16;
        frame.render_widget(
            Paragraph::new("│").style(Style::default().fg(Color::DarkGray)),
            Rect::new(x, inner.y, 1, inner.height),
        );
    }
}

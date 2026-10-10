use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};
use tui_big_text::{BigText, PixelSize};

pub fn run(title: &str) -> Result<Duration> {
    ratatui::run(|terminal| App::new(title).run(terminal))
}

#[derive(Debug)]
pub struct App {
    title: String,
    time_text: String,
    start_time: Option<Instant>,
    elapsed: Duration,
    is_paused: bool,
    exit: bool,
}

impl App {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            time_text: "00:00.00".to_string(),
            start_time: Some(Instant::now()),
            elapsed: Duration::ZERO,
            is_paused: false,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<Duration> {
        let tick_rate = Duration::from_millis(30);
        let mut last_tick = Instant::now();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)?
                && let Event::Key(key_event) = event::read()?
                && key_event.kind == KeyEventKind::Press
            {
                self.handle_key_event(key_event);
            }

            if last_tick.elapsed() >= tick_rate {
                self.update_time_text();
                last_tick = Instant::now();
            }
        }
        Ok(self.elapsed)
    }

    fn current_elapsed(&self) -> Duration {
        if let Some(instant) = self.start_time {
            self.elapsed + instant.elapsed()
        } else {
            self.elapsed
        }
    }

    fn update_time_text(&mut self) {
        let total_secs = self.current_elapsed().as_secs();

        let hours = total_secs / 3600;
        let minutes = (total_secs / 60) % 60;
        let seconds = total_secs % 60;

        self.time_text = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char(' ') => self.pause_timer(),
            _ => {}
        }
    }

    fn pause_timer(&mut self) {
        if let Some(start_time) = self.start_time.take() {
            self.elapsed += start_time.elapsed();
            self.is_paused = true;
        } else {
            self.start_time = Some(Instant::now());
            self.is_paused = false;
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Length(8),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Fill(1),
            ])
            .split(area);

        let title_widget = Paragraph::new(self.title.as_str())
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray).bold());
        title_widget.render(vertical[1], buf);

        let big_text = BigText::builder()
            .pixel_size(PixelSize::Full)
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Center)
            .lines(vec![self.time_text.as_str().into()])
            .build();

        big_text.render(vertical[3], buf);

        if self.is_paused {
            let paused_widget = Paragraph::new("PAUSED")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Gray).bold());
            paused_widget.render(vertical[5], buf);
        }
    }
}

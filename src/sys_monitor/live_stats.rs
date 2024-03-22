use crossterm::{
    event::{self, DisableMouseCapture, poll, read, Event as CEvent, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, LeaveAlternateScreen, ClearType, Clear},
};
use std::io;
use std::{sync::mpsc, thread, time::Duration};
use std::collections::VecDeque;

use sysinfo::{Disks, Networks, System};

use tui::{backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, widgets::{Block, Borders, Gauge}, Terminal};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Paragraph, Sparkline};

// Show live stats of the system
pub fn show_live_stats() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, Clear(ClearType::All))?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create a channel to receive events
    let (tx, _rx) = mpsc::channel();
    thread::spawn(move || {
        while let Ok(_) = tx.send(event::read()) {}
    });

    const CPU_HISTORY_SIZE: usize = 60;
    let mut cpu_history: VecDeque<u64> = VecDeque::with_capacity(CPU_HISTORY_SIZE);

    let mut sys = System::new_all();
    let mut disks = Disks::new_with_refreshed_list();
    let mut network = Networks::new();
    loop {
        terminal.draw(|f| {
            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(75),
                ])
                .split(f.size());

            let right_column_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(30),
                ])
                .split(main_chunks[1]);

            let left_column_stats = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .split(main_chunks[0]);

            sys.refresh_all();
            disks.refresh();
            network.refresh();
            network.refresh_list();

            let total_memory = sys.total_memory();
            let used_memory = sys.used_memory();
            let memory_usage = if total_memory > 0 {
                (used_memory as f64 / total_memory as f64) * 100.0
            } else {
                0.0
            };

            let network_usage = network.iter().fold(0.0, |acc, (_, data)| {
                acc + data.received() as f64 / 1_024.0 + data.transmitted() as f64 / 1_024.0
            });

            let mut total_read_bytes = 0_u64;
            let mut total_written_bytes = 0_u64;
            for (_pid, process) in sys.processes() {
                let disk_usage = process.disk_usage();
                total_read_bytes += disk_usage.read_bytes;
                total_written_bytes += disk_usage.written_bytes;
            }
            let disk_usage = (total_read_bytes as f64 + total_written_bytes as f64) / 1_000_000.0;

            let cpu_usage = sys.global_cpu_info().cpu_usage();
            if cpu_history.len() >= CPU_HISTORY_SIZE {
                cpu_history.pop_front();
            }
            cpu_history.push_back(cpu_usage as u64);
            let cpu_data: Vec<u64> = cpu_history.iter().cloned().collect();

            let cpu_speed = sys.global_cpu_info().frequency() as f64 / 1000.0;
            let memory_usage_info = format!("{} / {} MB", used_memory / 1024, total_memory / 1024);
            let disk_usage_info = format!("{:.2}%", disk_usage);

            let cpu_speed_text = Paragraph::new(format!("CPU Speed: {:.2} GHz", cpu_speed))
                .block(Block::default().borders(Borders::ALL).title("CPU Speed"));
            let memory_usage_text = Paragraph::new(memory_usage_info)
                .block(Block::default().borders(Borders::ALL).title("Memory"));
            let network_speed_text = Paragraph::new(format!("Internet: {:.0} Kbits/s", network_usage))
                .block(Block::default().borders(Borders::ALL).title("Network"));
            let disk_usage_text = Paragraph::new(disk_usage_info)
                .block(Block::default().borders(Borders::ALL).title("Disk"));


            let cpu_sparkline = Sparkline::default()
                .block(Block::default().title("CPU History").borders(Borders::ALL))
                .data(&cpu_data)
                .style(Style::default().fg(Color::LightCyan));

            let gauge_style = Style::default()
                .fg(Color::LightCyan)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD);

            let cpu_gauge = Gauge::default()
                .block(Block::default().title("CPU Usage").borders(Borders::ALL))
                .gauge_style(gauge_style)
                .percent(cpu_usage as u16);

            let memory_gauge = Gauge::default()
                .block(Block::default().title("Memory Usage").borders(Borders::ALL))
                .gauge_style(gauge_style)
                .percent(memory_usage as u16);

            let network_gauge = Gauge::default()
                .block(Block::default().title("Network Usage").borders(Borders::ALL))
                .gauge_style(gauge_style)
                .percent(network_usage as u16);

            let disk_gauge = Gauge::default()
                .block(Block::default().title("Disk Usage").borders(Borders::ALL))
                .gauge_style(gauge_style)
                .percent(disk_usage as u16);

            f.render_widget(cpu_gauge, right_column_chunks[0]);
            f.render_widget(memory_gauge, right_column_chunks[1]);
            f.render_widget(network_gauge, right_column_chunks[2]);
            f.render_widget(disk_gauge, right_column_chunks[3]);
            f.render_widget(cpu_sparkline, right_column_chunks[4]);

            f.render_widget(cpu_speed_text, left_column_stats[0]);
            f.render_widget(memory_usage_text, left_column_stats[1]);
            f.render_widget(network_speed_text, left_column_stats[2]);
            f.render_widget(disk_usage_text, left_column_stats[3]);
        })?;

        // Check if there is an event
        if poll(Duration::from_millis(100))? {
            match read()? {
                CEvent::Key(key) => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        execute!(terminal.backend_mut(), Clear(ClearType::All))?;
                        break;
                    },
                    _ => {}
                },
                _ => {}
            }
        }
        thread::sleep(Duration::from_secs(1));
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

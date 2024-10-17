use std::io;
use std::process;

use crate::build;

use clap::Command;
use clap::CommandFactory;
use clap::Parser;

use crossterm::execute;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use ratatui::backend::CrosstermBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::block::Position;
use ratatui::widgets::BorderType;
use ratatui::widgets::Widget;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;

#[derive(Parser, Debug)]
#[command(name = build::PROJECT_NAME, author, version, about, disable_help_flag = true)]
pub struct Args {
    /// Print help
    #[arg(short, long)]
    pub help: bool,

    /// The text to copy to the clipboard
    #[arg(name = "TEXT")]
    pub text: Option<String>,
}

/// Implements the `Args` struct and its associated methods.
impl Args {
    /// Initializes the command-line interface (CLI) and returns an `Args` object.
    /// ```
    /// let args = Args::init_cli();
    /// ```
    /// `exit` on error
    ///
    /// # Returns
    ///
    /// An `Args` object with the parsed command-line arguments.
    ///
    pub fn init_cli() -> Args {
        let args = Self::parse();
        if args.help {
            Self::print_help().unwrap();
            process::exit(0)
        }
        args
    }

    /// Prints the help message for the CLI.
    ///
    /// The help message is styled using the `clap-help` and
    /// the `termimad` crates.
    pub fn print_help() -> io::Result<()> {
        // Set up the Crossterm backend
        enable_raw_mode()?;

        let command = Args::command();
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        execute!(terminal.backend_mut(), Clear(ClearType::All))?;

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.area());

            let command_widget = CliArguemntWidget::new(command);
            f.render_widget(command_widget, chunks[0]);
        })?;

        disable_raw_mode()?;
        Ok(())
    }
}

pub struct CliArguemntWidget<'a> {
    pub arguments: Vec<Paragraph<'a>>,
}

impl<'a> CliArguemntWidget<'a> {
    pub fn new(command: Command) -> Self {
        let cli_arguments: Vec<_> = command.get_arguments().collect();
        let arguments = cli_arguments
            .iter()
            .map(|arg| {
                let mut text = format!("**{}**", arg.get_long().unwrap_or_default());
                if let Some(short) = arg.get_short() {
                    text.push_str(&format!(", **{}**", short));
                }
                if let Some(long) = arg.get_long() {
                    text.push_str(&format!(", **{}**", long));
                }
                text.push_str(&format!(": {}", arg.get_help().unwrap_or_default()));
                text
            })
            .map(Paragraph::new)
            .collect::<Vec<_>>();
        Self { arguments }
    }
}

impl<'a> Widget for CliArguemntWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [main] =
            Layout::vertical([Constraint::Length(self.arguments.len() as u16 + 2)]).areas(area);
        let chunks = Layout::vertical(
            self.arguments
                .iter()
                .map(|_| Constraint::Min(1))
                .collect::<Vec<_>>(),
        )
        .margin(1)
        .split(main);
        let block = Block::default()
            .borders(Borders::ALL)
            .title_position(Position::Top)
            .title("─ Options ")
            .border_style(Style::new().fg(Color::Gray))
            .border_type(BorderType::Rounded);
        block.render(main, buf);
        for (i, argument) in self.arguments.into_iter().enumerate() {
            let [left, _right] =
                Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .areas(chunks[i]);
            argument.render(left, buf);
        }
    }
}

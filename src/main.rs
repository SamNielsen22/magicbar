mod render;
mod transport;

use std::path::PathBuf;

use anyhow::Context as _;
use clap::{Parser, Subcommand};

use render::{Canvas, Rgb};
use transport::Display;

// Used for --png, where there is no hardware to ask.
const PANEL: (u32, u32) = (2170, 60);
const BG: Rgb = (0.0, 0.0, 0.0);
const FG: Rgb = (1.0, 1.0, 1.0);

#[derive(Parser)]
#[command(name = "magicbar", about = "App-aware Touch Bar daemon for Linux")]
struct Cli {
    /// Print debug output
    #[arg(short, long)]
    verbose: bool,
    /// Render to a PNG file instead of the bar
    #[arg(long, value_name = "FILE")]
    png: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Show the Touch Bar card, connector and modes
    Info,
    /// Fill the bar with one colour, e.g. "#ff8800"
    Fill { colour: String },
    /// Draw a line of text
    Text { text: String },
    /// Run the daemon
    Run,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let level = if cli.verbose { tracing::Level::DEBUG } else { tracing::Level::INFO };
    tracing_subscriber::fmt().with_max_level(level).init();

    match cli.command {
        Command::Info => transport::drm::info(),
        Command::Fill { colour } => {
            let colour = parse_colour(&colour)?;
            show(cli.png, |canvas| canvas.fill(colour))
        }
        Command::Text { text } => show(cli.png, |canvas| {
            canvas.fill(BG)?;
            canvas.text(&text, 28.0, FG)
        }),
        Command::Run => anyhow::bail!("the daemon is not written yet"),
    }
}

// Draw, then save a PNG or show it on the bar until Enter.
fn show<F>(png: Option<PathBuf>, draw: F) -> anyhow::Result<()>
where
    F: FnOnce(&mut Canvas) -> anyhow::Result<()>,
{
    if let Some(path) = png {
        let (width, height) = PANEL;
        let mut canvas = Canvas::new(width, height)?;
        draw(&mut canvas)?;
        canvas.save_png(&path)?;
        println!("wrote {}", path.display());
        return Ok(());
    }

    let mut display = transport::drm::DrmDisplay::open()?;
    let (width, height) = display.size();
    let mut canvas = Canvas::new(width, height)?;
    draw(&mut canvas)?;
    display.present(&canvas)?;

    println!("showing on the bar. Press Enter to exit.");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    Ok(())
}

fn parse_colour(s: &str) -> anyhow::Result<Rgb> {
    let hex = s.trim_start_matches('#');
    let n = u32::from_str_radix(hex, 16)
        .ok()
        .filter(|_| hex.len() == 6)
        .with_context(|| format!("colour must look like #rrggbb, got {s}"))?;
    let channel = |shift: u32| ((n >> shift) & 0xff) as f64 / 255.0;
    Ok((channel(16), channel(8), channel(0)))
}

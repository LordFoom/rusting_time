use clap::{arg, Parser};
use crossterm::style::Stylize;
use crossterm::terminal::size;
use crossterm::{cursor, style, terminal, ExecutableCommand, QueueableCommand};
use std::error::Error;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
#[derive(Parser)]
#[command(name = "")]
#[command(author = "Foom")]
#[command(version = "1.0")]
#[command(
    about = "The passage of time is the rust of clocks",
    long_about = "none"
)]
struct Args {
    #[arg(long, short)]
    time: Option<u128>,
}

struct Timer {
    current_duration: Duration,
}
///Count down timer for the terminal
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    //how many minutes are we going to do?
    let millis = if let Some(t) = args.time {
        t * 1000 * 60
    } else {
        //half an hour
        30 * 60 * 1000
        //but for testing, 30s
        // 30*1000
    };
    //get the original size
    let (cols, rows) = size()?;
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(terminal::SetSize(4, 12))?;
    stdout.execute(terminal::SetTitle("Time is the Rust of Clocks"))?;
    //set the start time

    let mut count = 0;
    loop {
        let start_instant = Instant::now();
        //set the label
        let pause_msieur = Duration::from_millis(100);
        loop {
            //clear the screen, don't want litter
            print!("\x1B[2J");
            let elapsed_millis = start_instant.elapsed().as_millis();
            let remaining_millis = if elapsed_millis < millis {
                millis - elapsed_millis
            } else {
                0
            };
            stdout.queue(cursor::MoveTo(5, 5))?;
            stdout.queue(style::PrintStyledContent(
                format_millis(remaining_millis).cyan(),
            ))?;
            stdout.queue(style::PrintStyledContent(format_count(count).yellow()))?;
            stdout.flush()?;
            std::thread::sleep(pause_msieur);
            if remaining_millis <= 0 {
                break;
            }
        }

        count += 1;
        if count > 10 {
            break;
        }
    }
    //restore
    stdout.execute(terminal::SetSize(cols, rows))?;
    // println!("That's the spiritus!");
    Ok(())
}

fn format_millis(millis: u128) -> String {
    let seconds = millis / 1000;
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    let remaining_millis = millis % 1000;
    format!(
        "{:0>3}:{:0>2}.{} ",
        minutes, remaining_seconds, remaining_millis
    )
}
fn format_count(count: u128) -> String {
    format!("count={}", count)
}

use std::error::Error;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use clap::{arg, Parser};
use crossterm::{cursor, ExecutableCommand, QueueableCommand, style, terminal};
use crossterm::style::Stylize;
use crossterm::terminal::size;
#[derive(Parser)]
#[command(name = "")]
#[command(author = "Foom")]
#[command(version = "0.1")]
#[command(
about = "The passage of time is the rust of clocks",
long_about = "none"
)]
struct Args{
    #[arg(long, short)]
    time: Option<String>,
}

struct Timer{
    current_duration: Duration,

}
///Count down timer for the terminal
fn main()->Result<(), Box<dyn Error>> {
    let args = Args::parse();
    //how many seconds are we going to do?
    let mut millis = if let Some(t) = args.time{
        t.parse::<u128>()? * 1000
    } else {
        //half an hour
        30*60*1000
        //but for testing, 30s
        // 30*1000
    };
    //get the original size
    let (cols,rows) = size()?;
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(terminal::SetSize(4,12))?;
    stdout.execute(terminal::SetTitle("Time is the Rust of Clocks"))?;
    //set the start time

    // let mut terminal =
    let start_instant = Instant::now();
    //set the label
    let pause_msieur = Duration::from_millis(100);
    loop{
        // println!("I like to parse it parse it {}", millis);
        let elapsed_millis = start_instant.elapsed().as_millis();
        // println!("elapsed={}, millis={}", elapsed_millis, millis);
        let remaining_millis = if elapsed_millis <  millis {
            millis- elapsed_millis
        }else{
            0
        };
        stdout.queue(cursor::MoveTo(5,5))?;
        stdout.queue(style::PrintStyledContent(format_millis(remaining_millis).cyan()))?;
        stdout.flush()?;
        std::thread::sleep(pause_msieur);
        if remaining_millis <= 0{
            break;
        }
    }
    //restore
    stdout.execute(terminal::SetSize(cols, rows))?;
    // println!("That's the spiritus!");
    Ok(())
    //start the timer
}

fn format_millis(millis: u128)->String {
    let seconds = millis / 1000;
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    let remaining_millis = millis % 1000;
    format!("{:0>3}:{:0>2}.{}", minutes, remaining_seconds, remaining_millis)
}

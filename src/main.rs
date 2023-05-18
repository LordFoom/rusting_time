use clap::{arg, Parser};

#[derive(Parser)]
#[command(name = "")]
#[command(author = "Foom")]
#[command(version = "0.1")]
#[command(
about = "The passage of time is the rust of clocks",
long_about = "none"
)]
struct Arg{
    time: Option<String>,
}
///Count down timer for the terminal
fn main() {
    println!("Hello, world!");
}

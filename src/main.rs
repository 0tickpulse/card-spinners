use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
mod card;
extern crate rand;

const SIMULATIONS: u32 = 10000000;

fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))
        .unwrap();
    writeln!(
        &mut stdout,
        "== Welcome to the simulation for Card Spinners =="
    )
    .unwrap();
    stdout.reset().unwrap();

    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))
        .unwrap();
    writeln!(&mut stdout, "== GAME INSTRUCTIONS ==").unwrap();
    stdout.reset().unwrap();

    writeln!(
        &mut stdout,
        "The spinner gives you one of the four options: number, suite, color"
    )
    .unwrap();
    writeln!(&mut stdout, "When the spinner gives you an option, you choose something from the option. For example, if the spinner gives you a number, you choose a number from 1 to 13").unwrap();
    writeln!(&mut stdout, "Draw a card from the deck of cards. If the card matches your choice, you win. Otherwise, you lose.").unwrap();

    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))
        .unwrap();
    writeln!(&mut stdout, "== SIMULATION DETAILS ==").unwrap();
    stdout.reset().unwrap();
    writeln!(&mut stdout, "Number of simulations: {}", SIMULATIONS).unwrap();
    writeln!(&mut stdout, "Number of threads: {}", SIMULATIONS / 1000).unwrap();

    // create threads
    let mut threads = Vec::new();
    for _ in 0..SIMULATIONS / 1000 {
        threads.push(std::thread::spawn(|| {
            let mut wins = 0;
            let mut losses = 0;
            for _ in 0..1000 {
                let sim = simulation();
                if sim {
                    wins += 1;
                } else {
                    losses += 1;
                }
            }
            (wins, losses)
        }));
    }

    // join threads
    let mut wins = 0;
    let mut losses = 0;
    for thread in threads {
        let result = thread.join().unwrap();
        wins += result.0;
        losses += result.1;
    }

    // write simulation results
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))
        .unwrap();
    writeln!(&mut stdout, "== SIMULATION RESULTS ==").unwrap();
    stdout.reset().unwrap();

    writeln!(&mut stdout, "Wins: {}", wins).unwrap();
    writeln!(&mut stdout, "Losses: {}", losses).unwrap();
    writeln!(
        &mut stdout,
        "Win percentage: {:.2}%",
        (wins as f32 / SIMULATIONS as f32) * 100.0
    )
    .unwrap();
}

fn simulation() -> bool {
    let spinner = card::random_spinner();
    let card = card::random_card();
    match spinner {
        card::Spinner::Suite => card.suite == card::random_suite(),
        card::Spinner::Color => card.color == card::random_color(),
        card::Spinner::Number => card.number == card::random_number(),
    }
}

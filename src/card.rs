use rand::Rng;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Spinner {
    Number,
    Suite,
    Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suite {
    Spade,
    Heart,
    Diamond,
    Club,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub suite: Suite,
    pub number: u8,
    pub color: Color,
}

impl Card {
    pub fn new(suite: Suite, number: u8) -> Card {
        let color = match suite {
            Suite::Spade | Suite::Club => Color::Black,
            Suite::Heart | Suite::Diamond => Color::Red,
        };
        Card {
            suite,
            number,
            color,
        }
    }
}

pub const NUMBER_MAX: u8 = 13;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Red,
    Black,
}

pub fn random_spinner() -> Spinner {
    let mut rng = rand::thread_rng();
    let spinner: Spinner = match rng.gen_range(0..=2) {
        0 => Spinner::Number,
        1 => Spinner::Suite,
        2 => Spinner::Color,
        _ => panic!("Invalid spinner"),
    };
    spinner
}

pub fn random_suite() -> Suite {
    let mut rng = rand::thread_rng();
    let suite: Suite = match rng.gen_range(0..=3) {
        0 => Suite::Spade,
        1 => Suite::Heart,
        2 => Suite::Diamond,
        3 => Suite::Club,
        _ => panic!("Invalid suite"),
    };
    suite
}

pub fn random_number() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=NUMBER_MAX)
}

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    let color: Color = match rng.gen_range(0..=1) {
        0 => Color::Red,
        1 => Color::Black,
        _ => panic!("Invalid color"),
    };
    color
}

pub fn random_card() -> Card {
    let suite = random_suite();
    let number = random_number();
    Card::new(suite, number)
}


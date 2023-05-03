use console::Term;
use std::time::{Instant};

fn main() {
    let mut count = 1;
    loop {
    
    let term = Term::stdout();
    term.write_line(&("-------------------------------------------------------------------------\n".to_owned() + &count.to_string())).unwrap();
    term.write_line("ONE").unwrap();
    while term.read_char().unwrap() != ' ' {}

    let start = Instant::now();

    term.write_line("TWO").unwrap();
    while term.read_char().unwrap() != ' ' {}

    let duration = start.elapsed();

    let hz = 1.0 / duration.as_secs_f64();
    term.write_line(&format!("The frequency between the two spacebar presses is: {} Hz", hz))
        .unwrap();
    
    count = count + 1;
    }
}
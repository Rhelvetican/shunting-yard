extern crate shunting_yard;

use std::env;
use std::io::prelude::*;

fn main() {
    let inp = env::args().collect::<Vec<String>>();
    if inp.len() >= 2 {
        let v = shunting_yard::evaluate::<f64>(inp.last().unwrap()).expect("Invalid input.");
        println!("{}", v);
    } else {
        repl();
    };
}

fn repl() {
    let input = std::io::stdin();
    let mut input_lock = input.lock();
    loop {
        print!("> ");
        std::io::stdout().flush().expect("Can't flush stdout.");
        let mut buffer = String::new();
        input_lock.read_line(&mut buffer).expect("Can't read line.");
        let v = shunting_yard::evaluate::<f64>(&buffer).expect("Invalid input.");
        println!(">>> {}", v);
        buffer.clear();
    }
}

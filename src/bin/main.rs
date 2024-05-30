extern crate shunting_yard;

use anyhow::Result;
use shunting_yard::evaluate;
use std::{
    env::args,
    io::{prelude::*, stdin, stdout},
};

fn main() -> Result<()> {
    let inp = args().collect::<Vec<String>>();
    if inp.len() >= 2 {
        let v = evaluate::<f64>(inp.last().unwrap())?;
        println!("{}", v);
    } else {
        repl()?;
    };
    Ok(())
}

fn repl() -> Result<()> {
    let input = stdin();
    let mut input_lock = input.lock();
    loop {
        print!("> ");
        stdout().flush()?;
        let mut buffer = String::new();
        input_lock.read_line(&mut buffer).expect("Can't read line.");
        if buffer.trim() == "exit" {
            break;
        }
        let v = evaluate::<f64>(&buffer)?;
        buffer.clear();
        println!(">>> {}", v);
    }
    Ok(())
}

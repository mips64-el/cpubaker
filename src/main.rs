use num_cpus;
use std::{thread, io::{self, Write}};

fn main() {
    let num_cores = num_cpus::get();

    for _ in 0..num_cores {
        thread::spawn(|| { 
            loop {}
        });
    }

    print!("CPU baker already running, if you wish to stop press enter...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();

}

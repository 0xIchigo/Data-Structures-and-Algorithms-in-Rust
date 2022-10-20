// An algorithm that prints out all positive integers up to 2000...because why not!

use std::fs::OpenOptions;
use std::io::{ BufWriter, Write };

fn main() {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("numbers.txt")
        .expect("Unable to open numbers.txt");
    
    let mut f = BufWriter::new(file);

    for i in 1..2001 {
        if i % 50 == 0 { write!(f, "\n")
            .expect("Unable to write to numbers.txt"); }
        write!(f, "{}", i).expect("Unable to write to numbers.txt");
    }
}
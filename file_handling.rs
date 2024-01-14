use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom::Start;

fn read_all_lines(filename: &str) -> io::Result<()> {
    // file is a handle(identifier to some resource) given by OS
    // to perform operations (abstracted)
    let file = File::open(&filename)?;
    // BufReader temporarily store's data read from file
    let mut reader = io::BufReader::new(file);

    let mut buf = String::new();

    while reader.read_line(&mut buf)? > 0 {
        let line = buf.trim_right();
        println!("{}",line);
    }
    buf.clear();

    // * Inefficient, new string is allocated for each line
    // for line in reader.lines() {
    //     let line = line?;
    //     println!("{}", line);
    // }
    Ok(())
}

fn main () {
   read_all_lines("input.txt");

}

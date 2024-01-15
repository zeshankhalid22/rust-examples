use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Lines<R> {
    // BufReader temporarily store's data read from file
    reader: io::BufReader<R>,
    buf: String,
}

impl<R: Read> Lines<R> {
    fn new(r: R) -> Lines<R> {
        Lines {
            reader: io::BufReader::new(r),
            buf: String::new(),
        }
    }
    // Result<T, io::Error>
    fn next(&mut self) -> Option<io::Result<&str>> {
        self.buf.clear();
        match self.reader.read_line(&mut self.buf) {
            Ok(nBytes) =>
                if nBytes == 0 {
                    None   // 0/no line
                }
            else {
                let line = self.buf.trim_right();
                Some(Ok(line))
            }
            Err(e) => Some(Err(e))
        }
    }
}


fn read_all_lines(filename: &str) -> io::Result<()> {
    // file is a handle(identifier to some resource) given by OS
    // to perform operations (abstracted)
    let file = File::open(&filename)?;

    let mut lines = Lines::new(file);


    while let Some(line) = lines.next()  {
        // if line Err, return
        let line = line?;
        println!("{}", line);
    }

    // * Inefficient, new string is allocated for each line
    // for line in reader.lines() {
    //     let line = line?;
    //     println!("{}", line);
    // }
    Ok(())
}

fn main() {
    read_all_lines("input.txt");
}

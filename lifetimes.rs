
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    // arguments have to live at least 'a time scope
    fn new(heystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: heystack,
            delimiter,
        }
    }
}

// StrSplit { remainder, delimiter}
impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item>{
        // Some(usize) if delim found at Some<> index within remainder
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            // slice(&) from start(..) to next_delim
            let until_remainder = &self.remainder[..next_delim];
                   //  slice to (start of next_delim + it's length)...(continued)
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            // &str till delimiter cutted down from remainder, and returned
            Some(until_remainder)
        }
            // if it all consumed/returned
        else if self.remainder.is_empty() {
            None
        }
        // if delim is not found, return whole
        else {
            let rest = self.remainder;
            //   &'a str   <- &'static str
            self.remainder = "";
            Some(rest)
        }
    }
}


fn main () {
    let haystack = "abc, dsd,sd nn, uy, ";
    let letters = StrSplit::new(haystack,", ");
    let mut j = 0;
    for i in letters{
        j = j + 1;
        println!("{}",i);
    }
    println!("Total words broken {}",j);
}

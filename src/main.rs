use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}", c);
        if c == 'q' {
            break;
        }
    }
}

use std::io::Write;
use std::io::stdout;

fn main() {
    print!("Hello, world!\n");
    stdout().flush().expect("Unable to flush stdout");
}

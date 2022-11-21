use std::io::{stdout, BufWriter};

use ferris_says::say;

fn main() {
    let stdout = stdout();
    let message = String::from("Hello test message");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

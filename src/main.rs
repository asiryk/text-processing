mod m0_str;

use m0_str::handle_string;
use std::fs::read_to_string;

fn main() {
    let str_file = read_to_string("assets/the-problems-of-philosophy.txt").unwrap();
    handle_string(str_file);
}

#![no_std]
#![no_main]

use noli::prelude::*;

fn main() {
    Api::write_string("Browser project step1 OK!\n");
    Api::exit(0);
}

entry_point!(main);
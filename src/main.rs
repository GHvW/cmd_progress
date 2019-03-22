use std::{ thread, time};
extern crate cmd_progress_lib;
use cmd_progress_lib as progress;

fn main() {
    println!("Hello, world!");
    let duration = time::Duration::from_millis(150);

    for i in 0..=100 {
        progress::print_progress(i).unwrap();
        thread::sleep(duration);
    }
}

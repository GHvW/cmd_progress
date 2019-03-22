use std::{ thread, time};
extern crate cmd_progress_lib;
use cmd_progress_lib as progress;

fn main() {
    println!("Hello, world!");
    let _duration = time::Duration::from_millis(1000);
    let o_duration = time::Duration::from_millis(150);

    // progress::print_progress(30).unwrap();
    // thread::sleep(duration);

    // progress::print_progress(40).unwrap();
    // thread::sleep(duration);
    
    // progress::print_progress(50).unwrap();
    // thread::sleep(duration);

    // progress::print_progress(60).unwrap();
    // thread::sleep(duration);

    // progress::print_progress(100).unwrap();
    // thread::sleep(duration);

    for i in 0..=100 {
        progress::print_progress(i).unwrap();
        thread::sleep(o_duration);
    }
}

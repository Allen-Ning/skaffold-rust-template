use std::{thread, time};

fn main() {
    while true {
        let ten_seconds = time::Duration::from_millis(1000);
        thread::sleep(ten_seconds);
        println!("Hello rust!!!!!");
    }
}

use std::thread;
use std::time::Duration;

fn main() {
    let example_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let s = example_closure(5);
    let n = example_closure(5);

    println!("s {}, n {}", s, n);
}

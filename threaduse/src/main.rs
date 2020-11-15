use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let a = String::from("123");
        let b = String::from("456");
        let c = String::from("789");

        println!("a ----> {:p}", &*a);
        println!("b ----> {:p}", &*b);
        println!("c ----> {:p}", &*c);

        let vals = vec![a, b, c];

        for val in vals {
            println!("s ----> {:p}", &*val);
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("r ----> {:p}", &*received);
    }
}

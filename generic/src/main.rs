use generic::{self};

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("{:?} {:?}", integer, float);

    let at = generic::NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
        hockey team in the NHL.",
        ),
    };

    let tw = generic::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    generic::notify(&at);
    generic::notify(&tw);

    //println!("{:?} {:?}", at, tw);

    let string1 = String::from("abcd");
    let result2;

    {
        let string2 = String::from("xyvvvvvvvvvvv");
        result2 = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result2);
    }

    let i = ImportantExcerpt { part: "abccba" };

    println!("{:?}", i);
}

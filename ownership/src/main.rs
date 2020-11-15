fn main() {
    let mut s = String::from("hello");

    let mut r1 = &mut s;
    change(&mut r1);

    println!("{}", r1)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}

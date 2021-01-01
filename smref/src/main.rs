#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::{cell::Cell, ops::Deref};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn test_fat_pointers() {
    let mut arr = [1, 2, 3, 4];
    let slice = &mut arr[1..4];
    slice[0] = 100;
    println!("{:?}", arr); // [1, 100, 3, 4]

    let slice = &arr[1..4];
    println!("{:p} {:?}", slice, unsafe {
        std::mem::transmute::<_, (usize, usize)>(slice)
    });

    // Output: 0x8a6c0ff4ac (594518471852, 3)
    //      0x8a6c0ff4ac 594518471852 这两个值是相等的。
    //      (594518471852, 3) 分别表示 具体数据的堆地址 和 长度 两个字段。
    //      注意这里是用 slice，而不是 &slice。(&slice表示这个变量本身的栈地址)

    println!("sizeof &[i32]:{}", std::mem::size_of::<&[i32]>());
    // Output: sizeof &[i32]:16
    // 因为包含了两个字段：地址 + 长度，所以其占用内存为 2 个 usize 类型大小
}

fn test_smart_pointers() {
    // 将本应存在栈上的地址，存在了堆上
    let mut num = Box::new(1);
    // num_address 指向 box 里面的具体内容（也就是储存在堆上的数值 1）
    let num_address1: *mut i32 = &mut *num;
    let num_address2: *mut i32 = &mut *num;
    unsafe {
        *num_address1 = 100;
        *num_address2 = 200
    }
    println!("{}", *num + 100)
    // Output: 200
}

fn high_unsafe() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe fn dangerous() {
    println!("I'm dangerous! ans I call c abs {}", abs(-3));
    println!("{}", HELLO_WORLD);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);
    //(&mut slice[..mid], &mut slice[mid..])

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len),
        )
    }
}

fn main() {
    let l1 = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));

    println!("{:?}", l1);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let a = 5;
    let b = MyBox::new(a);

    assert_eq!(5, a);
    assert_eq!(5, *b);

    let a1 = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a1));

    // let b1 = Cons(3, Rc::clone(&a1));
    // let c1 = Cons(4, Rc::clone(&a1));

    let bb = Box::new(5);
    println!("{}", bb);

    test_fat_pointers();
    test_smart_pointers();
    println!("-----------------------------------");
    println!("{}", HELLO_WORLD);
    high_unsafe();
    add_to_count(3);

    unsafe {
        dangerous();
        println!("COUNTER: {}", COUNTER);
    }

    let s = "hell".to_string();
    let bar = std::cell::Cell::new(s);

    let x = bar.into_inner();
    println!("{}", x);

    let x = std::cell::RefCell::new(vec![1, 2, 3, 4]);
    println!("{:?}", x.borrow());

    let mut mutx = x.borrow_mut();
    mutx.push(6);

    let mut mutxx = x.borrow_mut();

    println!("{:?}", x.borrow());
}

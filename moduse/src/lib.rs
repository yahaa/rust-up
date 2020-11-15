pub mod outer {
    pub fn a() {
        println!("function a");
    }
    pub fn b() {
        println!("function b");
    }

    pub mod inner {
        pub fn c() {
            println!("function c");
        }
        pub fn d() {
            println!("function d");
        }
    }
}

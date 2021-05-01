use std::fmt::{Debug, Display};

fn main() {
    another_function(5);
    let s1 = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;

    let tuple: (isize, isize) = (0, 9);
    let (t0, t1) = tuple;

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let black = Color(0, 0, 0);

    let user1 = User {
        email: String::from("someone@example.com"),
        username: "someusername123".into(),
        active: true,
        sign_in_count: 1,
    };
}

struct Foo<'foo> {
    bar: &'foo [u8],
}

#[derive(Debug)]
pub struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    pub fn new(username: String, email: String, sign_in_count: u64, active: bool) -> Self {
        Self {
            username,
            email,
            sign_in_count,
            active,
        }
    }

    pub fn print(&self) {
        println!("{:?}", self);
    }
}

struct Color(i32, i32, i32);

/// Bar is a const generic struct that holds a slice of `T` with length `L`.
struct Bar<T, const L: usize> {
    data: [T; L],
    length: L,
}

struct Wrapper(Vec<isize>);

fn another_function(x: i32) {
    println!("The value of x is: {}", x);

    if x % 4 == 0 {
        println!("number is divisible by 4");
    } else if x % 3 == 0 {
        println!("number is divisible by 3");
    } else if x % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

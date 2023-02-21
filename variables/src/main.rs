fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let a = 5;
    let a = a + 1;
    let a = a * 2;
    println!("The value of a is: {}", a);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let b = 98_222;
    println!("The value of b is: {}", b);
    let b = 0b1111_0000;
    println!("The value of b is: {}", b);

    let x = 2.0; // f64
    println!("The value of x:f64 is: {}", x);
    let y: f32 = 3.0; // f32
    println!("The value of y:f32 is: {}", y);

    let t = true;
    println!("The value of t:bool is: {}", t);
    let f: bool = false; // with explicit type annotation}
    println!("The value of f:bool is: {}", f);

    let c = 'z';
    println!("The value of c:char is: {}", c);
    let z = 'ℤ';
    println!("The value of z:char is: {}", z);
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat:char is: {}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of tup:x,y,z is: {},{},{}", x,y,z);
    println!("The value of tup:tuple is: {},{},{}", tup.0,tup.1,tup.2);

    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    another_function(5);

    let x = plus_one(5);

    println!("The value of x is: {}", x);

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    let s2 = s;
    let mut s3 = takes_ownership(s2);
    let slen = calculate_length(&s3);
    let s4 = &s3;
    let s5 = &s3;
    println!("{}, {}, {}", s4, s5, slen);
    change(&mut s3);
    println!("{}, {}", s3, slen);
    let s6 = first_word(&s3);
    println!("first word {}", s6);
    let user = build_user(String::from("email"),String::from("user"));

    println!("user {},{},{},{}",
        user.email, user.username, user.active, user.sign_in_count);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);
    println!("The area of the rectangle is {} square pixels.",
        rect1.area());
    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("The area can_hold {}.",
        rect1.can_hold(&rect2));
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("value_in_cents {}.",value_in_cents(Coin::Quarter(UsState::Alabama)));
    let six = option_plus_one(Some(5));
    let non_option = option_plus_one(None);
}

fn another_function(x: i32) {
    println!("Another function. x {}",x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn takes_ownership(some_string: String) -> String { // some_string comes into scope
    println!("{}", some_string);
    some_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str("!");
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
    }
}
enum IpAddr {
    V4(String),
    V6(String),
}
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
fn option_plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

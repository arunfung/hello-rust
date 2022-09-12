use std::{fs, io};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // 1、猜测数字的小游戏
    // guess_the_number();

    // 2、请求一个url并将获取的HTML转成md
    // html_to_md();

    // 3、函数传参
    // function_parameter();

    // 分号;表示其返回值为 unit
    returned_value();
}

// 1、猜测数字的小游戏
fn guess_the_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// 2、请求一个url并将获取的HTML转成md
fn html_to_md() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";
    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes()).unwrap();
}

// 3、函数传参
fn function_parameter() {
    println!("apply square: {}", apply(2, square));
    println!("apply cube: {}", apply(2, cube));
}

fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

// 显示指定返回值
// 分号;表示其返回值为 unit
// 最后一个表达式就是返回值
fn returned_value() {
    let is_pi = pi();
    let is_unit1 = not_pi();
    let is_unit2 = {
        pi();
    };

    println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2);
}

fn pi() -> f64 {
    3.1415926
}

fn not_pi() {
    3.1415926;
}
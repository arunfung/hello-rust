use std::{fs, io};
use std::cmp::Ordering;
use std::error::Error;
use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1、猜测数字的小游戏
    // guess_the_number();

    // 2、请求一个url并将获取的HTML转成md
    html_to_md()

    // 3、函数传参
    // function_parameter();

    // 4、分号;表示其返回值为 unit
    // returned_value();

    // 5、聊天服务数据结构
    // chat_room();

    // 6、loop、while、for
    // let n = 10;
    // fib_loop(n);
    // fib_while(n);
    // fib_for(n);
    // fib_arr();
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
fn html_to_md() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = Vec::new();
    for arg in std::env::args() {
        args.push(arg);
    }
    let url = args[1].as_str();
    let output = args[2].as_str();

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;
    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);
    Ok(())
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


// 聊天服务数据结构
#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

// 聊天室中的事件
#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn chat_room() {
    let alice = User { id: UserId(1), name: "Alice".into(), gender: Gender::Female };
    let bob = User { id: UserId(2), name: "Bob".into(), gender: Gender::Male };
    let topic = Topic { id: TopicId(1), name: "rust".into(), owner: UserId(1) };
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello world!".into()));
    process_event(&event1);
    process_message(&event3);
    println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, event2, event3);
}

fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        fibonacci_sequence(&mut a, &mut b);
        i += 1;
        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);
    while i < n {
        fibonacci_sequence(&mut a, &mut b);
        i += 1;
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);
    for _i in 2..n {
        fibonacci_sequence(&mut a, &mut b);
    }
}

fn fibonacci_sequence(a: &mut i32, b: &mut i32) {
    let c = *a + *b;
    *a = *b;
    *b = c;
    println!("next val is {}", b);
}

fn fib_arr() {
    let arr = [1, 2, 3];
    assert_eq!(arr[..], [1, 2, 3]);
    assert_eq!(arr[0..=1], [1, 2]);
}

fn process_event(event: &Event) {
    match event {
        Event::Join((uid, _tid)) => println!("user {:?} joined", uid),
        Event::Leave((uid, tid)) => println!("user {:?} left {:?}", uid, tid),
        Event::Message((_, _, msg)) => println!("broadcast: {}", msg),
    }
}

fn process_message(event: &Event) {
    if let Event::Message((_, _, msg)) = event {
        println!("broadcast: {}", msg);
    }
}

#[cfg(test)]
mod tests {
    use crate::fib_arr;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        fib_arr();
    }
}
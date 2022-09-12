use std::{fs, io};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // 1、猜测数字的小游戏
    // println!("Guess the number!");

    // let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    // loop {
    //     println!("Please input your guess.");
    //
    //     let mut guess = String::new();
    //
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //
    //         let guess: u32 = match guess.trim().parse() {
    //             Ok(num) => num,
    //             Err(_) => continue,
    //         };
    //
    //     println!("You guessed: {guess}");
    //
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         },
    //     }
    // }

    // 2、请求一个url并将获取的HTML转成md
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";
    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes()).unwrap();

}

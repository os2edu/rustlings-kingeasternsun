/*
 * @Description: fix enums2
 * @Version: 2.0
 * @Author: kingeasternsun
 * @Date: 2022-11-07 03:42:40
 * @LastEditors: kingeasternsun
 * @LastEditTime: 2022-11-07 09:35:45
 * @FilePath: /rustlings-kingeasternsun/exercises/enums/enums2.rs
 */
// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.

#[derive(Debug)]
enum Message {
    // define the different variants used below
    Move {x:i32,y:i32},
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}

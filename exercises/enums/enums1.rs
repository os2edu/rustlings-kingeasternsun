/*
 * @Description: fix enums1
 * @Version: 2.0
 * @Author: kingeasternsun
 * @Date: 2022-11-07 03:42:40
 * @LastEditors: kingeasternsun
 * @LastEditTime: 2022-11-07 09:29:55
 * @FilePath: /rustlings-kingeasternsun/exercises/enums/enums1.rs
 */
// enums1.rs
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    // define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}

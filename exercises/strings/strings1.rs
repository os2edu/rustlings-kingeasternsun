/*
 * @Description: fix strings1
 * @Version: 2.0
 * @Author: kingeasternsun
 * @Date: 2022-11-07 03:42:40
 * @LastEditors: kingeasternsun
 * @LastEditTime: 2022-11-07 09:42:21
 * @FilePath: /rustlings-kingeasternsun/exercises/strings/strings1.rs
 */
// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a hint.


fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue".to_string()
}

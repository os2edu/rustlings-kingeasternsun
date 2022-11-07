/*
 * @Description: fix strings2
 * @Version: 2.0
 * @Author: kingeasternsun
 * @Date: 2022-11-07 03:42:40
 * @LastEditors: kingeasternsun
 * @LastEditTime: 2022-11-07 09:43:43
 * @FilePath: /rustlings-kingeasternsun/exercises/strings/strings2.rs
 */
// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a hint.

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

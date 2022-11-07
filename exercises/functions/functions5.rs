/*
 * @Description: fix fn5
 * @Version: 2.0
 * @Author: kingeasternsun
 * @Date: 2022-11-07 03:42:40
 * @LastEditors: kingeasternsun
 * @LastEditTime: 2022-11-07 06:43:34
 * @FilePath: /rustlings-kingeasternsun/exercises/functions/functions5.rs
 */
// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}

/*
 * @Description: fix fn3
 * @Version: 2.0
 * @Author: kingeasternsun
 * @Date: 2022-11-07 03:42:40
 * @LastEditors: kingeasternsun
 * @LastEditTime: 2022-11-07 06:41:19
 * @FilePath: /rustlings-kingeasternsun/exercises/functions/functions3.rs
 */
// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.

fn main() {
    call_me(9);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

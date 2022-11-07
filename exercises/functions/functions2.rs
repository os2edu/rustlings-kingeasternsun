/*
 * @Description: fix func2
 * @Version: 2.0
 * @Author: kingeasternsun
 * @Date: 2022-11-07 03:42:40
 * @LastEditors: kingeasternsun
 * @LastEditTime: 2022-11-07 06:38:52
 * @FilePath: /rustlings-kingeasternsun/exercises/functions/functions2.rs
 */
// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.

fn main() {
    call_me(3);
}

fn call_me(num:i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

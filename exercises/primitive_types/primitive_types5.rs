/*
 * @Description: fix pt5
 * @Version: 2.0
 * @Author: kingeasternsun
 * @Date: 2022-11-07 03:42:40
 * @LastEditors: kingeasternsun
 * @LastEditTime: 2022-11-07 07:39:26
 * @FilePath: /rustlings-kingeasternsun/exercises/primitive_types/primitive_types5.rs
 */
// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age)= cat;

    println!("{} is {} years old.", name, age);
}

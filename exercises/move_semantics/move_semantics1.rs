/*
 * @Description: fix move1
 * @Version: 2.0
 * @Author: kingeasternsun
 * @Date: 2022-11-07 03:42:40
 * @LastEditors: kingeasternsun
 * @LastEditTime: 2022-11-07 08:28:13
 * @FilePath: /rustlings-kingeasternsun/exercises/move_semantics/move_semantics1.rs
 */
// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.

fn main() {
    let vec0 =vec![1,2,3];

    println!("{:p}",&vec0[0]);
    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    println!("{:p}",&vec[0]);
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

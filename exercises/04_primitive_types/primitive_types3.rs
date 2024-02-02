// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.



fn main() {
    // 使用数组宏来声明并初始化一个包含从1到100的整数的数组
let a: [i32; 100] = [1; 100].map(|x| x + 1);

// 或者，更简单的方法是使用 `..` 运算符来生成范围，并转换为数组
let array: [i32; 100] = (1..=100).collect::<Vec<i32>>().try_into().unwrap();

    // let a = [0..988];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}

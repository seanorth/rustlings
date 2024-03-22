// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.

use std::mem::swap;

macro_rules! abacus {
    ((- $($moves:tt)*) -> (+ $($count:tt)*)) => {
        {
            println!("{} [-]{} | [+]{}", "-+1", stringify!($($moves)*), stringify!($($count)*));
            abacus!(($($moves)*) -> ($($count)*))
        }
    };
    ((- $($moves:tt)*) -> ($($count:tt)*)) => {
        {
            println!("{} [-]{} | - {}", "- 2", stringify!($($moves)*), stringify!($($count)*));
            abacus!(($($moves)*) -> (- $($count)*))
        }
    };
    ((+ $($moves:tt)*) -> (- $($count:tt)*)) => {
        {
            println!("{} [+]{} | [-]{}", "+-3", stringify!($($moves)*), stringify!($($count)*));
            abacus!(($($moves)*) -> ($($count)*))
        }
    };
    ((+ $($moves:tt)*) -> ($($count:tt)*)) => {
        {
            println!("{} [+]{} | + {}", "+ 4", stringify!($($moves)*), stringify!($($count)*));
            abacus!(($($moves)*) -> (+ $($count)*))
        }
    };

    (() -> ()) => {0};
    (() -> (- $($count:tt)*)) => {{-1 + abacus!(() -> ($($count)*)) }};
    (() -> (+ $($count:tt)*)) => {{1 + abacus!(() -> ($($count)*)) }};
}


#[allow(unused_variables, unused_assignments,clippy::let_unit_value)]
#[deny(clippy::panicking_unwrap)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // my_option.unwrap();
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;
    swap(& mut value_a,& mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);

    println!("算盘游戏：左边与右边异号时抵消；非异号时，把左边的符号转移到右边；左边无符号时，游戏结束，计算右边得分");
    println!("图示注解：左右符号消耗情况，分支编号，[消失的符号] 左边情况 | [消失的符号] 右边情况\n");

    println!("计数结果：{}\n", abacus!((++-+-+) -> (--+-+-)));
    println!("计数结果：{}\n", abacus!((++-+-+) -> (++-+-+)));
    println!("计数结果：{}\n", abacus!((---+) -> ()));
    println!("计数结果：{}\n", abacus!((++-+-+) -> ()));
    println!("计数结果：{}\n", abacus!((++-+-+++--++---++----+) -> ())); // 这是作者给的例子 :)
}

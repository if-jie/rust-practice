/*
 * @Author: 1454164915@qq.com 1454164915@qq.com
 * @Date: 2024-11-01 16:10:46
 * @LastEditors: 1454164915@qq.com 1454164915@qq.com
 * @LastEditTime: 2024-11-01 16:22:39
 * @FilePath: /learn/rust/rust-practice/keyword/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

fn add_one(i: i32) -> i32{
    i+1
}


 fn main() {

    let i: i32 = 6;
    if i == 5{
        println!("timing");
    }else{
        println!("3")
    }


    match i {
        5 => println!("5"),
        6 => println!("6"),
        _ => println!("0"),
    }

    // loop
    for ii in 0..10{
        println!("1")
    }

    let mut aa = 0;
    while aa<10 {
        println!("aa");
        aa += 1;
    }
    println!("Hello, world!");
}

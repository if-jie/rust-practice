/*
 * @Author: 1454164915@qq.com 1454164915@qq.com
 * @Date: 2024-11-03 23:29:28
 * @LastEditors: 1454164915@qq.com 1454164915@qq.com
 * @LastEditTime: 2024-11-03 23:34:23
 * @FilePath: /learn/rust/rust-practice/ligic/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
struct mystruct{}

impl Drop for mystruct{
    fn drop(&mut self){
        println!("I was dropped");
    }
}

fn dosome(){
    let a = mystruct{};
    println!("do something");

}






fn main() {
    dosome();
    // println!("Hello, world!");
}

#![allow(overflowing_literals)]
mod casting;
mod two_sum;

fn main() {
    casting::casting_types();
    let nums = vec![2, 7, 11, 15];
    println!("nums:{:?}",nums);
    let target = 17;
    let result = two_sum::two_sum(nums, target);
    println!("{:?}", result); // 输出: [0, 1]
    casting::literals();
    casting::inference();
    casting::aliasing();

}

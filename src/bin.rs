extern crate ad_lib;

use ad_lib::*;

fn main() {
    let test = from_vec(vec![1,2,3,4,5,6,7]);
    let test2 = test.set(4, &202);
    let test3 = test.append(&16);
    let test4 = test.concat(&test);

    println!("{:?}", test.flatten());
    println!("{:?}", test2.flatten());
    println!("{:?}", test3.flatten());
    println!("{:?}", test4.flatten());
}

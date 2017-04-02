fn main() {
    println!("Hello, world!");

    let i = 1;
    let i2 = i;

    println!("iii = {}", i);

    let v = vec![1, 2, 3];
    let v = take(v);
    // let mut v2 = v;

    // v2.truncate(2);

    println!("v[0] is {}", v[0]);
    
}
fn take(v: Vec<i32>) -> Vec<i32> {
    // println!("v[0] is {}", v[0]);
    v
}

fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    unimplemented!();
}
#[derive(Debug)]
struct O {
    val: i32,
}

fn main() {
    let o0 = O { val: 0 };
    let o1 = O { val: 1 };
    let o2 = O { val: 2 };
    let o3 = O { val: 3 };
    let v = vec![o0, o1, o2, o3];

    for o in v.iter() {
        println!("{:?}", *o);
    }
}

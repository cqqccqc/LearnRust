fn main() {

    // let a1 = 5;
    // let a2:i32 = 5;
    
    // assert_eq!(a1, a2);

    // let b1:u32 = 5;
    // // assert_eq!(a1, b1);

    // let mut a: f64 = 1.0;
    // let b = 2.0f32;
    // a = 2.0;

    // let a = a;
    // // a = 3.0;
    // print!("{:?}", a);

    // let (a, mut b): (bool, bool) = (true, false);
    // println!("a = {:?}, b = {:?}", a, b);

    // b = true;
    // assert_eq!(a, b);

    let t = true;
    let f: bool = false;

    let c = 'c';

    let x = 42;
    let y: u32 = 123_456;
    let z: f64 = 1.32e+2;
    let zero = z.abs_sub(123.4);
    let bin = 0b111_0000;
    let oct = 0o7320_1546;
    let hex = 0xf23a_b049;

    let str = "Hello, world!";
    let mut string = str.to_string();

    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4];

    println!("{:?}", middle);
    let mut ten_zeross: [u64; 10] = [0; 10];

    // raw printers
    let p:i32 = 5;
    let raw = &p as *const i32;
    println!("{:?}", raw);
}

fn foo(x: i32) -> i32 {
    x
}
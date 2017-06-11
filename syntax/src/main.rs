fn main() {
    // let (x, y) = (1, 2);
    // println!("{}, {}", x, y);


    // fn add_one(i: i32) -> i32 {
    //     i + 1
    // }

    // let plus_one: fn(i32) -> i32 = add_one;

    // let v1 = vec![1, 2, 3];
    // let v2 = vec![1, 2, 3];
    // let (v1, v2, answer) = foo(v1, v2);

    // // Don't worry if you don't understand how `fold` works, the point here is that an immutable reference is borrowed.
    // fn sum_vec(v: &Vec<i32>) -> i32 {
    //     return v.iter().fold(0, |a, &b| a + b);
    // }
    // // Borrow two vectors and sum them.
    // // This kind of borrowing does not allow mutation through the borrowed reference.
    // fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    //     // Do stuff with `v1` and `v2`.
    //     let s1 = sum_vec(v1);
    //     let s2 = sum_vec(v2);
    //     // Return the answer.
    //     s1 + s2
    // }

    // let v1 = vec![1, 2, 3];
    // let v2 = vec![4, 5, 6];

    // let answer = foo(&v1, &v2);
    // println!("{}", answer);

    // let mut x = 5;
    // // {
    // //     let y = &mut x;
    // //     *y += 1;
    // // }
    // let y = &x;

    // // *y += 1;

    // println!("{}", x);

//     let line = "lang:en=Hello World!";
// let lang = "en";

// let v;
// {
//     let p = format!("lang:{}=", lang);  // -+ `p` comes into scope.
//     v = skip_prefix(line, p.as_str());  //  |
// }                                       // -+ `p` goes out of scope.
// println!("{}", v);

    let y = &5;
    let f = Foo { x: y};

    println!("{}", f.x);
}

struct Foo<'a> {
    x: &'a i32,
}

impl <'a> Foo<'a> {
     fn x(&self) -> &'a i32 { self.x }
}


// fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
//     (v1, v2, 12)
// }

fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
    // ...
    line
}


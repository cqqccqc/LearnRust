fn main() {
    // let mut point = Point{ x: 0, y: 0 };

    // point.x = 1;

    // let point = point;

    // println!("The point is at ({}, {})", point.x, point.y);

    // let inch = Inchies(1);

    // println!("Inchie {}", inch.0);

    // let x:Message = Message::Move{ x: 3, y: 4};

    // let x = 1;

    // match x {
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     3 => println!("three"),
    //     _ => println!("anything"),
    // }

let x = 1;

match x {
    y => println!("x: {} y: {}", x, y),
    // _ => println!("anything"), // this causes an error as it is unreachable
}


}

struct Point {
    x: i32,
    y: i32,
}

struct Inchies(i32);


#[derive(Debug)]
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String)
}
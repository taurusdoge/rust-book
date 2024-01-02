fn main() {
    let x = five(1);

    println!("X is: {x}");

    let y = {
        let x = 3;
        x + 1
    };

    println!("Y is: {y}");
}

fn five(x: i32) -> i32 {
    x + 1
}

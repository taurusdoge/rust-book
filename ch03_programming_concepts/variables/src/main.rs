fn main() {
    // let x = 5;

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // let five_hundred = tup.0;

    // let a: [i32; 5] = [1, 2, 3, 4, 5];

    let y = {
        let x = 3;
        x + 1
    };

    println!("y value is: {y}");
}

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    if number != 0 {
        println!("The number should be zero");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The number is {number}");

    let mut i = 1;
    let was_broken = loop {
        if i > 3 {
            break true;
        }
        println!("in the loop, iteration, {i}");
        i += 1;
    };
    println!("The loop was broken: {was_broken}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let a = [10, 20, 30, 40, 50];
    let mut i = 0;
    while i < 5 {
        println!("Array item {i} is {}", a[i]);
        i += 1;
    }

    for elem in a {
        println!("Elem is: {elem}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
}

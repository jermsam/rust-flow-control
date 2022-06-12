fn main() {
    println!("Hello, world!");
    conditional();

    let conditional = false;
    // Because if is an expression, we can use it on the right side of a let statement
    // to assign the outcome to a variable
    let number = if conditional { 5 } else { 6 };
    println!("The value of number is: {}", number);

    count_to(10);
    println!("__________________");
    counting_up(10);
    println!("__________________");
    return_fro_loop(10);
    println!("__________________");
    count_to_while(10);
    println!("__________________");
    read_array([5,7,0,4,6,2]);
    println!("__________________");
    count_fro_for(10)
}

fn conditional() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn count_to(max_count: u32){
    let mut count = 0;
    loop {
        println!("{}", count);
        if count < max_count {
            count += 1;
            continue;
        } else {
            break;
        }
    }
}

fn counting_up(max_count: u32){
    let mut count = 0;
    'counting_up : loop {
        println!("count = {}", count);
        let mut remaining = max_count;
        loop {
            println!("remaining = {}", remaining);
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
    println!("End count = {}", count);
}

fn return_fro_loop(max_count: u32){
    let mut count = 0;
    let result = loop {
        if count < max_count {
            count += 1;
            continue;
        } else {
            break count;
        }
    };
    println!("Loop breaks at: {}",result);
}

fn count_to_while(max_count: u32){
    let mut count = 0;
    while count <= max_count {
        println!("{}", count);
            count += 1;

        }
}

fn read_array(array:[u32;6]){
    for el in array {
        println!("{}", el);
    }
}

fn count_fro_for(max_count: u32){
    for count in (1..=max_count).rev() {
        println!("{}", count);
    }
}
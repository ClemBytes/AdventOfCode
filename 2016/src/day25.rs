#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY25 -------");
    day25();
}

/*
fn execute_code(init_a: i32){
    let mut a = init_a;
    let mut d = a;
    let mut c = 4;
    let mut b = 633;
    // inc d
    // dec b
    // jnz b -2 <=> (d += 1) × 633 <=> d += 633;
    // dec c
    // jnz c -5 (back to b = 633) <=> (d += 633) × 4 <=> d += 633*4
    d += 633 * 4; // Here, we have d = init_a + 633 * 4;
    a = d; // So a = init_a + 633 * 4;
    // jnz 0 0 is useless
    b = a; // So b = init_a + 633 * 4;
    // a = 0;
    // while b > 0 {
    //     // c = 2;
    //     // while c != 0 {
    //     //     b -= 1;
    //     //     c -= 1;
    //     // } // <=> b -= 2;
    //     // a += 1;
    //     b -= 2;
    //     a += 1;
    // } // <=> a = b % 2; and b = b % 2;
    a = b / 2; // <=> a = (init_a + 633 * 4) / 2;
    b = b % 2; // <=> b = (init_a + 633 * 4) % 2;
    while c != 0 {
        b -= 1;
        c -= 1;
    } // <=> b -= c;
    println!("{b}");
}
*/

/*
fn execute_code(init_a: i32) -> String {
    let d = init_a + 633 * 4;
    let mut a = init_a;
    let mut s = String::from(&format!("{init_a}: ").to_string());
    loop {
        a = d;
        while a != 0 {
            let b = a % 2;
            a = a / 2;
            s += &format!("{b}").to_string();
        }
    }
}
So in fact this function prints the binary representation of (init_a + 2532) over and over
So we are looking for the first number >= 2532 such that it's binary representation is '1010...10'.
And 2532 in binary is '100111100100', so we need at least '10'×6
*/

fn day25() {
    // Solve puzzle
    let mut init_a = 0;
    let mut x = 2532;
    while !format!("{x:b}").starts_with("101010101010") {
        init_a += 1;
        x = 2532 + init_a;
    }
    println!("Result: {init_a}");
    assert_eq!(init_a, 198);
    println!("> DAY25 OK!!!");
}

fn main() {
    /*
    *   let (mut) x: u8 (unsigned 8-bit integer)
    *   let (mut) x: i32 (integer 32-bit)
    *   let (&mut) x: f32 (float 32-bit)
    *   let (mut) x: f64 (float 64-bit)
    *
    *   let a = 10;
    *   let b = 3.0;
    *   let c = a as f64 / b;
    *       as = casting (static_cast<double>)
    *   * otherwise leads to undefined
    */
    /*  
    * println!("x is {}",x); to print to the console
    *
    */


    // formatting 
    let a = 10.0;
    let b = 3.0;
    let c = a/b;
    println!("c is {0:08.3}\na is {1}",c,a);
    
    let mut value = 0b1111_0101u8;

    println!("value is {}",value);
    println!("value is {:08b}",value);

    let aa = true;
    let bb = false;
    let c = (aa ^ bb) || panic!();
    print!("{}",c);
    // panic causes termination with an error
    value = !value;
    println!("value is {:08b}",value);


    // not pase new line after print, use just print!(); 

    let letter = 'a';
    let number = '1';
    
    let finger = '\u{261D}';

    print!("{}\n{}\n{}\n",letter,number,finger);

    // challenge
    let a:f64 = 13.0;
    let b:f64 = 2.3;
    let c: f64 = 120.0;

    let average: f64 = (a+b+c)/3.0;

    assert_eq!(average,45.1);
    println!("Test passed!");

    let mut letters = ['a','b','c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first_letter is {}",first_letter);

    let numbers: [i64; 5];
    numbers = [0;5];
    let index = numbers.len()-1;
    println!("{}",numbers[index]);    
    //let lots = [[1,2,3],
    //                           [4,5,6]];
    // tuples  (optional)
    let mut stuff: (u8,f64,char) = (10,3.14,'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first_item is {}",first_item);

   // let (a, b, c) = stuff;
    // like in c++ binding declaration
    // int a[2] = {1,2};
    // auto [x,y] = a;
    say_hello();
    let a = say_hello;
    a();
}


fn say_hello() {
    println!("Hello!");
    say_number(25);
}

fn say_number(number: i32)
{
    println!("number is {}",number);
}

fn square(x: i32) -> i32 {
    x*x
}

fn celsius_to_fahrenheit(celsius: f64) -> f64
{
    (1.8 * celsius) + 32.0   
}

/*
*   cargo run - to run the program 
*   cargo build --release - to build release
*/
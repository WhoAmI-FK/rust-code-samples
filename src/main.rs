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

    /*
    let make_x_odd = true;
    let x;

    if make_x_odd {
        x = 1;
    } else {
        // if you try to comment it, it wouldn't compile Error: use of possibly-uninitialized further in println!
        x = 2;
    }

    // we can use

    let x = if make_x_odd {1} else {2};

    */

    let mut count = 0;
    let result = loop {
        if count == 1999 {
            break count * 10;
        }
        count+=1;
        println!("count is {}",count);
    };
    println!("After loop count is {}",result);

    count = 0;
    let letters = ['a','b','c'];
    while count < letters.len() {
        println!("letter is {}",letters[count]);
        count+=1;
    }

    let message = ['h','e','l','l','o'];

    for item in message.iter().enumerate() {
        print!("{}",item.1);
    }



    let mut mean: f64 = 0.0;
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut min:i64 = numbers[0];
    let mut max:i64 = numbers[0];
    for &iter in numbers.iter() {
        min = if iter < min { iter } else { min };
        max = if iter > max { iter } else { max };
        mean+=iter as f64; 
    }
    mean = mean / numbers.len() as f64;
    assert_eq!(max,56);
    assert_eq!(min,-18);
    assert_eq!(mean, 12.5);
    println!("\nTests passed!");

    /*
     Shadowing

     let planet = "Earth";
     let planet = "Mars";
     
     is valid expression
    */
    let mut messg = String::from("Earth");
    messg.push_str(" is home.");
    println!("{}",messg);


    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        outer_planet = inner_planet.clone(); // by default moves
        println!("Inner planet is {}",inner_planet);
        inner_planet.clear();
    }
    println!("outer_planet is {}",outer_planet);

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
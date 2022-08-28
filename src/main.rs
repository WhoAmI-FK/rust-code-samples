use std::io;
use rand::prelude::*;
use std::env;
use std::fs;
use std::io::prelude::*;

#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U
}

impl <T,U> Rectangle<T,U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn new(name: &str) -> Shuttle {
        Shuttle { name: String::from(name), crew_size: (7), propellant: (0.0) }
    }
}



struct Color(u8, u8, u8); // RGB

struct Point(u8, u8, u8); // XYZ

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


    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel(&mut rocket_fuel);

    println!("rocket_fuels is {} and length is {}",rocket_fuel,length);
    
    let greetings = String::from("Greetings from Earth!");
    println!("{}",greetings);

    let last_word = &greetings[15..15+6];
    println!("Slice: {}",last_word);

    let msg_f = String::from("Greetings from Earth!");
    let first_word = get_first_word(&msg_f);
    println!("first_word is {}",first_word);

    let mut buffer = String::new();
    println!("Enter a message: ");
    io::stdin().read_line(&mut buffer);
    println!("buffer is {}",buffer);

    let num: i32 = buffer.trim().parse().unwrap();
    println!("number + 1 is {}",num + 1);

    let nnum = rand::random::<f64>();
    println!("number is {}",nnum);
    println!("number is {}",thread_rng().gen_range(1..11));

    
    let secret_number = rand::thread_rng().gen_range(1..101);
    let bol = false;
   if bol {
    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number: ");
    loop{
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input line.");
        let guess: u32 = guess.trim().parse().expect("Failed to parse the guess.");
        
        if guess > secret_number {
            println!("\n{} is too high! Guess Lower:",guess);
        }else if guess < secret_number {
            println!("\n{} is too low! Guess higher:",guess);
        }
        else{
            println!("\nYou got it! The secret number was {}.",secret_number);
            break;
        }
    }
    }
     for(index,argument) in env::args().enumerate() {
        println!("argument {} is {}", index,argument);
    }

    if env::args().len() > 2 {
    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);
    }

    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("contents is {}", contents);

    for line in contents.lines() {
        println!("{}",line);
    }

    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.");

    fs::write("speech.txt", speech);


    // to append
//    let mut file = fs::OpenOptions::new().append(true).open("planets.txt").unwrap();

//    file.write(b"\nPluto");
    
    if isNameInFile() {
        println!("YES!");
    }


    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0
    };

    let vehicle2 =  Shuttle {
        ..vehicle.clone()
    };

    println!("name {}",vehicle.name);
    vehicle.name = String::from("Atlantis");

    let red = Color(255, 0, 0);
    println!("{}",red.0);


    let rect = Rectangle {
        width: 1.2,
        height: 3.4
    };
    println!("rect is {:?}",rect);

}


fn isNameInFile() -> bool {
    if env::args().len() < 1 {
        return false;
    }
    let Name = env::args().nth(1).unwrap();
    let contents: String = fs::read_to_string("moonwalkers.txt").unwrap();
    for name in contents.lines() {
        if !name.contains(" ") {
            if name == Name.as_str() {
                return true;
            }
        }
    } 
    false
}

fn process_fuel(propellant: &mut String) -> usize
{
    println!("Processing propellant {}...",propellant);
    propellant.push_str(" is highly flammable!");
    let length = propellant.len();
    length
}

fn produce_fuel() -> String{
    let new_fuel = String::from("RP-1");
    new_fuel
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

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for(index,&item) in bytes.iter().enumerate() {
        if item == b' ' || item == b',' || item == b'!' || item == b'?'|| item == b'.'
        {
            return &s[..index]; 
        }
    }

    &s
}


fn trim_spaces(s: &str) -> &str {
    let mut start = 0;
    for(index, character) in s.chars().enumerate() {
        if character != ' ' {
            start = index;
             break;
        }
    }


    let mut end = 0;
    for (index, character) in s.chars().rev().enumerate() {
        if character != ' ' {
            end = s.len() - index;
            break;
        }
    }

    &s[start..end]

}


/*
*   cargo run - to run the program 
*   cargo build --release - to build release
*/
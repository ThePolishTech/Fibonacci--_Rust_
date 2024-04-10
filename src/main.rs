//  ----====  IMPORTS  ==----  \\
use std::io;
use std::cmp::Ordering;

//  ----====  FUNCTIONS  ==----  \\
fn fibonacci(fib_number: u32) -> u128 {

    if fib_number == 0 || fib_number == 1 {  // Needs to be hard coded
        return 1;
    }
    // else ↴

    // Variables
    let mut register_a: u128 = 0;
    let mut register_b: u128 = 1;
    let mut register_c: u128 = 0;  // Value here doesn't matter, as long  ¬
                                   //     as it is within the type's limits

    // iteration
    for _i in 0..fib_number {
        register_c = register_a
                   + register_b; 
    
        register_a = register_b;  // Moves the proverbial 'head'  ¬
        register_b = register_c;  //    along to the next possition
    }
    

    return register_c;
}


//  ----====  Main  ==----  \\
fn main() {
    println!("Fibonacci Number Generator\n\tTo quit type \"quit\"\n");
    let mut input: String = String::new();


    loop {
        // Clear then read input
            input = String::from("");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed To Read Line"); // This has yet to happen
            
        // If quit entered, quit
            if input.trim() == String::from("quit") {
                break;
            }

        // Parse
            let input: u32 = 
            match input.trim().parse() {
                Ok(num)  => num,
                Err(_)        => {println!("    Please Type a Number"); continue}
            };

        // Make sure input is smaller then 186
            match input.cmp(&186) {
                Ordering::Less     => { println!(  "    Fibonacci Number: {:?}", fibonacci(input)  ) },
                Ordering::Greater  => { println!(  "    Too big! Please Enter a Value Bellow 186"  )  },
                Ordering::Equal    => { println!(  "    Too big! Please Enter a Value Bellow 186"  )  }
            }
    }


} // <<< fn main()

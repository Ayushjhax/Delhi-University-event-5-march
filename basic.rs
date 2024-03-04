fn main() {
    println!("Hello World");
}

fn main(){
    let lucky_number = 7; // This is my lucky number
}

//variables
fn main(){
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

if else
fn main() {
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

let
fn main() {
    let condition = true;
    let number = if condition { "5" } else { "six" };

    println!("The value of number is: {}", number);
}

//loop
fn main(){
    loop{
        println!("again!");
    }
}

//function
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("ANDC"); // Call the greet function 
}




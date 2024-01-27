fn main() {
    let age=16;
    if age>18{
        println!("you are allowed to vote in elections!");
    }
    else {
        println!("you are NOT allowed to vote in elections!");
    }



// Note : Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean.
// You must be explicit and always provide if with a Boolean as its condition. 


// Handling Multiple conditions
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

  // Using If with let as Expression

  let condition = true;
  let number = if condition { 5 } else { 6 };

  println!("The value of number is: {number}");

  //Loop
  
    loop {
        println!("again!");
        break;
    
}
     //While 
     let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // Better Alternative of above code to iterate over an array
    
    
        for element in a {
            println!("the value is: {element}");
        }

        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
   

}
    

fn main() {
    println!("Hello, world!");
    another_function();
    is_even_odd(12);
    is_even_odd(199);
    let x = plus_one(110);

    println!("The value of x is: {x}");

}


fn another_function(){
    println!("Hello , from another function!");
}

fn is_even_odd(x:i32){
    if x%2==0{
        println!("{} is Even", x);
    }
    else{
        println!("{} is Odd", x);

    }

}

// Function Expressions
// Statements do not return values  but function Expressions do.



fn plus_one(x:i32)->i32 {
   
    x + 1
}
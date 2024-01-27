fn main() {

    // Variables in Rust are immutable by default.
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //  CONSTANT 
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3hr = {} seconds", THREE_HOURS_IN_SECONDS);

    /* NOTES:
    => You cannot use mut keyword with const variable bcoz 
    Constants aren’t just immutable by default—they’re always immutable. 

    => Constants can be declared in any scope, including the global scope, 
    which makes them useful for values that many parts of code need to know about.
    
    => Constants may be set only to a constant expression,
     not the result of a value that could only be computed at runtime.

     */

    

    //SHADOWING 

    /*
    => Shadowing is different from marking a variable as mut because we’ll get a compile-time error 
    if we accidentally try to reassign to this variable without using the let keyword. 
     */

    let y =10;
    let y=y+2;

    {
        let y=y*2;
        println!("The value of y is {}", y);
    }
    println!("The value of y is {}", y);

    /*
    The other difference between mut and shadowing is that because we’re effectively creating a new variable
     when we use the let keyword again, we can not only change the type of the value but reuse the same name. 
     */
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces :{}",spaces);

    // Note : we’re not allowed to mutate a variable’s type, but  this can be done via shadowing.


    
}
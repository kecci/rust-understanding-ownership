fn box_value(value: Box<u8>) {
    println!("New ownership is here {}", value);
}

fn box_value_as_variable(value: Box<u8>) -> Box<u8>{
    println!("Value is {}", value);

    value
}

fn main() {
    // stack-allocated integer variable
    let number = 30;
    // we're copying value of 'number' variable to 'number2'. we're not moving it
    let number2 = number;
    println!("Number {} and Number2 {}", number, number2);
    /* 
        This will work because these are stack-allocated. 
        Primitives like i32, u64, f64, or bool are small types. 
        This means, copying them is fast and they don’t need to too much size in memory. 
    */

    
    let number = Box::new(30);
    println!("Number {}", number);
    let number2 = number;
    // println!("Number {} and Number2 {}", number, number2);
    /* 
        This won't work. It will throw an error like the use of moved. 
        Why? As we mentioned before, variable ownership transferred to a function or a macro. 
        In this case println! takes ownership of the number variable.
     */
    println!("Number2 {}", number2);


    let a_variable = Box::new(30);
    box_value(a_variable);
    // won't work
    // println!("Value is {}", a_variable);
    /* 
        Because a_variable doesn't exist in this scope anymore. 
        It doesn't point to the address you know before. 
        Because we moved the resource of a_variable.
    */

    
    /* What To Do Solve Ownership Problems? */
    
    // 1-) Create Scope Only Variables
    let a_variable = Box::new(30);
    {
        let a_variable = Box::new(30);
        box_value(a_variable);
    }
    println!("A variable {}", a_variable);

    // 2-) Clone Variable
    let a_variable = Box::new(30);
    println!("A variable {}", a_variable.clone());
    println!("A variable2 {}", a_variable);

    // 3-) Returning Variables with Functions
    let a_variable = Box::new(30);
    let a_variable = box_value_as_variable(a_variable);
    println!("A variable {}", a_variable);
    let a_variable = box_value_as_variable(a_variable);
    println!("A variable2 {}", a_variable);
}

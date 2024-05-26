fn main() {
    let age = 31;
    let name = "Jack";

    // print the variables using println!
    println!("Name = {}, Age = {}", name, age);
    prac();
    numbers();
}

fn prac() {
    let age = 31;
    let name = "Jack";

    // print the variable using println!
    println!("Name = {0}, Age = {1}", name, age);
}



fn numbers() {
    // variable to store integer value
    let age = 31;
    println!("Age: {}", age);

    // variable to store floating-point value
    let salary = 342523.23;
    println!("Salary: {}", salary);

    // variable to store string
    let name = "Jackie";
    println!("Name: {}", name);
}
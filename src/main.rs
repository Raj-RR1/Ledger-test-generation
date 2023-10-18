use core::num;
use std::io;

struct MyObject {
    index: String,
    name: String,
    blob: String,
    output: Vec<String>,
    output_expert: Vec<String>,
}
fn main() {
    //let mut objects = Vec::new();
    let mut num_arguments = String::new();
    println!("Enter the number of arguments the extrinsic has"); 
    io::stdin().read_line(&mut num_arguments).expect("Failed to read the number of arguments");
    let num_arguments = num_arguments.trim().parse::<usize>().expect("Not able to parse num_arguments");
    //println!("{}",num_arguments);

    let mut argument_names = Vec::new();
    for i in 0..num_arguments{
      let mut name = String::new();
      println!("Enter the name of argument {}:",i+1); 
      io::stdin().read_line(&mut name).expect("Failed to read argument name");
      let name=name.trim().to_string();
      //println!("{}",name);
      argument_names.push(name);
    }

}

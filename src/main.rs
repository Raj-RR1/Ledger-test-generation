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
    println!("Enter the number of arguments the extrinsic has");
    let mut num_arguments = String::new();
    io::stdin().read_line(&mut num_arguments).expect("Failed to read the number of arguments");
    let num_arguments = num_arguments.trim().parse::<usize>().expect("Not able to parse num_arguments");

    let mut argument_names = Vec::new();

}

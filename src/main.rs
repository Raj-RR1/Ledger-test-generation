use std::io;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyObject {
    index: String,
    name: String,
    blob: String,
    output: Vec<String>,
    //output_expert: Vec<String>,
}

fn generate_output_argument(index:usize,argument_name:&String, argument_value:&String)-> Vec<String>{

let mut output = Vec::new();
let mut part_number = 1;
let mut remaining_value = argument_value.clone();

let total_parts = ((argument_value.len()-1)/38) + 1;

let mut part_label;

if total_parts <= 1 {
   part_label="".to_string(); 
   output.push(format!("{} | {} {} : {}",index,argument_name,part_label,argument_value));
}
else{

  while !remaining_value.is_empty() {

    if remaining_value.len() <= 38{
        part_label = format!("[{}/{}]",part_number, total_parts);
        output.push(format!("{} | {} {} : {}",index,argument_name,part_label,remaining_value));
        break;
    }
    
    let (part, rest)= remaining_value.split_at(38);
    //remaining_value=rest.to_string();
    
    part_label = format!("[{}/{}]",part_number, total_parts);

    // let part_label = if total_parts > 1{
    //   format!("[{}/{}]",part_number, total_parts)
    // }
    // else{
    //   "".to_string()
    // };

    output.push(format!("{} | {} {} : {}",index,argument_name,part_label,part));
    part_number+=1;
    remaining_value=rest.to_string();
}

}

   output

}
fn main() {
    let mut objects = Vec::new();
    let mut num_arguments = String::new();
    println!("Enter the number of arguments the extrinsic has"); 
    io::stdin().read_line(&mut num_arguments).expect("Failed to read the number of arguments");
    let num_arguments = num_arguments.trim().parse::<usize>().expect("Not able to parse num_arguments");
    //println!("{}",num_arguments);

    for k in 1..=2{

      let mut argument_names = Vec::new();
      let mut argument_values=Vec::new();
  
      for i in 0..num_arguments{
  
        let mut name = String::new();
        println!("Enter the name of argument {}:",i+1); 
        io::stdin().read_line(&mut name).expect("Failed to read argument name");
        let name=name.trim().to_string();
        //println!("{}",name);
        argument_names.push(name);
  
        let mut value = String::new();
        println!("Enter the value of argument {}:",i+1);
        io::stdin().read_line(&mut value).expect("Failed to read line");
        let value = value.trim().to_string();
        argument_values.push(value);
      }

      let mut output=Vec::new();
     // let output_expert=Vec::new();

      output.push(format!("0 | Identity : Kill Identity"));

      for j in 0..num_arguments{

            let argument_name=&argument_names[j];
            let argument_value=&argument_values[j];

            output.extend(generate_output_argument(j+1,argument_name,argument_value));
      }

      let obj = MyObject{
        index : "".to_string(),
        name : "Identity_Kill_Identity".to_string(),
        blob : "".to_string(),
        output,
      };
      objects.push(obj);
  
    }

    let json = serde_json::to_string_pretty(&objects).unwrap();
    println!("{}", json);

}

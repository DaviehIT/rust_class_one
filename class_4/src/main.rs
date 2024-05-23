
    

    // let x = vec![1,2,3,4,5]
    // println!("{:?}",x)

    // for item in x{
    //     println!("looping through {item}")



//         let x = vec![1, 2, 3, 4, 5];

// let mut i = 0; // Initialize an iterator variable

// loop {
//   if i >= x.len() {
//     break; // Exit the loop when iterator reaches the end of the vector
//   }
//   println!("looping through {}", x[i]);
//   i += 1;
// }


// let mut x : = vec :: new();
// x.push(: 1);
// x.push(: 2);
// x.push(: 3);

//loop
//mutate each item in the vec
//print out the mutated vec and the original vector



// let date_time_string:&str="2024-05-23T12:34:56";
// //find a way to split date and time
// //print out the time


// // Destructure directly into string slices
// let (date, time) = date_time_string.splitn(2, 'T').collect::<Vec<_>>()[1];

// println!("{}", time);


//rust program
//create a struct with vectors and hashmaps
//call the function of the struct to update vector and print it out
//call a function of the struct to update hashmap and print it out





use std::collections::HashMap;

struct MyData {
  numbers: Vec<i32>,
  names: HashMap<String, i32>,
}

impl MyData {
  fn new() -> Self {
    MyData {
      numbers: vec![1, 2, 3],
      names: HashMap::new(),
    }
  }

  // Update the numbers vector
  fn update_numbers(&mut self, new_number: i32) {
    self.numbers.push(new_number);
  }

  // Update the names hashmap
  fn update_names(&mut self, name: String, age: i32) {
    self.names.insert(name, age);
  }

  // Print the numbers vector
  fn print_numbers(&self) {
    println!("Numbers: {:?}", self.numbers);
  }

  // Print the names hashmap
  fn print_names(&self) {
    println!("Names: {:?}", self.names);
  }
}

fn main() {
  let mut data = MyData::new();

  // Update the numbers vector
  data.update_numbers(4);

  // Update the names hashmap
  data.update_names("David".to_string(), 21);
  data.update_names("Joy".to_string(), 25);

  // Print the updated data
  data.print_numbers();
  data.print_names();
}

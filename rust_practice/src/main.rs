
/* 
//loops and conditions

let a: i32=3;
let b: i32=4;
 
 if a > b {
    println!("a is greater than b")

}else if a == b {

    println!("a is equal to b")

}else{
    println!("a is less than b")
}

*/


/* 
//code gets inputs from the terminal
// Task 1
//Prints out items in reverse from 50 to 1
let network_response: i32=200;
let is_success: bool =if network_response == 200 {true} else {false};
if is_success{
    println!("Operation was successful");
}
*/




/* 
let mut counter: i32=0;
loop{
    counter = counter + 1;
    println!("I am at number {counter}");

    if counter == 1{
        println!("Starting point");
        
    }
    if counter == 50 {
        break;
    }
} 

*/
 
//a while loop takes a condition









/* 
 
let mut array = [1,2,3,4,5,6,7,8,9];
let mut index: usize = 5;

while index != 0 {
    println!("Looping through index {index} item in array {}",array [index]);
    index -= 1;
    
}
println!("LIFTOFF!!!");
*/



/* 
let mut array:[i32; 9] = [1,2,3,4,5,6,7,8,9];
for element in array {

    println!("we are looping in item {element}")


}

    


println!("LIFTOFF!!!");

*/

 /* 
for item in 1..<21{

    println!("count item {item}")
}


println!("LIFTOFF!!!");
    
 */

 /* 
 let mut input_str = String::new();

 println!("Enter an integer:");

 io::stdin().read_line(&mut input_str)
     .expect("Failed to read user input");

 let input: u32 = match input_str.trim().parse() {
     Ok(num) => num,
     Err(_) => panic!("Please enter a valid integer"),
 };

 // Create an iterator from 0 to input + 1
 let mut range = 0..=input;

 // Loop through the iterator
 for counter in range {
     println!("{}", counter);
 }

*/



/* 
use std::{array, ops::{Index, IndexMut}, slice::SliceIndex};


fn sum_array(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for num in arr.iter() {
        sum += *num;
    }
    return sum;
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let result = sum_array(&numbers);
    println!("The sum of the array is: {}", result);
}
*/



fn main (){

    //code gets inputs from the terminal
// Task 1
//Prints out items in reverse from 50 to 1
let network_response: i32=200;
let is_success: bool =if network_response == 200 {true} else {false};
if is_success{
    println!("Operation was successful");
}






let mut counter: i32=0;
loop{
    counter = counter + 1;
    println!("I am at number {counter}");

    if counter == 1{
        println!("Starting point");
        
    }
    if counter == 50 {
        break;
    }
} 
}



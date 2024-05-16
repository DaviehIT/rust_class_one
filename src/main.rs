fn main() {
    

//code gets inputs from the terminal
// Task 1
//Prints out items in reverse from 50 to 1
let network_response: i32=200;
let is_success: bool =if network_response == 200 {true} else {false};
if is_success{
    println!("Operation was successful");
}






let mut counter: i32=50;
loop{
    counter = counter - 1;
    println!("I am at number {counter}");

    if counter == 50{
        println!("Starting point");
        
    }
    if counter == 1 {
        break;
    }
} 






}

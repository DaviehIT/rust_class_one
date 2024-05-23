mod directory_module;   
mod file_modules::second_item;



fn main() {
    

    
    
        //  the file module function
        let numbers = vec![1, 2, 3, 4, 5];
        match file_module::second_item(&numbers) {
            Some(item) => println!("The second item is: {}", item),
            None => println!("The slice is too short to have a second item."),
        }


        //  the directory module function
    directory_module::print_message("Hello from the directory module!");



    
    }



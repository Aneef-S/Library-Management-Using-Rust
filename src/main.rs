#![allow(warnings)]


use std::collections::HashMap;
use std::io::{self, Write};
use std::str::FromStr;
mod user;
mod book;



fn main() 
{
    let mut current_user_id: u32 = 1;
    let mut current_book_id: u32 = 1;
    let mut user_data : HashMap<u32, user::User> = HashMap::new();
    let mut book_data : HashMap<u32, book::Book> = HashMap::new();
    
    //display_options(&mut user_data, &mut current_user_id);

    book::add_new_book(&mut book_data, &current_book_id);
    book::display_book_details(&book_data);
    
}

fn display_options(user_data:&mut HashMap<u32,user::User>,current_user_id:&mut u32)
{
    let mut option:u8 = 0;
    'operation : loop
    {
        println!("You can perform the following operations in the current version - \n
        1 - Insert New User \n
        2 - Qeury User Details \n
        3 - Delete User \n
        0 - Quit
        ");

        get_input(&mut option);

        match option
        {
            1 => 
            {
                user::add_new_user(user_data, &current_user_id);
                *current_user_id += 1;
            },
            2 => user::print_user_details(user_data),
            3 => user::remove_from_data(user_data),
            0 => 
            {
                println!("Quit operation called!");
                break 'operation;
            }
            _ => println!("Operation not found!! Enter valid operation."), 
        }

    }
}


fn get_input<T>(value:&mut T)
where 
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    let mut user_input:String = String::new();
    io::stdout()
        .flush()
        .expect("Failed to flush");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read from user");

    match user_input.trim().parse::<T>()
    {
        Ok(parsed_value) => *value = parsed_value,
        Err(e) => 
        {
            println!("Error : {:?}",e);
        }
    }
                            
}



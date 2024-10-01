use std::collections::HashMap;
use std::io::{self, Write};
use std::str::FromStr;


#[derive(Debug)]
pub struct Book{
    id:u32,
    name:String,
    current_owner:u32,
    previous_owners_count:u32,
}



impl Book {
    
    pub fn new(l_id:&u32,l_name:String)->Book
    {
        Book 
        { 
            id: (*l_id), 
            name: (l_name), 
            current_owner: (0),
            previous_owners_count: 0,
        }
    }

    pub fn assign_book(&mut self,user_id:u32)
    {
        self.current_owner = user_id;
        self.previous_owners_count += 1;
    }
}

pub fn add_new_book(book_data:&mut HashMap<u32,Book>,current_book_id:&u32)
{

    println!("Enter book name: ");
    let mut book_name:String = String::new();
    get_input(&mut book_name);

    book_data.insert(*current_book_id
        ,Book::new(current_book_id
            , book_name));
}

pub fn display_book_details(book_data:&HashMap<u32,Book>)
{
    println!("{:#?}",book_data);
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
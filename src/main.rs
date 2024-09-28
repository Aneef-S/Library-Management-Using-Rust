#![allow(warnings)]


use std::collections::HashMap;
use std::io::{self, Write};
use std::str::FromStr;



fn main() 
{
    let mut current_id: u32 = 1;
    let mut user_data : HashMap<u32, User> = HashMap::new();

    
    let mut user_name:String = String::new();
    print!("Enter your name - ");
    get_input(&mut user_name);
    println!("User name is {user_name}");


   
    print!("Enter your age - ");
    let mut user_age:u32 = 0;
    get_input(&mut user_age);
    println!("User age is {user_age}");

    print!("");




    user_data.insert(current_id,
        User::new(current_id,
            user_name,
            user_age,
            Membership::READER)
    );
    
    println!("User Ages: {:?}", &user_data[&1]);
    


}

#[derive(Debug,PartialEq)]
enum Membership
{
    PREMIUM,
    READER,
    REGULAR,
}

#[derive(Debug)]
struct User
{
   
    id:u32,
    name:String,
    age:u32,
    membership:Membership,
    max_books:u8,
    current_book_count:u8

}

impl User
{
    fn new(l_id:u32,l_name:String,l_age:u32,l_memb:Membership) -> User
    {
        let mut l_book_count: u8 = 0;
        if l_memb == Membership::PREMIUM
        {
            l_book_count = 10;
        }
        else if l_memb == Membership::READER
        {
            l_book_count = 5;
        }
        else if l_memb == Membership::REGULAR
        {
            l_book_count = 3;
        }
        User
        {
            id:l_id,
            name:l_name,
            age:l_age,
            membership:l_memb,
            max_books:l_book_count,
            current_book_count:0
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



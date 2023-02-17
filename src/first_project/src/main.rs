use regex::Regex;
use std::{collections::HashMap, io::stdin};
fn user_verification(){
    let mut verify:HashMap<String,String> = HashMap::new();
    let mut user= String::new();
    let mut pswd= String::new(); 
    println!("enter username:");
    std::io::stdin().read_line(&mut user).expect("can't read username");
    user = user.trim().to_string();
    if verify.contains_key(&mut user){
        println!("user existed please enter password");
        println!("enter password:");
        std::io::stdin().read_line(&mut pswd).expect("couldnt read password");
        pswd=pswd.trim().to_string();
        if verify.get(&mut user)==Some(&mut pswd){
            println!("loginned");
        }
        else{
            println!("wrong password");
        }
    }
    else{
        println!("user not existed");
    }
}
fn user_validation(){
    let mut verify:HashMap<String,String> = HashMap::new();
    let mut user= String::new();
    let mut pswd= String::new(); 
    println!("enter username:");
    std::io::stdin().read_line(&mut user).expect("can't read username");
    user = user.trim().to_string();
    let  re:Regex = Regex::new(r"[a-zA-Z0-9._&@!*]").expect("could'nt read");
    if re.is_match(&mut user)==true{
        println!("user enrolled");         
        println!("enter password:");
        std::io::stdin().read_line(&mut pswd).expect("could'nt read");
        pswd=pswd.trim().to_string();
        if re.is_match(&mut pswd)==true{
            println!("password entered");
            verify.insert( user,pswd);
        }

    }
}
fn main(){
    let mut choice = String::new();
    println!("1:login \n 2:signup \n 3:update user \n 4: delete user 5: logout"); 
    println!("enter your choice:");
    choice= choice.trim().to_string();
    match choice.as_str(){
        "1" => { user_verification();},
        "2" => { user_validation();},
        "3" => { println!("in progress");}
        "4" => {println!("in progress");}
    }
}

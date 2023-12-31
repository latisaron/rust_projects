use std::env;
use std::fs;

fn echo(generic_vector: &[String]) {
    println!("{}", generic_vector[0]);
}

fn ls(_generic_vector: &[String]) {
    fs::read_dir("./").unwrap().for_each(|path| println!("{}", path.unwrap().path().display()));
}

fn cat(generic_vector: &[String]) {
    for index in 0..=1 {
        let file = &generic_vector[index];
        let content = fs::read_to_string(file).expect("Something went wrong!");
        println!("{}", content); 
    }
}

fn throw_error(_generic_vector: &[String]) {
    println!("Unknown command.");
}


fn main() {
    let args : Vec<String> = env::args().collect();
    let empty_string_vec: Vec<String> = Vec::new(); 
    if args.len() < 2 {
        println!("Too few arguments");
        return;
    }
    let (func_call, func_args): (fn(&[String]), &[String]) = match args[1].as_str() {
        "echo" => (echo, &args[2..]),
        "ls" => (ls, &empty_string_vec[..]),
        "cat" => (cat, &args[2..]),
        _ => (throw_error, &empty_string_vec[..]),
    };
    
    func_call(func_args);
}


// echo(string),
// cat(string, string)
// ls(-)
// find(string)
// grep(string, string)

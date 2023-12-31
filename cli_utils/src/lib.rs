use std::fs;

pub fn echo(generic_vector: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if generic_vector.len() != 1 {
        println!("Function needs 1 single argument.");
        std::process::exit(1);
    }
    println!("{}", generic_vector[0]);
    Ok(())
}

pub fn ls(_generic_vector: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    fs::read_dir("./").unwrap().for_each(|path| println!("{}", path.unwrap().path().display()));
    Ok(())
}

pub fn cat(generic_vector: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if generic_vector.len() != 2 {
        println!("Function needs 2 arguments.");
        std::process::exit(1);
    }
    for index in 0..=1 {
        let file = &generic_vector[index];
        let content = fs::read_to_string(file)?;
        println!("{}", content); 
    }
    Ok(())
}

pub fn grep(generic_vector: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if generic_vector.len() > 2 {
        println!("Function needs 2 arguments.");
        std::process::exit(1);
    }
    let query = &generic_vector[0];
    let file_path = &generic_vector[1];
    let content = fs::read_to_string(file_path)?;
    for line in content.lines() {
        if line.contains(query) {
            println!("{}", line);
        }
    }
    Ok(())
}

pub fn throw_error(_generic_vector: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    println!("Unknown command.");
    Ok(())
}

pub fn run(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let empty_string_vec: Vec<String> = Vec::new();
    let (func_call, func_args): (fn(&[String]) -> Result<(), Box<dyn std::error::Error>>, &[String]) = match args[1].as_str() {
        "echo" => (echo, &args[2..]),
        "ls" => (ls, &empty_string_vec[..]),
        "cat" => (cat, &args[2..]),
        "grep" => (grep, &args[2..]),
        _ => (throw_error, &empty_string_vec[..]),
    };
    
    func_call(func_args)
}

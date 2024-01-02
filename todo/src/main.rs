use std::io;
use std::collections::HashMap;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// false is not done
// true is done
fn main() {
    let mut input = String::new();
    let mut tasks: HashMap<String, bool> = HashMap::new();
    loop {
        input.clear();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.contains(&String::from("exit")) {
                    break;
                } else {
                    // ok so apparently if we want to have a vec of slices of strings, givne how those work, we will have
                    // to constantly fight with immutable/ mutable borrows, so chose to go with <Vec<String>> instead
                    let split_items = input.trim().split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
                    if split_items[0] == String::from("add") {
                        // in adition to this, we needed to clone the item, or send a reference to it (which we do not want to manage)
                        // so that it doesn't get consumed by the insert - i wonder why the insert doesn't by default require a reference
                        // and then dereference it inside?
                        split_items[1..].iter().for_each(|item| {tasks.insert(item.clone(), false); });
                    } else if split_items[0] == String::from("finish") {
                        split_items[1..].iter().for_each(|item| {
                            let item_clone = item.clone();
                            if tasks.contains_key(&item_clone) {
                                tasks.remove(&item_clone);
                                // here we can let insert consume the item_clone as we dont use it anymore
                                tasks.insert(item_clone, true);
                            } else {
                                println!("{} is not present as a task.", item_clone);
                            }
                        })
                    } else if split_items[0] == String::from("show") {
                        // realistically could've used an Enum here that implements the Display trait but cant be arsed.
                        for (key, val) in &tasks {
                            let output = if *val {
                                "done."
                            } else {
                                "not done."
                            };
                            println!("{} is {}\n", key, output);
                        }
                    } else {
                        println!("You can either add or finish tasks, nothing else");
                    }
                    // realistically could've used an Enum here that implements the Display trait but cant be arsed
                }
            },
            Err(_) => println!("Something went wrong."),
        }
    }
}





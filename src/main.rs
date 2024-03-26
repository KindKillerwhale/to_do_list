use std::io;

struct ToDo {
    list : String,
    check_flag : bool,
}

impl ToDo {
    fn new_list (list : String) -> ToDo {
        ToDo {
            list,
            check_flag : false,
        }
    }

    fn change_check_flag(&mut self) {
        self.check_flag = !self.check_flag;
    }

    fn check_icon(&self) -> &str {
        if self.check_flag {
            "[ ☑️ ]"
        } else {
            "[ ☐ ]"
        }
    }

    fn edit(&mut self, new_list: String) {
        self.list = new_list;
    }
}


fn main() {
    let mut to_do_list : Vec<ToDo> = Vec::new();

    loop {
        menu();
        
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Enter the ToDo: ");
                let mut todo = String::new();
                io::stdin()
                    .read_line(&mut todo)
                    .expect("Failed to read line");
                    
                to_do_list.push(ToDo::new_list(todo.trim().to_string()));

                // need to check push
                // let len = to_do_list.length();
                
                println!("ToDo added successfully!!");
                
                //if todo.trim() == to_do_list.last().expect("Failed to retrieve last ToDo").as_str() 

                //else {
                //    println!("Failed to add ToDo: {}", todo.trim().to_string());
                //}
            },

            "2" => {
                view_todo(&to_do_list);

                println!("Enter the index of the ToDo to remove: ");

                let mut remove_idx = String::new();
                io::stdin()
                    .read_line(&mut remove_idx)
                    .expect("Failed to read line");

                let remove_check_idx : usize = match remove_idx.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid index! Please enter a valid index.");
                        continue;
                    }
                };

                if remove_check_idx == 0 || remove_check_idx > to_do_list.len() {
                    println!("Invalid index! Please enter a valid index.");
                    continue;
                }
                
                to_do_list.remove(remove_check_idx - 1);                

                println!("ToDo removed successfully!");
            },

            "3" => {
                view_todo(&to_do_list);
                
                println!("Enter the index of the ToDo to check/uncheck:");
                let mut check_idx = String::new();

                io::stdin()
                    .read_line(&mut check_idx)
                    .expect("Failed to read line");

                let chk_check_idx : usize = match check_idx.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid index! Please enter a valid index.");
                        continue;
                    }
                };

                if chk_check_idx > to_do_list.len() || to_do_list.len() == 0 {
                    println!("Invalid index! Please enter a valid index.");
                    continue;
                }

                to_do_list[chk_check_idx - 1].change_check_flag();
            },
            // Edit function need

            "4" => {
                view_todo(&to_do_list);

                println!("Enter the index of the ToDo to edit: ");
                let mut edit_idx = String::new();

                io::stdin()
                    .read_line(&mut edit_idx)
                    .expect("Failed to read line");

                let edit_check_idx: usize = match edit_idx.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid index! Please enter a valid index.");
                        continue;
                    }
                };

                if edit_check_idx == 0 || edit_check_idx > to_do_list.len() {
                    println!("Invalid index! Please enter a valid index.");
                    continue;
                }

                println!("Enter the new ToDo: ");
                let mut new_todo = String::new();

                io::stdin()
                    .read_line(&mut new_todo)
                    .expect("Failed to read line");
                
                to_do_list[edit_check_idx - 1].edit(new_todo.trim().to_string());
                
                println!("ToDo edited successfully!");
            },

            "5" => {
                view_todo(&to_do_list);
            },

            "6" => {
                println!("Exiting the program...");
                break;
            },
            _ => println!("Invalid choice. Please enter a valid option."),
    
        }
    }
}

fn view_todo(to_do_list : &Vec<ToDo>) {
    println!("ToDoList: ");

    for(idx, todo) in to_do_list.iter().enumerate() {
        println!("{}. {} {}", idx + 1, todo.list, todo.check_icon());
    }

    if to_do_list.is_empty() {
        println!("ToDoList is Empty!!");
    }
}

fn menu() {
    println!("            (           (               ");
    println!("  *   )     )\\ )        )\\ )         )  ");
    println!("` )  /(    (()/(       (()/((     ( /(");
    println!(" ( )(_)|    /(_))  (    /(_))\\ (  )\\()) ");
    println!("(_(_()))\\  (_))_   )\\  (_))((_))\\(_))/  ");
    println!("|_   _((_)  |   \\ ((_) | |  (_|(_) |_   ");
    println!("  | |/ _ \\  | |) / _ \\ | |__| (_-<  _|  ");
    println!("  |_|\\___/  |___/\\___/ |____|_/__/\\__|  ");
    println!("                                         ");
    println!(" _____________________________________");
    println!("|                                     |");
    println!("|            Main Menu                |");
    println!("|_____________________________________|");
    println!("|                                     |");
    println!("|  1. Add a ToDo                      |");
    println!("|  2. Remove a ToDo                   |");
    println!("|  3. Complete ToDo                   |");
    println!("|  4. Edit ToDo                       |");
    println!("|  5. View ToDoList                   |");
    println!("|  6. Exit                            |");
    println!("|_____________________________________|");
    println!("                                         ");
    println!("                                         ");
    println!("Enter your choice:");
}

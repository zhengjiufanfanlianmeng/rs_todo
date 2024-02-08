use prettytable::{row, Table};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

mod time;

#[derive(Debug)]
enum Status {
    TODO,
    DONE,
}

enum Order {
    DESCTIME,
    ASCTIME,
}

enum Filter {
    Status(Status),
    Order(Order),
}

#[derive(Debug)]
struct TodoItem {
    item: String,
    status: Status,
    time: String,
}

impl TodoItem {
    fn new(item: String) -> TodoItem {
        TodoItem {
            item: item,
            status: Status::TODO,
            time: time::get_time().unwrap(),
        }
    }
}

fn main() {
    println!("\r\nUSAGE");
    println!("· 'list' -> list all todo items\r\n· 'add' -> add a new todo item\r\n· 'done' -> finish the todo item\r\n· 'quit' or 'exit' -> exit the program\r\n· 'remove' -> remove the todo item\r\n");

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
        let command = parts.get(0).map(|&s| s.to_lowercase());

        match command.as_deref() {
            Some("add") => {
                if let Some(item) = parts.get(1) {
                    add_todo(item.to_string());
                } else {
                    println!("No todo item provided.")
                }
            }
            Some("list") => {
                let filter_opt = match parts.get(1) {
                    Some(&"--asctime") => Some(Filter::Order(Order::DESCTIME)),
                    Some(&"--desctime") => Some(Filter::Order(Order::ASCTIME)),
                    Some(&"--done") => Some(Filter::Status(Status::DONE)),
                    Some(&"--todo") => Some(Filter::Status(Status::TODO)),
                    _ => None,
                };
                list_todos(filter_opt);
            }
            Some("done") => {
                if let Some(num) = parts.get(1) {
                    done_todo(num);
                } else {
                    println!("No todo ID provided.");
                }
            }
            Some("remove") => {
                if let Some(num) = parts.get(1) {
                    remove_todo(num);
                } else {
                    println!("No todo ID provided.");
                }
            }
            Some("quit") | Some("exit") => break,
            _ => println!("Unknown or incomplete command."),
        }
    }
}

fn list_todos(param: Option<Filter>) {
    let mut table = Table::new();
    table.add_row(row!["ID", "Item", "Status", "CreateTime"]);
    let path = Path::new("todo.txt");
    let file = match OpenOptions::new().read(true).open(&path) {
        Ok(file) => file,
        Err(_) => {
            println!("No todo.txt found. You might want to add some todo items first!");
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut rows: Vec<_> = reader
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let line = line.unwrap().replace("\\r", "");
            let parts = line
                .split("\\t ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            // 构建行，包括index以便排序
            (index + 1, parts)
        })
        .collect();

    match param {
        Some(Filter::Order(Order::DESCTIME)) => {
            rows.sort_by_key(|&(.., ref line_parts)| std::cmp::Reverse(line_parts[2].clone()))
        }
        Some(Filter::Order(Order::ASCTIME)) => {
            rows.sort_by_key(|&(.., ref line_parts)| line_parts[2].clone())
        }
        Some(Filter::Status(Status::TODO)) => {
            rows.retain(|&(.., ref line_parts)| line_parts[1].contains("TODO"))
        }
        Some(Filter::Status(Status::DONE)) => {
            rows.retain(|&(.., ref line_parts)| line_parts[1].contains("DONE"))
        }
        None => {} // 不进行排序
    }

    // 对表中行内容进行插入排序结果
    for (index, row) in rows {
        table.add_row(row![index, row[0], row[1], row[2]]);
    }

    if table.len() == 1 {
        println!("No todo item found, pelase add some items first!\r\n");
        return;
    }

    table.printstd();
    println!("");
}

fn add_todo(item: String) {
    let todo_item = TodoItem::new(item.trim().to_string());

    let todo_entry = format!(
        "{}\\t {:?}\\t {}\\r\
",
        todo_item.item.trim(),
        todo_item.status,
        todo_item.time
    );

    let path = Path::new("todo.txt");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&path)
        .expect("Unable to open the file");

    writeln!(file, "{}", todo_entry).expect("Failed to write to file");
    list_todos(None);
}

fn done_todo(num: &str) {
    let path = Path::new("todo.txt");

    // 将输入转换为有意义的数字
    let id: usize = if let Ok(num) = num.parse::<usize>() {
        num
    } else {
        println!("Please enter a valid ID");
        return;
    };

    // 读取旧的todo items
    let mut todos = Vec::new();
    let mut found = false;
    let file = BufReader::new(File::open(&path).expect("Failed to open the file"));
    for (index, line) in file.lines().enumerate() {
        let line = line.expect("Unable to read line");
        let line_arr: Vec<&str> = line.split("\\t ").collect();

        let status = match line_arr[1].trim() {
            "TODO" => Status::TODO,
            "DONE" => Status::DONE,
            _ => unreachable!(),
        };

        let time = line_arr[2].trim().to_string();

        todos.push(TodoItem {
            item: String::from(line_arr[0]),
            status,
            time,
        });

        if id == index + 1 {
            found = true;
        }
    }

    if !found {
        println!("Please enter a correct ID");
        return;
    } else {
        todos[id - 1].status = Status::DONE;
    }

    // 将更新后的TODO列表写回文件
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .expect("Unable to open the file");

    for todo in todos {
        let todo_entry = format!(
            "{}\\t {:?}\\t {}\\r\
",
            todo.item.trim(),
            todo.status,
            todo.time
        );
        writeln!(file, "{}", todo_entry).expect("Failed to write to file");
    }

    if found {
        println!("Todo item {} set as done.", id);
        list_todos(None);
    }
}

fn remove_todo(num: &str) {
    let path = Path::new("todo.txt");

    let id: usize = if let Ok(num) = num.parse::<usize>() {
        num
    } else {
        println!("Please enter a valid ID.");
        return;
    };

    let mut todos = Vec::new();
    let file = match File::open(&path) {
        Ok(file) => BufReader::new(file),
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => {
                    println!("Todo file not found. Perhaps you'd like to add some todo item first?")
                }
                _ => println!("An error occurred while opening todo file: {}", error),
            }
            return;
        }
    };

    
    let file = BufReader::new(file);
    
    for line in file.lines() {
        // 收集非要删除的 todo items
        let line = line.expect("Unable to read line.");
        let time_parts: Vec<&str> = line.split("\\t ").collect(); // 分割字符串获取各个字段
        
        let status = match time_parts[1] {
            "TODO" => Status::TODO,
            "DONE" => Status::DONE,
            _ => unreachable!(),
        };
        
        let time = time_parts[2].replace("\\r", "");
        
        todos.push(TodoItem {
            item: String::from(time_parts[0]),
            status,
            time,
        });
        
    }
    
    if todos.is_empty() {
        println!("No todo item found, pelase add some items first!\r\n");
        return;
    } else {
        todos.remove(id - 1);
    }

    if id > todos.len() + 1 || id == 0 {
        println!("The ID provided does not exist.\r\n");
        return;
    }

    // 把更新后的 todo 列表写回文件
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .expect("Unable to open file");

    for todo in todos {
        let todo_entry = format!(
            "{}\\t {:?}\\t {}\\r\
",
            todo.item, todo.status, todo.time
        );

        writeln!(file, "{}", todo_entry).expect("Failed to write to file");
    }

    println!("Todo item {} has been removed.", id);
    list_todos(None);
}

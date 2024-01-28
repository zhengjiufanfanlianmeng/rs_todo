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
    println!("· 'list' -> list all todo items\r\n· 'add' -> add a new todo item\r\n· 'done' -> finish the todo item\r\n· 'quit' or 'exit' -> exit the program\r\n");

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        // let command = input.trim();

        let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
        let command = parts.get(0).map(|&s| s.to_lowercase());

        match command.as_deref() {
            Some("add") => {
                if let Some(item) = parts.get(1) {
                    add_todo(item.to_string());
                } else {
                    println!("No todo item provided.")
                }
            },
            Some("done") => {
                if let Some(num) = parts.get(1) {
                    done_todo(num);
                } else {
                    println!("No todo ID provided.");
                }
            },
            Some("list") => list_todos(),
            Some("quit") | Some("exit") => break,
            _ => println!("Unknown or incomplete command."),
        }
    }
}

fn list_todos() {
    let mut table = Table::new();
    table.add_row(row!["ID", "Item", "Status", "CreateTime"]);
    let path = Path::new("todo.txt");
    let file = match OpenOptions::new().read(true).open(&path) {
        Ok(file) => file,
        Err(_) => {
            println!("No todo.txt found. You might want to add some todo items first.");
            return;
        }
    };

    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap().replace("\\r", "");
        let line_arr = line
            .split("\\t ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        table.add_row(row![index + 1, line_arr[0], line_arr[1], line_arr[2]]);
    }

    if table.len() == 1 {
        println!("No todo items found, pelase add some items first.");
        return;
    }

    table.printstd();
    println!("");
}

fn add_todo(item: String) {
    // print!("Please enter your todo item: ");
    // io::stdout().flush().expect("Failed to flush stdout");
    let todo_item = TodoItem::new(item.trim().to_string());
    // io::stdin()
    //     .read_line(&mut todo_item.item)
    //     .expect("Failed to read line");

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
    list_todos();
}

fn done_todo(num: &str) {
    let path = Path::new("todo.txt");

    // 询问用户输入完成的 todo 序号
    // print!("Enter the ID of the todo you've done: ");
    // io::stdout().flush().expect("Failed to flush stdout");
    // let mut id_input = String::new();
    // io::stdin()
    //     .read_line(&mut id_input)
    //     .expect("Failed to read line");

    // 将输入转换为有意义的数字
    let id: usize = if let Ok(num) = num.parse::<usize>() {
        num
    } else {
        println!("Please enter a valid number");
        return;
    };

    // 读取旧的todo items
    let mut todos = Vec::new();
    let mut found = false;
    let file = BufReader::new(File::open(&path).expect("Failed to open the file"));
    for (index, line) in file.lines().enumerate() {
        let line = line.expect("Unable to read line");
        let line_arr: Vec<&str> = line.split("\\t ").collect();

        let status = 
            if line_arr[1].contains("TODO") {
                Status::TODO
            } else if line_arr[1].contains("DONE") {
                Status::DONE
            } else {
                Status::TODO
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
        list_todos();
    }
}

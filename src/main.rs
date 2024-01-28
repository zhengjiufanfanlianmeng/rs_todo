use prettytable::{row, Table};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug)]
enum Status {
    TODO,
    DONE,
}

#[derive(Debug)]
struct TodoItem {
    item: String,
    status: Status,
}

impl TodoItem {
    fn new(item: String) -> TodoItem {
        TodoItem {
            item: item,
            status: Status::TODO,
        }
    }
}

fn main() {
    println!("\r\nUSAGE");
    println!("· Type 'list' to list all todo items\r\n· Type 'add' to add a new todo item\r\n· Type 'done' to finish the todo item\r\n· Type 'quit' to exit the program\r\n");

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let command = input.trim();

        match command {
            "add" => add_todo(),
            "done" => done_todo(),
            "list" => list_todos(),
            "quit" | "exit" => break,
            _ => println!("Unknown command."),
        }
    }
}

fn list_todos() {
    let mut table = Table::new();
    table.add_row(row!["ID", "Item", "Status"]);
    let path = Path::new("todo.txt");
    let file = match OpenOptions::new().read(true).open(&path) {
        Ok(file) => file,
        Err(_) => {
            println!("No todo.txt found. You might want to add some todo items first.");
            return;
        }
    };

    let reader = BufReader::new(file);
    // println!("{:?}", reader);
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap().replace("\\r", "");
        let line_arr = line
            .split("\\t ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        table.add_row(row![index + 1, line_arr[0], line_arr[1]]);
    }

    if table.len() == 1 {
        println!("No todo items found, pelase add some items first.");
        return;
    }

    table.printstd();
    println!("");
}

fn add_todo() {
    print!("Please enter your todo item: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut todo_item = TodoItem::new(String::new());
    io::stdin()
        .read_line(&mut todo_item.item)
        .expect("Failed to read line");

    let todo_entry = format!(
        "{}\\t {:?}\\r\
",
        todo_item.item.trim(),
        todo_item.status
    );

    let path = Path::new("todo.txt");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&path)
        .expect("Unable to open the file");

    writeln!(file, "{}", todo_entry).expect("Failed to write to file");
    // println!("Todo item added with status 'todo'.");

    list_todos();
}

fn done_todo() {
    let path = Path::new("todo.txt");

    // 询问用户输入完成的 todo 序号
    print!("Enter the ID of the todo you've done: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut id_input = String::new();
    io::stdin()
        .read_line(&mut id_input)
        .expect("Failed to read line");

    // 将输入转换为有意义的数字
    let id: usize = match id_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
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


        todos.push(TodoItem {
            item: String::from(line_arr[0]),
            status,
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
            "{}\\t {:?}\\r\
",
            todo.item.trim(),
            todo.status
        );
        writeln!(file, "{}", todo_entry).expect("Failed to write to file");
    }

    if found {
        println!("Todo item {} set as done.", id);
        list_todos();
    }
}










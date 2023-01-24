use crossterm::{event, terminal, cursor, ExecutableCommand, execute};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{style, Color, Print};
use crossterm::style::Stylize;
use serde_derive::{Serialize, Deserialize};
use serde_json::{Value, json};
use std::fs::{File, self};
use std::io::{Write, Read, self};
use std::ops::Add;

use crate::common::generate;
use crate::todo::todo;

#[derive(Serialize, Deserialize)]
struct Todos {
    items: serde_json::Value,
}

pub fn main(item: String, index: usize) {
    let main_file_path = format!("./todos/{}.json", &item.to_ascii_lowercase().trim().replace(" ", "_"));

    let mut file = File::open(&main_file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    
    let mut todos: Todos = serde_json::from_str(&contents)
        .expect("Unable to parse JSON");

    let todo_items_length = todos.items.as_array().expect("Invalid todo item list").len();
    let extra_content: Option<Vec<String>> = if todo_items_length == 0 {
        Some(vec!["(You have no TO-DO items!)".to_string()])
    } else {
        None
    };
        
    display_main_title(&item, &extra_content);
    if todo_items_length != 0 {
        let mut last = todos.items
            .as_array_mut()
            .unwrap()
            .pop().
            unwrap();

        last["name"] = Value::String(last["name"]
            .as_str()
            .unwrap()
            .to_string()
            .add("\n\r"));
            
        todos.items
            .as_array_mut()
            .unwrap()
            .push(last);
    }

    let specials = vec!["Create", "Clear", "Back"];
    for item in specials.iter() {
        push_item(&mut todos, item);
    }

    let user_input = &display_main_context_menu(&todos.items, &item, index, &extra_content, &specials);
    handle_user_input(&user_input, item, main_file_path, &mut todos, &specials)    
}

fn handle_user_input(user_input: &Vec<(Value, usize)>, item: String, main_file_path: String, todos: &mut Todos, specials: &Vec<&str>) {
    let selected_item = &user_input[0].0;
    let maybe_item: String = match selected_item {
        Value::String(str) => str.to_string(),
        Value::Object(obj) => obj["name"].as_str().unwrap().to_string(),
        _ => panic!("Invalid Input at {}", selected_item)
    };

    match maybe_item.as_ref() {
        "Back" => {
            todo::main(item);
        },
        "Clear" => {
            let default = json!({ "items": []});
            fs::write(main_file_path, serde_json::to_string(&default).unwrap()).unwrap();
            main(item, 0);
        },
        "Create" => {
            print!("\nEnter a name -> ");
            terminal::disable_raw_mode().unwrap();
            io::stdout().flush().expect("Unexpected error when trying to flush the buffer");
    
            let user_input = generate::input();
            terminal::enable_raw_mode().unwrap();
    
            undo_changes(todos, &specials);
    
            todos.items
                .as_array_mut()
                .unwrap()
                .push(json!({ "complete": Value::Bool(false), "name": Value::String(user_input)}));
            
            fs::write(main_file_path, serde_json::to_string(&todos).unwrap()).unwrap();
            main(item, 0);
        },
        _ => {
            todos.items
                .as_array_mut()
                .unwrap()
                .iter_mut()
                .find(|item| item["name"].as_str().unwrap() == selected_item["name"])
                .unwrap()["complete"] = Value::Bool(!selected_item["complete"].as_bool().unwrap());

            undo_changes(todos, &specials);

            fs::write(main_file_path, serde_json::to_string(&todos).unwrap()).unwrap();
            main(item, user_input[0].1);
        }
    }
}

fn display_main_title(todo: &String, extra_content: &Option<Vec<String>>) {
    match extra_content {
        Some(content) => {
            print!("Viewing items inside {}! \n\r{} \n\n\r", todo.trim(), content.join(""));
        }
        None => {
            print!("Viewing items inside {}! \n\n\r", todo.trim());
        }
    }
}

fn undo_changes(todos: &mut Todos, specials: &Vec<&str>) {
    for _ in specials.iter() {
        todos.items
            .as_array_mut()
            .unwrap()
            .pop();
    }

    if todos.items.as_array().expect("Invalid todo item list").len() != 0 {
        let mut last = todos.items
            .as_array_mut()
            .unwrap()
            .pop()
            .unwrap();

        last["name"] = Value::String(last["name"]
            .as_str()
            .unwrap()
            .to_string()
            .replace("\n\r", ""));

        todos.items
            .as_array_mut()
            .unwrap()
            .push(last);
    }
}

fn push_item(todos: &mut Todos, item: &str) {
    todos.items
        .as_array_mut()
        .unwrap()
        .push(Value::String(item.to_string()));
}

fn display_main_context_menu(items: &Value, todo: &String, index: usize, extra_content: &Option<Vec<String>>, specials: &Vec<&str>) -> Vec<(Value, usize)> {
    // Wait for the user to select an item
    let mut selected_index = index;

    redraw_main_menu(items.as_array().unwrap(), selected_index, &todo, extra_content, specials);
    loop {
        let event = event::read().unwrap();
        match event {
            crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                // Move the selection up
                if selected_index > 0 {
                    selected_index -= 1;
                }
                redraw_main_menu(items.as_array().unwrap(), selected_index, &todo, extra_content, specials);
            }
            crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE, 
                ..
            }) => {
                // Move the selection down
                if selected_index < items.as_array().unwrap().len() - 1 {
                    selected_index += 1;
                }
                redraw_main_menu(items.as_array().unwrap(), selected_index, &todo, extra_content, specials);
            }
            crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE, 
                ..
            }) => {
                // Return the selected item
                break vec![(items[selected_index].clone(), selected_index)];
            }
            _ => {
                // Return the selected item
                break vec![(items[selected_index].clone(), selected_index)];
            }
        }
    }
}

fn redraw_main_menu(items: &Vec<Value>, selected_index: usize, todo: &String, extra_content: &Option<Vec<String>>, specials: &Vec<&str>) {
    let mut stdout = std::io::stdout();

    // Move the cursor to the top-left corner of the screen
    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All)
    ).unwrap();    

    display_main_title(todo, extra_content);

    // Draw the context menu
    for (index, base_item) in items.iter().enumerate() {
        let maybe_item: String = match base_item {
            Value::String(str) => str.to_string(),
            Value::Object(obj) => obj["name"].as_str().unwrap().to_string(),
            _ => panic!("Invalid Input at {}", base_item)
        };
        let item = maybe_item.replace("\"", "").trim().to_string();
        let emote = if specials.contains(&item.as_ref()) { "" } else { 
            match base_item["complete"].as_bool().expect(&format!("Could not unwrap bool in {}", item)) {
                true => "✔ ",
                false => "✘ ",
            }
         };

        let item_text = if index == selected_index {
            "> ".to_string() + &style(emote).with(Color::Blue).bold().to_string() + &style(&&maybe_item).with(Color::Green).bold().to_string()
        } else {
            emote.to_string() + &maybe_item.to_string()
        };
        stdout.execute(Print(item_text)).unwrap();
        stdout.execute(cursor::MoveToNextLine(1)).unwrap();
    }

    // Flush the output
    stdout.flush().unwrap();
}
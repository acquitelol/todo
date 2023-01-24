use crossterm::{event, terminal, cursor, ExecutableCommand, execute};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{style, Color, Print};
use crossterm::style::Stylize;
use std::fs;
use std::io::{Write};

use crate::paths::{create, entry};
use crate::common::generate;
use crate::todo::todo;

pub fn main() {
    let entries = match fs::read_dir("./todos/") {
        Ok(entries) => entries,
        Err(_) => panic!("Failed to read directory"),
    };

    let has_todos = entries.count() > 0;

    if has_todos {
        let content = "Here are your current TO-DO lists! :3\n\r".to_string();
        display_main_title(&content);

        let mut items = generate::name_vector("./todos/");

        let last = items.pop().unwrap();
        items.push(last + "\n\r");
        items.push("Clear".to_string());
        items.push("Back".to_string());
        
        let selected_item = display_main(&items, &content);

        match selected_item.as_ref() {
            "Clear" => {
                let todos = match fs::read_dir("./todos/") {
                    Ok(todos) => todos,
                    Err(_) => panic!("Failed to read directory"),
                };

                // Iterate over the files and delete them one by one
                for todo in todos {
                    let todo = match todo {
                        Ok(todo) => todo,
                        Err(_) => continue,
                    };
                    let path = todo.path();
                    match fs::remove_file(path) {
                        Ok(done) => done,
                        Err(e) => println!("Error deleting file: {}", e),
                    }
                }

                entry::main(Some(vec!["\r\nCleared all TO-DOs!".to_string()]))
            }
            "Back" => {
                entry::main(None);
            },
            _ => todo::main(selected_item)
        }
    } else {
        let content = "You don't have any TO-DOs! D:\n\r".to_string();
        display_main_title(&content);

        let options = vec!["Create".to_string(), "Back".to_string()];
        let selected_item = display_main(&options, &content);

        match selected_item.as_ref() {
            "Create" => create::main(),
            "Back" => entry::main(None),
            _ => panic!("You chose an invalid option somehow @_@")
        }
    }
}

fn display_main_title(content: &String) {
    println!("{}", content);
}

fn display_main(items: &Vec<String>, content: &String) -> String {
    // Wait for the user to select an item
    let mut selected_index = 0;

    redraw_main_menu(items, selected_index, &content);
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
                redraw_main_menu(items, selected_index, &content);
            }
            crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE, 
                ..
            }) => {
                // Move the selection down
                if selected_index < items.len() - 1 {
                    selected_index += 1;
                }
                redraw_main_menu(items, selected_index, &content);

            }
            crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE, 
                ..
            }) => {
                // Return the selected item
                break items[selected_index].to_string();
            }
            _ => {
                // Return the selected item
                break items[selected_index].to_string();
            }
        }
    }
}

fn redraw_main_menu(items: &Vec<String>, selected_index: usize, content: &String) {
    let mut stdout = std::io::stdout();

    // Move the cursor to the top-left corner of the screen
    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All)
    ).unwrap();    

    display_main_title(&content);

    // Draw the context menu
    for (index, item) in items.iter().enumerate() {
        let item_text = if index == selected_index {
            "> ".to_string() + &style(item).with(Color::Green).bold().to_string()
        } else {
            item.to_string()
        };
        stdout.execute(Print(item_text)).unwrap();
        stdout.execute(cursor::MoveToNextLine(1)).unwrap();
    }

    // Flush the output
    stdout.flush().unwrap();
}
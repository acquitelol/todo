use crossterm::{event, terminal, cursor, ExecutableCommand, execute};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{style, Color, Print};
use crossterm::style::Stylize;
use serde_json::json;
use std::fs::{File, self};
use std::io::{Write, self};

use crate::paths::entry;
use crate::common::generate;

pub fn main() {
    display_main_title();

    let items = vec!["Enter Name", "Use Default", "Back"];
    let selected_item = display_main(&items);

    match selected_item.as_ref() {
        "Enter Name" => handle_create_todo(None),
        "Use Default" => handle_create_todo(Some("Default".to_string())),
        "Back" => entry::main(None),
        _ => panic!("Unexpected Error: Invalid choice picked!")
    }
}

fn buf_exists(buf_name: &String) -> bool {
    if let Ok(_) = fs::metadata(buf_name) {
        return true;
    } else {
        return false;
    };
}

fn handle_create_todo(file_name: Option<String>) {
    let handled_file_name = match file_name {
        Some(input) => input,
        None => {
            print!("\nEnter a name -> ");
            terminal::disable_raw_mode().unwrap();
            io::stdout().flush().expect("Unexpected error when trying to flush the buffer");

            let user_input = generate::input();
            terminal::enable_raw_mode().unwrap();

            user_input.trim().replace(" ", "_")
        }
    };

    if !buf_exists(&"./todos/".to_string()) {
        fs::create_dir("./todos/").unwrap();
    }

    let file_extension = "json";
    let default_json = json!({
        "items": []
    });

    let mut counter = 0;
    loop {
        counter += 1;

        let original_formatted = format!("./todos/{}.{}", handled_file_name, file_extension);
        if !buf_exists(&original_formatted) {
            match File::create(&original_formatted) {
                Ok(file) => file,
                Err(error) => panic!("Unexpected Error Occured When Creating Original: {}", error)
            };
            fs::write(original_formatted, serde_json::to_string(&default_json).unwrap()).unwrap();
            break;
        }

        let duplicate_formatted = format!("./todos/{}_{}.{}", handled_file_name, counter, file_extension);
        if !buf_exists(&duplicate_formatted) {
            match File::create(&duplicate_formatted) {
                Ok(file) => file,
                Err(error) => panic!("Unexpected Error Occured When Creating Duplicate: {}", error)
            };
            fs::write(duplicate_formatted, serde_json::to_string(&default_json).unwrap()).unwrap();
            break;
        }
    }

    entry::main(Some(vec![format!("\r\nCreated new TO-DO with name {}!", handled_file_name)]));
}

fn display_main_title() {
    println!("Create a new TO-DO List item! :3\r");
    println!("Give your wonderful creation a name hehe~\r");
    println!("You can leave this blank and choose `Use Default`..\r\n")
}

fn display_main(items: &Vec<&str>) -> String {
    // Wait for the user to select an item
    let mut selected_index = 0;

    redraw_main_menu(items, selected_index);
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
                redraw_main_menu(items, selected_index);
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
                redraw_main_menu(items, selected_index);

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

fn redraw_main_menu(items: &Vec<&str>, selected_index: usize) {
    let mut stdout = std::io::stdout();

    // Move the cursor to the top-left corner of the screen
    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All)
    ).unwrap();    

    display_main_title();

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
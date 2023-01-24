use crossterm::{event, terminal, cursor, ExecutableCommand, execute};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{style, Color, Print};
use crossterm::style::Stylize;
use std::fs;
use std::io::{Write, self};

use crate::common::generate;
use crate::paths::list;
use crate::todo::view;

pub fn main(item: String) {
    let main_file_path = format!("./todos/{}.json", &item.to_ascii_lowercase().trim().replace(" ", "_"));
    display_main_title(&item);

    let items = vec!["View", "Rename", "Delete", "Back"];
    let selected_item = display_main_context_menu(&items, &item);

    match selected_item.as_ref() {
        "View" => {
            view::main(item, 0)
        },
        "Rename" => {
            print!("\nEnter a name -> ");
            terminal::disable_raw_mode().unwrap();
            io::stdout().flush().expect("Unexpected error when trying to flush the buffer");

            let user_input = generate::input();
            terminal::enable_raw_mode().unwrap();
            
            fs::rename(&main_file_path, &format!("./todos/{}.json", &user_input.to_ascii_lowercase().trim().replace(" ", "_"))).unwrap();
            main(user_input);
        },
        "Delete" => {
            fs::remove_file(&main_file_path).unwrap();
            list::main();
        },
        "Back" => list::main(),
        _ => panic!("Unexpected Error: Invalid choice picked!")
    }
}

fn display_main_title(todo: &String) {
    print!("Opened the TO-DO {}! \n\n\r", todo.trim());    
}

fn display_main_context_menu(items: &Vec<&str>, todo: &String) -> String {
    // Wait for the user to select an item
    let mut selected_index = 0;

    redraw_main_menu(items.to_vec(), selected_index, &todo);
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
                redraw_main_menu(items.to_vec(), selected_index, &todo);
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
                redraw_main_menu(items.to_vec(), selected_index, &todo);
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

fn redraw_main_menu(items: Vec<&str>, selected_index: usize, todo: &String) {
    let mut stdout = std::io::stdout();

    // Move the cursor to the top-left corner of the screen
    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All)
    ).unwrap();    

    display_main_title(todo);

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
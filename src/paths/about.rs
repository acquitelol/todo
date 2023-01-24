use crossterm::{event, terminal, cursor, ExecutableCommand, execute};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{style, Color, Print};
use crossterm::style::Stylize;
use std::io::Write;

use crate::paths::entry;

pub fn main(version: &String) {
    display_main_title(&version);

    let items = vec!["Back"];
    let selected_item = display_main(&items, &version);

    match selected_item.as_ref() {
        "Back" => entry::main(None),
        _ => panic!("Unexpected Error: Invalid choice picked!")
    }
}

fn display_main_title(version: &String) {
    println!("It's so sweet! :3\r");
    println!("Created by Rosie/Acquite as a random Rust Project~\r");
    println!("Version: {}!\n\r", version)
}

fn display_main(items: &Vec<&str>, version: &String) -> String {
    // Wait for the user to select an item
    let mut selected_index = 0;

    redraw_main_menu(items, selected_index, version);
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
                redraw_main_menu(items, selected_index, version);
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
                redraw_main_menu(items, selected_index, version);
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

fn redraw_main_menu(items: &Vec<&str>, selected_index: usize, version: &String) {
    let mut stdout = std::io::stdout();

    // Move the cursor to the top-left corner of the screen
    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All)
    ).unwrap();    

    display_main_title(version);

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
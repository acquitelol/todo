use crossterm::{event, terminal, cursor, ExecutableCommand, execute};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{style, Color, Print};
use crossterm::style::Stylize;
use std::io::Write;

use crate::paths::{about, create, list};
use crate::common::generate;

pub fn main(extra_content: Option<Vec<String>>) {
    display_main_title(&extra_content);

    let mut items = generate::name_vector("./src/paths");
    items.push("Exit".to_string());
    let selected_item = display_main_context_menu(&items, &extra_content);

    match selected_item.as_ref() {
        "About" => about::main(),
        "Create" => create::main(),
        "List" => list::main(),
        "Exit" => std::process::exit(0),
        _ => panic!("Unexpected Error: Invalid choice picked!")
    }
}

fn display_main_title(extra_content: &Option<Vec<String>>) {
    match extra_content {
        Some(content) => {
            print!("Welcome to this random TO-DO list terminal app I wrote! {} \n\n\r", content.join(""));
        }
        None => {
            print!("Welcome to this random TO-DO list terminal app I wrote! \n\n\r");
        }
    }
}

fn display_main_context_menu(items: &Vec<String>, extra_content: &Option<Vec<String>>) -> String {
    // Wait for the user to select an item
    let mut selected_index = 0;

    redraw_main_menu(items.to_vec(), selected_index, &extra_content);
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
                redraw_main_menu(items.to_vec(), selected_index, &extra_content);
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
                redraw_main_menu(items.to_vec(), selected_index, &extra_content);
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

fn redraw_main_menu(items: Vec<String>, selected_index: usize, extra_content: &Option<Vec<String>>) {
    let mut stdout = std::io::stdout();

    // Move the cursor to the top-left corner of the screen
    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All)
    ).unwrap();    

    display_main_title(&extra_content);

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
mod paths;
use crossterm::terminal;
use paths::entry;
mod common;
mod todo;

fn main() {
    terminal::enable_raw_mode().unwrap();
    entry::main(None);
}

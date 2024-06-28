mod db;

use std::error::Error;
use arboard::Clipboard;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, KeyCode};
use crossterm::{event, execute};
use crossterm::event::Event::Key;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use rusqlite::ErrorCode;
use tui::backend::{Backend, CrosstermBackend};
use tui::{Frame, Terminal};
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, BorderType, Clear, List, ListItem, ListState, Paragraph};
use crate::db::Database;

enum InputMode {
    Normal,
    Title,
    Username,
    Password,
    Submit,
    Search,
    List,
    Delete
}

#[derive(Clone)]
pub struct Password {
    id: usize,
    title: String,
    username: String,
    password: String,
}

impl Password {
    pub fn new(title: String, username: String, password: String) -> Password {
        Password {
            id: 0,
            title: title,
            username: username,
            password: password
        }
    }

    pub fn new_with_id(title: String, username: String, password: String, id: usize) -> Password {
        Password {
            id: id,
            title: title,
            username: username,
            password: password
        }
    }

}

struct PassMngr {
    db: Database,
    mode: InputMode,
    list_state: ListState,
    passwords: Vec<Password>,
    search_txt: String,
    search_list: Vec<Password>,
    new_title: String,
    new_username: String,
    new_password: String,
    edit_mode: bool,
    edit_index: Option<usize>
}

impl PassMngr {
    pub fn new(key: String) -> PassMngr {
        let db = match Database::new(key) {
            Ok(db) => db,
            Err(e) => {
                if e.sqlite_error_code().unwrap() == ErrorCode::NotADatabase {
                    println!("Passphrase is not valid!");
                    std::process::exit(1);
                }else {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
        };

        let passwords = db.load();
        PassMngr {
            db: db,
            mode: InputMode::Normal,
            list_state: ListState::default(),
            passwords: passwords,
            search_txt: String::new(),
            search_list: vec![],
            new_password: String::new(),
            new_title: String::new(),
            new_username: String::new(),
            edit_mode: false,
            edit_index: None
        }
    }

    pub fn change_mode(&mut self, mode: InputMode) {
        self.mode = mode;
    }

    pub fn clear_fields(&mut self) {
        self.new_password.clear();
        self.new_title.clear();
        self.new_username.clear();
    }

    pub fn insert(&mut self) {
        let password = Password::new(self.new_title.to_owned(), self.new_username.to_owned(), 
        self.new_password.to_owned());

        self.db.insert(&password);
        self.passwords.push(password);

        self.clear_fields();
        self.change_mode(InputMode::Normal);
    }

    pub fn start_edit_mode(&mut self) {
        // TODO
    }


}

fn main() {

}
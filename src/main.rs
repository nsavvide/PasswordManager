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

fn main() {

}
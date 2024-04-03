use std::env;
use crate::{Terminal, Document};
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const MOVE_KEYS: [Key; 8] = 
[Key::Left, 
 Key::Up, 
 Key::Right, 
 Key::Down,
 Key::Char('h'),
 Key::Char('j'), 
 Key::Char('k'),
 Key::Char('l')];

const SHORTCUT_MOVE_KEYS: [Key; 4] = 
[Key::PageDown,
 Key::PageUp,
 Key::Home,
 Key::End];

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    want_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
}


impl Editor {
    pub fn default() -> Self {
        Self {
            want_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position { x: 0, y: 0 },
            document: Document::default(),
        }
    }

    pub fn run(&mut self) {
        // Ownership system. (Function owns this variable)
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.want_quit {
                break;
            }         
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
    
    fn welcome_msg(&self) {
        let mut msg = format!("Mercury Editor. v {}\r", VERSION);
        let width = self.terminal.size().width as usize;
        let len = msg.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        msg = format!("|{}{}", spaces, msg);
        msg.truncate(width);
        println!("{}\r", msg);
    }
    
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.welcome_msg();
            } else {
                println!("|\r");
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        // Exit Cmd
        if pressed_key == Key::Ctrl('p') {
            self.want_quit = true;
        } else if Self::is_move_key(pressed_key) {
            self.move_cursor(pressed_key);
        } else if Self::is_move_shortcut(pressed_key) {
            self.move_cursor(pressed_key);
        }

        Ok(())
    }

    fn is_move_shortcut(key: Key) -> bool {
        return SHORTCUT_MOVE_KEYS.contains(&key);
    }

    fn is_move_key(key: Key) -> bool {
        return MOVE_KEYS.contains(&key);
    }

    fn move_cursor(&mut self, key: Key) {
        let Position {mut y, mut x} = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;

        match key {
            Key::Up | Key::Char('k') => y = y.saturating_sub(1),
            Key::Down | Key::Char('j') => {
                if y < height {
                    y = y.saturating_add(1)
                }
            },
            Key::Left | Key::Char('h') => x = x.saturating_sub(1),
            Key::Right | Key::Char('l') => {
                if x < width {
                    x = x.saturating_add(1)
                }
            },
            Key::PageDown => y = height,
            Key::PageUp => y = 0,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }

        self.cursor_position = Position {x, y};
    }

    pub fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        if self.want_quit {
            Terminal::clear_screen();
            print!("Bye Now!");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        return Terminal::flush();
    }
}

fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
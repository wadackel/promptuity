use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode, ClearType};
use crossterm::{cursor, Command, QueueableCommand};

use crate::Error;

/// A struct to represent the terminal size.
#[derive(Debug)]
pub struct TermSize {
    /// The width of the terminal.
    pub width: u16,
    /// The height of the terminal.
    pub height: u16,
}

impl TermSize {
    /// Creates a new [`TermSize`] instance.
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

/// A struct to represent the cursor position.
#[derive(Debug)]
pub struct CursorPosition {
    /// The column of the cursor.
    pub col: u16,
    /// The row of the cursor.
    pub row: u16,
}

impl CursorPosition {
    /// Creates a new [`CursorPosition`] instance.
    pub fn new(col: u16, row: u16) -> Self {
        Self { col, row }
    }
}

/// A trait to represent a terminal.
///
/// The `Terminal` trait includes the APIs necessary for building prompts, and the specific implementation provided by Promptuity is [`Term`].  
/// Since the expected terminal is defined as a trait, it allows for switching to any implementation. For example, an implementation for a different terminal library or for testing purposes.
pub trait Terminal<T: std::io::Write> {
    /// Returns a mutable reference to the writer. e.g. `&mut std::io::Stderr`.
    fn writer(&mut self) -> &mut T;
    /// Returns the terminal size.
    fn size(&self) -> Result<TermSize, Error>;
    /// Enables raw mode.
    fn enable_raw(&mut self) -> Result<(), Error>;
    /// Disables raw mode.
    fn disable_raw(&mut self) -> Result<(), Error>;
    /// Shows the cursor.
    fn cursor_show(&mut self) -> Result<(), Error>;
    /// Hides the cursor.
    fn cursor_hide(&mut self) -> Result<(), Error>;
    /// Returns the cursor position.
    fn cursor_pos(&self) -> Result<CursorPosition, Error>;
    /// Moves the cursor to the specified position.
    fn move_to(&mut self, col: u16, row: u16) -> Result<(), Error>;
    /// Moves the cursor to the specified column.
    fn move_column(&mut self, to: u16) -> Result<(), Error>;
    /// Moves the cursor to the next line.
    fn move_next_line(&mut self, to: u16) -> Result<(), Error>;
    /// Moves the cursor to the previous line.
    fn move_previous_line(&mut self, to: u16) -> Result<(), Error>;
    /// Scrolls the terminal up.
    fn scroll_up(&mut self, row: u16) -> Result<(), Error>;
    /// Scrolls the terminal down.
    fn scroll_down(&mut self, row: u16) -> Result<(), Error>;
    /// Clears the terminal screen.
    fn clear(&mut self) -> Result<(), Error>;
    /// Clears the terminal screen from the cursor position downwards.
    fn clear_purge(&mut self) -> Result<(), Error>;
    /// Clears the current line.
    fn clear_current_line(&mut self) -> Result<(), Error>;
    /// Clears the terminal screen from the cursor position upwards.
    fn clear_cursor_up(&mut self) -> Result<(), Error>;
    /// Clears the terminal screen from the cursor position downwards.
    fn clear_cursor_down(&mut self) -> Result<(), Error>;
    /// Writes the specified value to the terminal.
    fn write(&mut self, value: &str) -> Result<(), Error>;
    /// Writes the specified value to the terminal and appends a newline.
    fn writeln(&mut self, value: &str) -> Result<(), Error>;
    /// Flushes all change queues.
    fn flush(&mut self) -> Result<(), Error>;
    /// Reads a key from the terminal.
    fn read_key(&mut self) -> Result<(KeyCode, KeyModifiers), Error>;
}

/// A struct to represent a terminal.
///
/// Provides an implementation of the [`Terminal`] trait that wraps [crossterm](https://github.com/crossterm-rs/crossterm).  
/// By default, it uses stderr as the Writer.
///
/// # Examples
///
/// ```no_run
/// use promptuity::{Term, Terminal};
///
/// # fn main() -> Result<(), promptuity::Error> {
/// // Create a new Term instance with stderr.
/// let mut term = Term::default();
///
/// // Clear the terminal screen.
/// term.clear()?;
/// # Ok(())
/// # }
/// ```
pub struct Term<T: std::io::Write> {
    writer: T,
}

impl<T: std::io::Write> Term<T> {
    /// Creates a new [`Term`] instance.
    pub fn new(writer: T) -> Self {
        Self { writer }
    }

    fn cmd(&mut self, command: impl Command) -> Result<(), Error> {
        self.writer.queue(command)?;
        Ok(())
    }
}

impl Term<std::io::Stderr> {
    /// Creates a new [`Term`] instance for stderr.
    pub fn stderr() -> Self {
        Self::new(std::io::stderr())
    }
}

impl Term<std::io::Stdout> {
    /// Creates a new [`Term`] instance for stdout.
    pub fn stdout() -> Self {
        Self::new(std::io::stdout())
    }
}

impl Default for Term<std::io::Stderr> {
    fn default() -> Self {
        Self::stderr()
    }
}

impl<T: std::io::Write> Terminal<T> for Term<T> {
    fn writer(&mut self) -> &mut T {
        &mut self.writer
    }

    fn size(&self) -> Result<TermSize, Error> {
        Ok(terminal::size().map(|(w, h)| TermSize::new(w, h))?)
    }

    fn enable_raw(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;
        Ok(())
    }

    fn disable_raw(&mut self) -> Result<(), Error> {
        disable_raw_mode()?;
        Ok(())
    }

    fn cursor_show(&mut self) -> Result<(), Error> {
        self.cmd(cursor::Show)
    }

    fn cursor_hide(&mut self) -> Result<(), Error> {
        self.cmd(cursor::Hide)
    }

    fn cursor_pos(&self) -> Result<CursorPosition, Error> {
        cursor::position()
            .map(|(col, row)| CursorPosition::new(col, row))
            .map_err(|err| err.into())
    }

    fn move_to(&mut self, col: u16, row: u16) -> Result<(), Error> {
        self.cmd(cursor::MoveTo(col, row))
    }

    fn move_column(&mut self, to: u16) -> Result<(), Error> {
        self.cmd(cursor::MoveToColumn(to))
    }

    fn move_next_line(&mut self, to: u16) -> Result<(), Error> {
        self.cmd(cursor::MoveToNextLine(to))
    }

    fn move_previous_line(&mut self, to: u16) -> Result<(), Error> {
        self.cmd(cursor::MoveToPreviousLine(to))
    }

    fn scroll_up(&mut self, row: u16) -> Result<(), Error> {
        self.cmd(terminal::ScrollUp(row))
    }

    fn scroll_down(&mut self, row: u16) -> Result<(), Error> {
        self.cmd(terminal::ScrollDown(row))
    }

    fn clear(&mut self) -> Result<(), Error> {
        self.cmd(terminal::Clear(ClearType::All))?;
        self.move_to(0, 0)?;
        Ok(())
    }

    fn clear_purge(&mut self) -> Result<(), Error> {
        self.cmd(terminal::Clear(ClearType::Purge))
    }

    fn clear_current_line(&mut self) -> Result<(), Error> {
        self.cmd(terminal::Clear(ClearType::CurrentLine))
    }

    fn clear_cursor_up(&mut self) -> Result<(), Error> {
        self.cmd(terminal::Clear(ClearType::FromCursorUp))
    }

    fn clear_cursor_down(&mut self) -> Result<(), Error> {
        self.cmd(terminal::Clear(ClearType::FromCursorDown))
    }

    fn write(&mut self, value: &str) -> Result<(), Error> {
        self.cmd(Print(value))?;
        Ok(())
    }

    fn writeln(&mut self, value: &str) -> Result<(), Error> {
        for line in value.to_string().lines() {
            self.write(&format!("{}\n", line))?;
            self.move_column(0)?;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.writer.flush()?;
        Ok(())
    }

    fn read_key(&mut self) -> Result<(KeyCode, KeyModifiers), Error> {
        loop {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                return Ok((code, modifiers));
            }
        }
    }
}

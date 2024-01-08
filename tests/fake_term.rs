use std::collections::VecDeque;
use std::io::Write;

use promptuity::event::*;
use promptuity::{CursorPosition, Error, TermSize, Terminal};

pub struct Term {
    output: Vec<u8>,
    actions: VecDeque<(KeyCode, KeyModifiers)>,
}

impl Term {
    pub fn new(actions: &[(KeyCode, KeyModifiers)]) -> Self {
        let actions = VecDeque::from(actions.to_vec());
        Self {
            output: vec![],
            actions,
        }
    }

    pub fn output(&self) -> String {
        String::from_utf8(self.output.clone()).unwrap()
    }
}

impl Terminal<Vec<u8>> for Term {
    fn writer(&mut self) -> &mut Vec<u8> {
        self.output.as_mut()
    }

    fn size(&self) -> Result<TermSize, Error> {
        Ok(TermSize::new(80, 40))
    }

    fn enable_raw(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn disable_raw(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn cursor_show(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn cursor_hide(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn cursor_pos(&self) -> Result<CursorPosition, Error> {
        todo!()
    }

    fn move_to(&mut self, _: u16, _: u16) -> Result<(), Error> {
        Ok(())
    }

    fn move_column(&mut self, _: u16) -> Result<(), Error> {
        Ok(())
    }

    fn move_next_line(&mut self, _: u16) -> Result<(), Error> {
        Ok(())
    }

    fn move_previous_line(&mut self, _: u16) -> Result<(), Error> {
        Ok(())
    }

    fn scroll_up(&mut self, _: u16) -> Result<(), Error> {
        Ok(())
    }

    fn scroll_down(&mut self, _: u16) -> Result<(), Error> {
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn clear_purge(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn clear_current_line(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn clear_cursor_up(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn clear_cursor_down(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn write(&mut self, value: &str) -> Result<(), Error> {
        self.output.write_all(value.as_bytes())?;
        Ok(())
    }

    fn writeln(&mut self, value: &str) -> Result<(), Error> {
        for line in value.lines() {
            self.write(line)?;
            self.write("\n")?;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn read_key(&mut self) -> Result<(KeyCode, KeyModifiers), Error> {
        Ok(self.actions.pop_front().unwrap())
    }
}

use std::io::Write;
use crossterm::{ExecutableCommand, terminal, event, QueueableCommand, style, cursor, queue};


struct Term {
    out: std::io::Stdout
}

type OutputResult<'a> = std::io::Result<&'a mut std::io::Stdout>;

impl Term {
    fn new() -> std::io::Result<Self> {
        terminal::enable_raw_mode()?;
        let mut out = std::io::stdout(); 
        queue!(out, cursor::SavePosition, terminal::EnterAlternateScreen)?;
        out.flush()?;
        Ok(Self {out})
    }

    fn clear(&mut self) -> OutputResult {
        self.out.execute(terminal::Clear(terminal::ClearType::All))
    }

    fn wait_for_event() -> std::io::Result<event::Event> {
        event::read()
    }

    fn print(&mut self, x: u16, y: u16, s: &str) -> OutputResult {
        self.out.queue(cursor::MoveTo(x, y))?.queue(style::Print(s))
    }

    fn refresh(&mut self) -> std::io::Result<()> {
        self.out.flush()
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        // moves cursor back to where it was and restores contents of terminal
        // ie, it "exits cleanly" like vim
        queue!(self.out, 
            terminal::LeaveAlternateScreen, 
            cursor::RestorePosition
        ).expect("restoring screen failed");

        self.out.flush().expect("flushing terminal failed");

        // Otherwise key press will be all weird
        terminal::disable_raw_mode()
            .expect("disabling terminal raw mode failed");
    }
}

fn main() -> crossterm::Result<()> {
    let mut term = Term::new()?;
    term.print(0, 0, "press any key to exit\n")?;
    term.refresh()?;

    Term::wait_for_event()?;
    term.clear()?;

    Ok(())
}

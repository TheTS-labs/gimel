use std::io;

use app::App;
use tui::Tui;

mod app;
mod tui;

fn main() -> io::Result<()> {
    Tui::init_panic_hook();
    let mut terminal = Tui::init()?;

    App::default().run(&mut terminal)?;

    Tui::restore()?;
    Ok(())
}

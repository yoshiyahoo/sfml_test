use sfml::window::{ContextSettings, Window, Style};
use sfml::graphics::{Color, RenderWindow, RenderTarget};
fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "SFML example that acutally works",
        Style::CLOSE,
        &ContextSettings::default()
    );

    window.set_vertical_sync_enabled(true);

    while window.is_open() {
        while let Some(ev) = window.poll_event() {
            match ev {
                sfml::window::Event::Closed | sfml::window::Event::KeyPressed { code: sfml::window::Key::Escape, .. } => window.close(),
                _ => ()
            }
        }

        window.clear(Color::BLACK);
        window.display();
    }
}

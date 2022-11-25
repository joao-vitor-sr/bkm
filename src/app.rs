use std::time::Duration;

#[derive(Debug)]
pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub input_timeout: Duration,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, input_timeout: Duration) -> App<'a> {
        App {
            title,
            should_quit: false,
            input_timeout,
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}

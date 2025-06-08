use iced::{
    Element,
    widget::{button, text},
};

pub fn run() {
    let _ = iced::run("title", update, view);
}

fn update(counter: &mut u64, other: u64) {}

fn view(counter: &u64) -> Element<'_, u64> {
    text(format!("Counter: {}", counter)).into()
}

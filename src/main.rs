use iced::{widget::text_editor, Element, Sandbox, Settings};

fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Default)]
struct App {
    editor_content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Default::default()
    }

    fn title(&self) -> String {
        String::from("Vord")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Edit(action) => self.editor_content.perform(action),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        text_editor(&self.editor_content)
            .on_action(Message::Edit)
            .into()
    }
}

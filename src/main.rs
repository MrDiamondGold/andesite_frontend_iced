use iced::{Column, Container, Element, Length, Sandbox, Settings, Text};

fn main() -> iced::Result {
    AndesiteFrontend::run(Settings::default())
}

struct AndesiteFrontend {
    
}

#[derive(Debug, Clone)]
enum Message {

}

impl Sandbox for AndesiteFrontend {
    type Message = Message;

    fn new() -> Self {
        AndesiteFrontend {

        }
    }

    fn title(&self) -> String {
        String::from("Andesite")
    }

    fn update(&mut self, message: Self::Message) {
        match message {

        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let column = Column::new()
            .push(
                Text::new(String::from("Hello, Iced!")).size(32),
            )
            .push(
                Text::new(String::from("Second Line.")).size(16),
            )
            .push(
                Text::new(String::from("Third Line.")).size(16),
            );
        
        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

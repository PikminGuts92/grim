use iced::{
    button, scrollable, slider, text_input, Align, Button, Checkbox, Column,
    Container, Element, Length, ProgressBar, Radio, Row, Sandbox, Scrollable,
    Settings, Slider, Space, Text, TextInput,
};

#[derive(Default)]
struct Styling {
    //theme: style::Theme,
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    button: button::State,
    slider: slider::State,
    slider_value: f32,
    toggle_value: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Styling {
    type Message = Message; // type Message = ();

    fn new() -> Self {
        Styling::default()
    }

    fn title(&self) -> String {
        String::from("Default Window")
    }

    fn update(&mut self, _message: Message) {
        // This application has no interactions
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Text::new("Hello, world!")
            )
            .push(
                Button::new(&mut self.button, Text::new("Click Me!"))
                    .on_press(Message::DecrementPressed)
                    //.on_press(Message::DecrementPressed),
            )
            .into()
    }
}

fn main() {
    let settings = Settings::default();

    Styling::run(settings);
}

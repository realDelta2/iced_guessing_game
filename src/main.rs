use iced::executor::Default;
use iced::{Application, Executor, Renderer};
use iced::widget::{button, text, text_input, column, TextInput};
use iced::{Command, Settings, Theme};

use rand::Rng;
use std::cmp::Ordering;

fn main() {
    GameState::run(Settings::default());
}

const GAME_TURNS: u16 = 12;

struct GameState {
    text_input_data: String,

    turn_count: u16,
    game_ending: String,
    winning_number: i32,
    hint: String,
}


#[derive(Debug)]
enum EndGameStates {
    Loss,
    Won
}

#[derive(Debug, Clone)]
enum Message {
    TextSubmit(String),
    Submit
}

impl Application for GameState {
    type Executor = Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            GameState {text_input_data: String::from(""), turn_count: 1, game_ending: String::from(""), winning_number: rand::thread_rng().gen_range(1..=100), hint: String::from("try inputting a value!")},
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Iced Guessing Game")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TextSubmit(guess) => self.text_input_data = guess,
        Message::Submit => {
            let guess_num = &self.text_input_data.trim().parse::<i32>().expect("borked");
            self.turn_count += 1;

               self.hint = match guess_num.cmp(&self.winning_number) {
                Ordering::Less => "Too small!",
                Ordering::Greater => "Too big!",
                Ordering::Equal => "Congradulations you won" }.to_string()
            }
        }
        Command::none()

}
    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        column!(
            text(format!("you have 12 guesses to win!  hint: {}", self.hint)),
            text_input("input guess here", &self.text_input_data).on_input(Message::TextSubmit).on_submit(Message::Submit),
            button("empty"),
            text(format!("it has taken you {} turns so far", &self.turn_count))
        ).into()
    }
}




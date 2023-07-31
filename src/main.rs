use iced::executor::Default;
use iced::widget::{button, column, text, text_input, Column, Text};
use iced::Application;
use iced::{Command, Settings, Theme};

use rand::Rng;
use std::cmp::Ordering;

fn main() {
    GuessingGame::run(Settings::default()).expect("game failed to run");
}

const MAX_TURNS: u16 = 10;

enum ViewStates {
    InGame,
    Won,
    Lost
}

struct GuessingGame {
    text_input_string: String,

    turn: u16,
    winning_number: i32,
    hint: String,
    view_state: ViewStates
}

#[derive(Debug, Clone)]
enum Message {
    TextSubmit(String), // this is required for text_input to actually reflect the input value
    Submit,
    Switch
}

impl Application for GuessingGame {
    type Executor = Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            GuessingGame {
                text_input_string: String::from(""),
                turn: MAX_TURNS,
                winning_number: rand::thread_rng().gen_range(1..=100),
                hint: String::from("try inputting a value!"),
                view_state: ViewStates::InGame
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Iced Guessing Game")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TextSubmit(guess) => self.text_input_string = guess,
            Message::Switch => {self.view_state = ViewStates::InGame;
                self.winning_number = rand::thread_rng().gen_range(1..=100);
                self.turn = MAX_TURNS;
            },
            Message::Submit => {
                let guess_num = &self.text_input_string.trim().parse::<i32>();

                self.hint = match guess_num {
                    Ok(guess) => {
                        match guess.cmp(&self.winning_number) {
                            Ordering::Less => "Too small!",
                            Ordering::Greater => "Too big!",
                            Ordering::Equal => {self.view_state = ViewStates::Won; "Congradulations you won"},
                        } // this needs to handle logic for winning scene
                    }
                    Err(_) => "Try again",
                }
                .to_string(); // re-evaluate error handling method

                self.text_input_string = String::from("");

                self.turn -= 1;

                if self.turn < 1 {
                    self.view_state = ViewStates::Lost
                }
            }
        }
        Command::none()
    }
    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let screen = match self.view_state {
            ViewStates::InGame => {
                column![
                    text(format!("you have 12 guesses to win!  hint: {}", self.hint)),
                    text_input(&self.hint, &self.text_input_string)
                        .on_input(Message::TextSubmit)
                        .on_submit(Message::Submit),
                    text(format!("it has taken you {} turns so far", &self.turn)),
                    button("empty")
                ]
            },
            ViewStates::Won => {
                column![
                    text("you won the game"),
                    button("replay!"). on_press(Message::Switch)
                ]
            },
            ViewStates::Lost => {
                column![
                    text("you lost the game"),
                    button("replay!"). on_press(Message::Switch)
                ]
            }
        };
        screen.into()
    } // figure out how to handle different screens
}

fn in_game() {

}

fn lose() -> Text<'static> {
    text("you lost")
}

fn win() -> Text<'static> {
    text("you win")
}

/* column!(
            text(format!("you have 12 guesses to win!  hint: {}", self.hint)),
            text_input(&self.hint, &self.text_input_string)
                .on_input(Message::TextSubmit)
                .on_submit(Message::Submit),
            text(format!("it has taken you {} turns so far", &self.turn)),
            button("empty")
        )
        .into() */
// figure out how to handle different screens
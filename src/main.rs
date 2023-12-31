use iced::executor::Default;
use iced::widget::{button, column, text, text_input};
use iced::Application;
use iced::{Command, Settings, Theme, Alignment};

use rand::Rng;
use iced_guessing_game::Hint;

fn main() {
    GuessingGame::run(Settings::default()).expect("game failed to run");
}

const MAX_TURNS: u16 = 12;

struct GuessingGame {
    text_input_string: String,

    turn: u16,
    winning_number: i32,
    hint: String,
    view_state: ViewStates
}

enum ViewStates {
    InGame,
    Won,
    Lost
}

#[derive(Debug, Clone)]
enum Message {
    TextSubmit(String), // this is required for text_input to actually reflect the input value
    Submit,
    ResetGame
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
            
            Message::ResetGame => {self.view_state = ViewStates::InGame;
                self.winning_number = rand::thread_rng().gen_range(1..=100);
                self.turn = MAX_TURNS;
            },
           
            Message::Submit => {
                let guess_num = &self.text_input_string.trim().parse::<i32>();

                self.hint = match guess_num {
                    Ok(guess) => {
                        let guess_range = 1..=100;
                        if self.hint != String::from("Invalid Input") && guess_range.contains(guess){
                            self.turn -= 1;
                        }

                        let hint = Hint::new(guess, self.winning_number);

                        if hint.has_won {
                            self.view_state = ViewStates::Won;
                        }

                        hint.hint
                    }
                    Err(_) => "Invalid Input",
                }
                .to_string();

                self.text_input_string = String::from("");

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
                    text(format!("you have 12 guesses to win!")),
                    text(format!("Hint: {}", self.hint)),
                    text_input(&self.hint, &self.text_input_string)
                        .on_input(Message::TextSubmit)
                        .on_submit(Message::Submit)
                        .size(20),
                    text(format!("you have {} turns left", &self.turn))
                ]
            },
            ViewStates::Won => {
                column![
                    text("you won the game").size(20),
                    button("replay!"). on_press(Message::ResetGame)
                ]
            },
            ViewStates::Lost => {
                column![
                    text("you lost the game").size(20),
                    button("replay!"). on_press(Message::ResetGame)
                ]
            }
        };
        screen.align_items(Alignment::Center).padding(20).into()
    }
}

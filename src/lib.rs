use std::cmp::Ordering;

pub struct Hint {
  pub  hint: &'static str,
  pub  has_won: bool
}

impl Hint {
    pub fn new(guess: &i32, secret_number: i32) -> Hint {
        match guess.cmp(&secret_number) {
            Ordering::Less => Hint { hint: "Too small", has_won: false },
            Ordering::Greater => Hint { hint: "Too big", has_won: false },
            Ordering::Equal => Hint { hint: "Congradulations you won", has_won: true },
        }
    }
}

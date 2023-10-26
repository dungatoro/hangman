use ninput::*;

#[derive(Clone, Copy)] // derive macros...
enum Tile {
    Found(char),
    Hidden(char),
    Space
}

struct Board {
    phrase: Vec<Tile>,
    lives: u8,
    won: bool
}

impl Board {
    fn new(phrase: &str) -> Self {
        let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let phrase: Vec<Tile> = phrase // iterator stuffs
            .chars()
            .map(|c| { // combinator
                if letters.contains(c) {
                    Tile::Hidden(c)
                } else if c == ' ' {
                    Tile::Space
                } else {
                    Tile::Found(c)
                }
            })
            .collect();

        Board {
            phrase,
            lives: 7,
            won: false
        }
    }

    fn draw_phrase(&self) {
        let mut tiles = String::from("");
        for tile in self.phrase.iter() {
            match tile { // enum pattern matching 
                Tile::Found(c) => tiles.push(*c),
                Tile::Hidden(_) => tiles.push('_'),
                Tile::Space => tiles.push('/'),
            }
        }
        println!("{}", tiles);
    }

    fn draw_hangman(&self) {
        let hangman = match self.lives {
            0 => {
r#"_______
|  |
|  O
| /|\
| / \
+------"#
            },
            1 => {
r#"_______
|  |
|  O
| /|\
|   \
+------"#
            },
            2 => {
r#"_______
|  |
|  O
| /|\
|   
+------"#
            },
            3 => {
r#"_______
|  |
|  O
|  |\
|  
+------"#
            },
            4 => {
r#"_______
|  |
|  O
|  |
|  
+------"#
            },
            5 => {
r#"_______
|  |
|  O
|  
|  
+------"#
            },
            6 => {
r#"_______
|  |
|  
| 
|  
+------"#
            },
            _ => {
r#"_______
|  
|  
| 
|  
+------"#
            }
        };
        println!("{}", hangman);
    }

    fn get_guess() -> char {
        let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        loop {
            let guess = ninput("Enter guess: ");

            if let Some(c) = guess.chars().next() {
                if letters.contains(c) {
                    return c
                } else {
                    println!("{} is not a valid letter!", c);
                }
            } else {
                println!("Input not recognised.") 
            }
        }
    }

    fn apply_guess(&mut self, guess: char) {
        let mut good_guess = false;
        self.won = true;
        self.phrase = self.phrase
            .iter()
            .map(|tile| {
                if let Tile::Hidden(c) = tile {
                    // magic iterator stuff
                    let lower = c.to_lowercase().next().expect("No letter in enum.");
                    if lower == guess {
                        good_guess = true;
                        return Tile::Found(*c)
                    } else {
                        self.won = false;
                        return *tile
                    }
                }
                *tile
            })
            .collect();

        if !good_guess {
            self.lives -= 1;
        }
    }

    fn turn(&mut self) {
        self.draw_hangman();
        self.draw_phrase();
        let guess = Board::get_guess();
        self.apply_guess(guess);
        println!("");
    }

    fn main(phrase: &str) {
        // while self.lives > 0 && self.won {
        //     self.turn();
        // }
        // -- Why is loop better than while here?
        let mut board = Board::new(phrase);
        loop {
            if board.lives == 0 {
                board.draw_hangman();
                println!("Out of lives :(");
                println!("The correct answer was {}", phrase);
                break;
            } else if board.won {
                println!("You win!");
                println!("The answer was {}!", phrase);
                break;
            }
            board.turn()
        }
    }
}

fn main() {
    Board::main("I MONSTER, Who Is She?");
}

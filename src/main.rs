use std::io;
use std::fs;
use rand::Rng;

fn print_game(secret_word: &String, mask: &Vec<bool>) {

    for (i, acc) in mask.iter().enumerate() {
        if *acc {
            print!("{} ", secret_word.chars().nth(i).unwrap());
        } else {
            print!("_ ");
        }
    }
    println!("");
}

fn check_guess(secret_word: &String, mask: &mut Vec<bool>, guess: char) -> u8 {
    
    let mut count = 0;
    for (i, c) in secret_word.chars().enumerate() {
       if c == guess {
           count+=1;
           mask[i] = true;
       }
    }

    return count;
}

fn make_guess() -> char {

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    
    let guess: char = guess.trim().parse().expect("not a char");
    return guess;    
}

fn check_win(points: u8, secret_word: &String) -> bool {
    return usize::from(points) == secret_word.chars().count();
}

fn check_lose(lifes: u8) -> bool {
    return lifes == 0;
}

fn get_secret_word() -> String {
    
    let contents = fs::read_to_string("words.txt")
        .expect("Something went wrong while reading file");

    let mut words = contents.lines();
    
    let idx = rand::thread_rng().gen_range(0..words.clone().count());
    let secret_word = words.nth(idx).unwrap();
    
    return secret_word.to_string();
}

fn setup() -> (String, Vec<bool>) {

    let secret_word: String = get_secret_word();
    let mut mask: Vec<bool> = Vec::with_capacity(secret_word.chars().count());
    
    for _ in 0..secret_word.chars().count() {
        mask.push(false);
    }
    
    return (secret_word, mask);
}

fn game() {
    
    let (secret_word, mut mask) = setup();
    let mut points = 0;
    let mut lifes = 5;
    
    loop {
        if check_win(points, &secret_word) {
            println!("you win, the secret word was {}", secret_word);
            break;
        }

        if check_lose(lifes) {
            println!("you lose, the secret word was {}", secret_word);
            break;
        }

        print_game(&secret_word, &mask);
        println!("Please make your guess, you have {} guesses remaining", lifes);

        let guess = make_guess();
        let count = check_guess(&secret_word, &mut mask, guess);
        
        if count != 0 {
            points += count;
        } else {
            lifes -= 1;
        }
        
    }
} 

fn main() {
    
    game();

}

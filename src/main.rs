/*
By: Hassan Rana
Date: 2026-11-10
Program Details: A program that outputs the number of star players from a game of Fergusonball. They are awarded 5 stars for each point they score and 3 stars are taken away per foul from the player.
If a player has more than 40 points than they are considered a star player. If a whole team is full of star players, the team is a gold team. The program outputs the number of star players from the player count
and then puts a + if the team is a gold team and nothing if it isn't.
*/
use std::io;

fn input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputnum = input.trim().parse().expect("Please type a number!");
    inputnum
}
fn main() {
    let playercount = input();
    let mut gold = false;
    let mut star_player = 0;
    for _ in 0..playercount {
        let score = input();
        let fouls = input();
        let total_points = score*5 - fouls*3;
        if total_points>40 {
            star_player+=1;
        }
    }
    if star_player == playercount {
        gold = true;
    }
    if gold == true {
        println!("{}+", star_player)
    }
    else {
        print!("{}", star_player)
    }
}

use std::collections::HashMap;
use day_21::{Player, DeterministicDie};

const START_POSITIONS: (usize, usize) = (9, 1); // I'm subtracting 1 from the positions
// const START_POSITIONS: (usize, usize) = (3, 7);

// ([p1 position, p2 position, p1 score, p2 score, winner], n_universes)
// winner = {0: no one, 1: p1, 2: p2}
type Universes = HashMap<([Player; 2], usize), usize>;

fn main() {
    let players = [
        Player::new(START_POSITIONS.0),
        Player::new(START_POSITIONS.1),
    ];

    println!("{}", part1(players.clone()));

    let universes = HashMap::from([
        ((players, 0), 1)
    ]);

    println!("{}", part2(universes));
}

fn part1(mut players: [Player; 2]) -> usize {
    const WIN_CONDITION: usize = 1000;

    let mut die = DeterministicDie::default();
    let mut player_turn = 0;

    while (players[0].score < WIN_CONDITION) & (players[1].score < WIN_CONDITION) {
        let movement = (0..3).map(|_| die.roll()).sum::<usize>();
        players[player_turn].advance(movement);
        player_turn += 1;
        player_turn %= 2;
    }

    let lower_score = players.iter().map(|player| player.score).min().unwrap();
    let n_rolls = die.n_rolls;
    lower_score * n_rolls
}

fn part2(mut universes: Universes) -> usize {

    const MULTIPLIERS: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];
    const MOVEMENT: [usize; 7] = [3, 4, 5, 6, 7, 8, 9];

    let mut player_turn = 0;

    let wins = loop {
        if universes // If there are no more universes where no one has won
            .iter()
            .map(|(state, _n)| state.1)
            .min()
            .unwrap() > 0
        { // Count up the number of wins
            // [p1 wins, p2 wins]
            let mut wins = [0; 2];
            universes.into_iter().for_each(|(state, n)| {
                wins[state.1-1] += n;
            });
            break wins
        }
        
        // Updating all the universes could be done more efficiently with a
        // binary heap but this is fine for this problem
        let mut new_universes = HashMap::with_capacity(universes.len()*7);
        for ((players, winner), n) in universes {
            if winner == 0 {
                for i in 0..7 {
                    let mut new_players = players.clone();
                    new_players[player_turn].advance(MOVEMENT[i]);

                    // Record if they win
                    let winner = if new_players[player_turn].score > 20 {
                        player_turn + 1
                    } else {
                        0
                    };

                    *new_universes.entry((new_players, winner)).or_insert(0) += n * MULTIPLIERS[i];
                }
            } else {
                *new_universes.entry((players, winner)).or_insert(0) += n;
            }
        }

        // Update the current universes with the new universes
        universes = new_universes;
        player_turn = (player_turn + 1) % 2;
    };

    // Return the higher number of wins
    wins.into_iter().max().unwrap()
}

// Rolls:
// 1 + 1 + 1 = 3
// 1 + 1 + 2 = 4 with three outcomes
// 1 + 1 + 3 = 1 + 2 + 2 = 5 with six (3 + 3) outcomes
// 1 + 2 + 3 = 2 + 2 + 2 = 6 with seven (6 + 1) outcomes
// 1 + 3 + 3 = 2 + 2 + 3 = 7 with six (3 + 3) outcomes
// 2 + 3 + 3 = 8 with three outcomes
// 3 + 3 + 3 = 9
//
// 1 + 3 + 6 + 7 + 6 + 3 + 1 = 27 different outcomes from 3 to 9 which checks out
//
// I only have to track 7 different outcomes and the likelihood of each
// The longest possible game shouldn't go any longer than 14 rounds (28 turns)
// (3^3)^28 total possible universes
//
// The hardest part is going to be keeping track of score and which games have
// been terminated.
//
// We can shrink the space to four dimensions:
// - Player 1's score
// - Player 1's position
// - Player 2's score
// - Player 2's position
//
// Each player's score only has 31 possible values: 0 -> 20 + 10 = 30
// Each player's position has only 10 possible values: 1 -> 10
// That means our space only contains 31 * 10 * 31 * 10 = 96100 unique universes
//
// Can I use a transition matrix?
//
// No, I will use a HashMap and repeatedly move the whole thing to a new HashMap

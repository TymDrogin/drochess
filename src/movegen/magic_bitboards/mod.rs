use crate::gamestate::{
    board::*,
    defs::*,
};
use super::{
    defs::*,
    masks::*,
    
};


// NOTE: This explanation talks about pseudolegal moves, king safety is taken care later.


// Link to more info https://www.chessprogramming.org/Magic_Bitboards
// When talking and implementing magic bitboards it is imortant to understand why do we even care about them.
// You see, any non sliding pieces (pawns, knights, and king) all move very trivialy. From any given square there is an
// Attack pattern, that can be easily retrieved and used. This piecec always have a defind pattern of moves that does not change 
// Over current board layout. That means if we were to get all the moves for a specific piece at a given position, we just take it attack bitboard, 
// exclude all friendly pieces and by simple bitboard magic get quiet and capture moves. You can find move in masks.rs file and movegen.rs

// Now, bare with me. Bishops, rooks and queen moves do depend on the current layout of the board. If the rook has a piece infront of it it can't simply jump 
// And take the pieces after it. As you can imagine, now we can't simlpy use the static attack mask, because they cannot account for the current state of the game
// The reason why do we care about magic bitboards is optimisation. Generating one rook attack bitboard whould involve looping thru the all squares, having several 
// If statements and bla bla bla. It does not sound like much, but because this can't be precomputed it means it will be in run time. The computation it self is not that
// Intensive, but if you compare it with all the others it is a biggest chunk of all, and when we are talking about 10th of millions position it will be a huge hit on the performance


//any consecutive
//relevant occupancy                      combination of
//rook d4, 10 bits                        the masked bits
//. . . . . . . .     . . . . . . . .     4 5 6 B C E F G]
//. . . 6 . . . .     . . .some . . .     . . . . . .[1 2
//. . . 5 . . . .     . . . . . . . .     . . . . . . . .
//. . . 4 . . . .     . . .magic. . .     . . . . . . . .
//. B C . E F G .  *  . . . . . . . .  =  . . garbage . .    >> (64-10)
//. . . 2 . . . .     . . .bits . . .     . . . . . . . .
//. . . 1 . . . .     . . . . . . . .     . . . . . . . .
//. . . . . . . .     . . . . . . . .     . . . . . . . .

// On this illustation you can see the rook on d4. I will not go into why borders are not here, it's just they don't matter.
// As you can see, at this position rook covers 10 squares. This squares can or cannot be taken by other pieces, which leaves us with this
// For every square we have around 2^10, or around 1024 possible combinations. That leaves us with around 8kib for square. Well, now we getting somewhere.
// You see, every square 1 2 3 4 etc its just some bit on a 64 bit bitboard. By setting them to 1 and 0 we can loop thru the all 1024 possible OCCUPACY
// combinations. Then, for every occupacy combination we can calculate a bitboard of where the rook can actually go. Here we can start to talk about a data structure 
// For this mess. it whould look something like an array with a len 64 for each square, every element of an array is a hash map that maps occupacy combination to the
// PRECOMPUTED BITBOARD that really tels us how the rook can move.

// Why magic numbers? Well, you see, the bitboards are numbers, a big fucking numbers, and it whould be nice to transform them into much smaler numbers.
// Okay cap, why not just loop over the bits and transform them into the key to a hash map? Whell that defeats the whole point of this optimisation, which is avoiding loops.
// Now, okay, that seems fair, but you may ask why this numbers are magical? The reason is that with magical numbers several different occupacy combination can lead to the same
// bitboard in hasmap. That is called constructive collision. Why is this genius? Picture a situation. You are in front of the door, behing it is a cat and after the cat is a dog. At the current state you can move only
// To the door, and if the cat sits after the dog, vice versa, or even if they are not there does not matter at a current situation, you can only go to the door. 
// That is why magics are magic. Carefully chosen magic numbers can save you a ton of space, and dont get me wrong, 3 mb or so
// Sound not bad, but in practice it will take move, and programmers love to optimize. Good magics can save you time and space

// It looks like a giant shitpost, but i think i finnaly get how they work and can go implementing them



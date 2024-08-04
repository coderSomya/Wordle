const GAMES = "abcde crane hello somya"


fn main() {
    let Guesser = Naive::new();
    for answer in GAMES.split_whitespace(){
        play(answer, Guesser);
    }
}



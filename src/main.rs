use std::io::{self, Read};

/**
 * You'll find that the first player will [lose]
 * when the starting number of stones s is: 1,7,8,14,15,21,22... (i.e. when s % 7 < 2).
 * With this knowledge in hand the code is trivial:
 */

pub fn game_of_stones<'a>(s: i32) -> &'a str {
    if s % 7 < 2 {
        "Second"
    } else {
        "First"
    }
}

fn main() {
    let mut line = Vec::new();
    let stdin = io::stdin();
    stdin
        .lock()
        .read_to_end(&mut line)
        .expect("Could not read line");
    let lines = String::from_utf8(line).expect("Error While Convert STDIN to String");

    for l in lines.lines().skip(1) {
        let stones: i32 = l.parse().expect("Error While Converting To i32");
        println!("{}", game_of_stones(stones));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_game_of_stones() {
        assert_eq!(game_of_stones(1), "Second");
        assert_eq!(game_of_stones(2), "First");
        assert_eq!(game_of_stones(3), "First");
        assert_eq!(game_of_stones(4), "First");
        assert_eq!(game_of_stones(5), "First");
        assert_eq!(game_of_stones(6), "First");
        assert_eq!(game_of_stones(7), "Second");
        assert_eq!(game_of_stones(10), "First");
    }
}

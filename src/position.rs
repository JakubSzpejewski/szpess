/// position  might only be "(\[a-h\], \[1-8\])"
///
/// Panics on other values or on parse fail
///
pub struct Position(pub char, pub char);

impl Position {
    fn validate(file: char, rank: char) {
        assert!(
            file == 'a'
                || file == 'b'
                || file == 'c'
                || file == 'd'
                || file == 'e'
                || file == 'f'
                || file == 'g'
                || file == 'h',
            "File needs to be [a-h]"
        );
        let rank: usize = rank.to_digit(10).unwrap().try_into().unwrap();

        assert!(rank >= 1 && rank <= 8, "Rank needs to be [1-8]");
    }

    pub fn new(file: char, rank: char) -> Position {
        Position::validate(file, rank);
        Position(file, rank)
    }

    pub fn get_index_values(&self) -> (usize, usize) {
        let file = self.0;
        let rank = self.1;
        // Rank is just row + 1 because of 0 indexing
        let rank_index: usize = rank.to_digit(10).unwrap().try_into().unwrap();

        let file_index: usize = match file {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!("Unknown board file"),
        };
        (file_index, rank_index - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn it_creates_position() {
        let pos: Position = Position::new('a', '2');
        assert_eq!(pos.0, 'a');
        assert_eq!(pos.1, '2');
    }

    #[test]
    #[should_panic(expected = "File needs to be [a-h]")]
    fn it_panics_on_incorrect_file() {
        Position::new('k', '2');
    }

    #[test]
    #[should_panic(expected = "Rank needs to be [1-8]")]
    fn it_panics_on_incorrect_rank() {
        Position::new('b', '9');
    }

    #[test]
    fn it_returns_correct_board_index() {
        let pos1 = Position::new('a', '1');
        let pos2 = Position::new('e', '4');
        let pos3 = Position::new('h', '8');

        assert_eq!(pos1.get_index_values(), (0, 0));
        assert_eq!(pos2.get_index_values(), (4, 3));
        assert_eq!(pos3.get_index_values(), (7, 7));
    }
}

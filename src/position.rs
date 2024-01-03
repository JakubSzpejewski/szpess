/// position  might only be "(\[a-h\], \[1-8\])"
///
/// Panics on other values or on parse fail
///
pub struct Position(pub char, pub char);
pub struct BoardIndex(pub usize, pub usize);

impl Position {
    fn validate(file: char, rank: char) {
        if !matches!(file, 'a'..='h') {
            panic!("File needs to be [a-h]");
        }

        let rank: usize = rank.to_digit(10).unwrap().try_into().unwrap();

        assert!(rank >= 1 && rank <= 8, "Rank needs to be [1-8]");
    }

    pub fn new(file: char, rank: char) -> Position {
        Position::validate(file, rank);
        Position(file, rank)
    }

    pub fn new_from_indices(indices: BoardIndex) -> Position {
        let file_index = indices.0;
        let rank_index = indices.1;
    
        assert!(matches!(file_index, 0..=7), "File index needs to be [0-7]");
        assert!(matches!(rank_index, 0..=7), "Rank index needs to be [0-7]");
    
        // Safely unwrap, because we're sure rank is a digit
        // Add 1 because of 0 array indexing and 1 board indexing
        let rank: char = char::from_digit((rank_index + 1) as u32, 10).unwrap();
        let file: char = match file_index {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!("Should never be seen"),
        };
    
        Position::new(file, rank)
    }

    pub fn get_indices(&self) -> (usize, usize) {
        let file = self.0;
        let rank = self.1;

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
            _ => panic!("Should never be seen"),
        };

        // Rank is just row + 1 because of 0 indexing
        (file_index, rank_index - 1)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[cfg(test)]
mod tests {
    use super::{Position, BoardIndex};

    #[test]
    fn it_creates_position() {
        let pos: Position = Position::new('a', '2');
        assert_eq!(pos.0, 'a');
        assert_eq!(pos.1, '2');
    }

    #[test]
    #[should_panic(expected = "File needs to be [a-h]")]
    fn it_panics_on_incorrect_position_file() {
        Position::new('k', '2');
    }

    #[test]
    #[should_panic(expected = "Rank needs to be [1-8]")]
    fn it_panics_on_incorrect_position_rank() {
        Position::new('b', '9');
    }

    #[test]
    fn it_returns_correct_board_index() {
        let pos1 = Position::new('a', '1');
        let pos2 = Position::new('e', '4');
        let pos3 = Position::new('h', '8');

        assert_eq!(pos1.get_indices(), (0, 0));
        assert_eq!(pos2.get_indices(), (4, 3));
        assert_eq!(pos3.get_indices(), (7, 7));
    }

    #[test]
    fn it_returns_correct_position_for_indices() {
        let i1 = BoardIndex(0, 0);
        let i2 = BoardIndex(4, 3);
        let i3 = BoardIndex(7, 7);

        assert!(Position::new_from_indices(i1) == Position::new('a', '1'));
        assert!(Position::new_from_indices(i2) == Position::new('e', '4'));
        assert!(Position::new_from_indices(i3) == Position::new('h', '8'));
    }

    #[test]
    #[should_panic(expected = "Rank index needs to be [0-7]")]
    fn it_panics_on_incorrect_rank_index() {
        Position::new_from_indices(BoardIndex(2, 20));
    }

    #[test]
    #[should_panic(expected = "File index needs to be [0-7]")]
    fn it_panics_on_incorrect_position_file_index() {
        Position::new_from_indices(BoardIndex(9, 2));
    }
}

use crate::piece::PieceColor;

pub enum CastlingSide {
    Kingside,
    Queenside,
}

pub struct CastlingRights {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}

impl Default for CastlingRights {
    fn default() -> Self {
        CastlingRights {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }
}

impl CastlingRights {
    fn is_available(&self, color: PieceColor, side: CastlingSide) -> bool {
        match color {
            PieceColor::White => match side {
                CastlingSide::Kingside => self.white_kingside,
                CastlingSide::Queenside => self.white_queenside,
            },
            PieceColor::Black => match side {
                CastlingSide::Kingside => self.black_kingside,
                CastlingSide::Queenside => self.black_queenside,
            },
        }
    }

    pub fn get_fen(&self) -> String {
        let mut ret = String::new();
        if !(self.white_kingside
            && self.white_queenside
            && self.black_kingside
            && self.black_queenside)
        {
            return String::from('-');
        }
        if self.white_kingside {
            ret.push('K');
        }
        if self.white_queenside {
            ret.push('Q');
        }
        if self.black_kingside {
            ret.push('k');
        }
        if self.black_queenside {
            ret.push('q');
        }
        ret
    }
}

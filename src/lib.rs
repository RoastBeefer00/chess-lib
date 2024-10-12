mod board;
mod piece;


#[cfg(test)]
mod tests {
    use super::*;
    use board::*;
    use piece::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_board_default() {
        let squares = vec![
            Square {
                file: board::File::A,
                rank: board::Rank::One,
                piece: Some(Piece {
                    piece: piece::PieceType::Rook,
                    color: piece::Color::White,
                }),
            },
            Square {
                file: board::File::B,
                rank: board::Rank::One,
                piece: Some(Piece {
                    piece: piece::PieceType::Knight,
                    color: piece::Color::White,
                }),
            },
            Square {
                file: board::File::C,
                rank: board::Rank::One,
                piece: Some(Piece {
                    piece: piece::PieceType::Bishop,
                    color: piece::Color::White,
                }),
            },
            Square {
                file: board::File::D,
                rank: board::Rank::One,
                piece: Some(Piece {
                    piece: piece::PieceType::Queen,
                    color: piece::Color::White,
                }),
            },
            Square {
                file: board::File::E,
                rank: board::Rank::One,
                piece: Some(Piece {
                    piece: piece::PieceType::King,
                    color: piece::Color::White,
                }),
            },
            Square {
                file: board::File::F,
                rank: board::Rank::One,
                piece: Some(Piece {
                    piece: piece::PieceType::Bishop,
                    color: piece::Color::White,
                }),
            },
            Square {
                file: board::File::G,
                rank: board::Rank::One,
                piece: Some(Piece {
                    piece: piece::PieceType::Knight,
                    color: piece::Color::White,
                }),
            },
            Square {
                file: board::File::H,
                rank: board::Rank::One,
                piece: Some(Piece {
                    piece: piece::PieceType::Rook,
                    color: piece::Color::White,
                }),
            },
            Square {
                file: board::File::A,
                rank: board::Rank::Two,
                piece: Some(Piece {
                    piece: piece::PieceType::Pawn,
                    color: piece::Color::White,
                }),
            },
            Square {
                file: board::File::B,
                rank: board::Rank::Two,
                piece: Some(Piece {
                    piece: piece::PieceType::Pawn,
                    color: piece::Color::White,
                }),
            },
            Square {
                file: board::File::C,
                rank: board::Rank::Two,
                piece: Some(Piece {
                    piece: piece::PieceType::Pawn,
                    color: piece::Color::White,
                }),
            },
            Square {
                file: board::File::D,
                rank: board::Rank::Two,
                piece: Some(Piece {
                    piece: piece::PieceType::Pawn,
                    color: piece::Color::White,
                }),
            },
            Square {
                file: board::File::E,
                rank: board::Rank::Two,
                piece: Some(Piece {
                    piece: piece::PieceType::Pawn,
                    color: piece::Color::White,
                }),
            },
            Square {
                file: board::File::F,
                rank: board::Rank::Two,
                piece: Some(Piece {
                    piece: piece::PieceType::Pawn,
                    color: piece::Color::White,
                }),
            },
            Square {
                file: board::File::G,
                rank: board::Rank::Two,
                piece: Some(Piece {
                    piece: piece::PieceType::Pawn,
                    color: piece::Color::White,
                }),
            },
            Square {
                file: board::File::H,
                rank: board::Rank::Two,
                piece: Some(Piece {
                    piece: piece::PieceType::Pawn,
                    color: piece::Color::White,
                }),
            },
            Square {
                file: board::File::A,
                rank: board::Rank::Three,
                piece: None,
            },
            Square {
                file: board::File::B,
                rank: board::Rank::Three,
                piece: None,
            },
            Square {
                file: board::File::C,
                rank: board::Rank::Three,
                piece: None,
            },
            Square {
                file: board::File::D,
                rank: board::Rank::Three,
                piece: None,
            },
            Square {
                file: board::File::E,
                rank: board::Rank::Three,
                piece: None,
            },
            Square {
                file: board::File::F,
                rank: board::Rank::Three,
                piece: None,
            },
            Square {
                file: board::File::G,
                rank: board::Rank::Three,
                piece: None,
            },
            Square {
                file: board::File::H,
                rank: board::Rank::Three,
                piece: None,
            },
            Square {
                file: board::File::A,
                rank: board::Rank::Four,
                piece: None,
            },
            Square {
                file: board::File::B,
                rank: board::Rank::Four,
                piece: None,
            },
            Square {
                file: board::File::C,
                rank: board::Rank::Four,
                piece: None,
            },
            Square {
                file: board::File::D,
                rank: board::Rank::Four,
                piece: None,
            },
            Square {
                file: board::File::E,
                rank: board::Rank::Four,
                piece: None,
            },
            Square {
                file: board::File::F,
                rank: board::Rank::Four,
                piece: None,
            },
            Square {
                file: board::File::G,
                rank: board::Rank::Four,
                piece: None,
            },
            Square {
                file: board::File::H,
                rank: board::Rank::Four,
                piece: None,
            },
            Square {
                file: board::File::A,
                rank: board::Rank::Five,
                piece: None,
            },
            Square {
                file: board::File::B,
                rank: board::Rank::Five,
                piece: None,
            },
            Square {
                file: board::File::C,
                rank: board::Rank::Five,
                piece: None,
            },
            Square {
                file: board::File::D,
                rank: board::Rank::Five,
                piece: None,
            },
            Square {
                file: board::File::E,
                rank: board::Rank::Five,
                piece: None,
            },
            Square {
                file: board::File::F,
                rank: board::Rank::Five,
                piece: None,
            },
            Square {
                file: board::File::G,
                rank: board::Rank::Five,
                piece: None,
            },
            Square {
                file: board::File::H,
                rank: board::Rank::Five,
                piece: None,
            },
            Square {
                file: board::File::A,
                rank: board::Rank::Six,
                piece: None,
            },
            Square {
                file: board::File::B,
                rank: board::Rank::Six,
                piece: None,
            },
            Square {
                file: board::File::C,
                rank: board::Rank::Six,
                piece: None,
            },
            Square {
                file: board::File::D,
                rank: board::Rank::Six,
                piece: None,
            },
            Square {
                file: board::File::E,
                rank: board::Rank::Six,
                piece: None,
            },
            Square {
                file: board::File::F,
                rank: board::Rank::Six,
                piece: None,
            },
            Square {
                file: board::File::G,
                rank: board::Rank::Six,
                piece: None,
            },
            Square {
                file: board::File::H,
                rank: board::Rank::Six,
                piece: None,
            },
            Square {
                file: board::File::A,
                rank: board::Rank::Seven,
                piece: Some(Piece {
                    piece: piece::PieceType::Pawn,
                    color: piece::Color::Black,
                }),
            },
            Square {
                file: board::File::B,
                rank: board::Rank::Seven,
                piece: Some(Piece {
                    piece: piece::PieceType::Pawn,
                    color: piece::Color::Black,
                }),
            },
            Square {
                file: board::File::C,
                rank: board::Rank::Seven,
                piece: Some(Piece {
                    piece: piece::PieceType::Pawn,
                    color: piece::Color::Black,
                }),
            },
            Square {
                file: board::File::D,
                rank: board::Rank::Seven,
                piece: Some(Piece {
                    piece: piece::PieceType::Pawn,
                    color: piece::Color::Black,
                }),
            },
            Square {
                file: board::File::E,
                rank: board::Rank::Seven,
                piece: Some(Piece {
                    piece: piece::PieceType::Pawn,
                    color: piece::Color::Black,
                }),
            },
            Square {
                file: board::File::F,
                rank: board::Rank::Seven,
                piece: Some(Piece {
                    piece: piece::PieceType::Pawn,
                    color: piece::Color::Black,
                }),
            },
            Square {
                file: board::File::G,
                rank: board::Rank::Seven,
                piece: Some(Piece {
                    piece: piece::PieceType::Pawn,
                    color: piece::Color::Black,
                }),
            },
            Square {
                file: board::File::H,
                rank: board::Rank::Seven,
                piece: Some(Piece {
                    piece: piece::PieceType::Pawn,
                    color: piece::Color::Black,
                }),
            },
            Square {
                file: board::File::A,
                rank: board::Rank::Eight,
                piece: Some(Piece {
                    piece: piece::PieceType::Rook,
                    color: piece::Color::Black,
                }),
            },
            Square {
                file: board::File::B,
                rank: board::Rank::Eight,
                piece: Some(Piece {
                    piece: piece::PieceType::Knight,
                    color: piece::Color::Black,
                }),
            },
            Square {
                file: board::File::C,
                rank: board::Rank::Eight,
                piece: Some(Piece {
                    piece: piece::PieceType::Bishop,
                    color: piece::Color::Black,
                }),
            },
            Square {
                file: board::File::D,
                rank: board::Rank::Eight,
                piece: Some(Piece {
                    piece: piece::PieceType::Queen,
                    color: piece::Color::Black,
                }),
            },
            Square {
                file: board::File::E,
                rank: board::Rank::Eight,
                piece: Some(Piece {
                    piece: piece::PieceType::King,
                    color: piece::Color::Black,
                }),
            },
            Square {
                file: board::File::F,
                rank: board::Rank::Eight,
                piece: Some(Piece {
                    piece: piece::PieceType::Bishop,
                    color: piece::Color::Black,
                }),
            },
            Square {
                file: board::File::G,
                rank: board::Rank::Eight,
                piece: Some(Piece {
                    piece: piece::PieceType::Knight,
                    color: piece::Color::Black,
                }),
            },
            Square {
                file: board::File::H,
                rank: board::Rank::Eight,
                piece: Some(Piece {
                    piece: piece::PieceType::Rook,
                    color: piece::Color::Black,
                }),
            },
        ];
        let board = Board::default();
        assert_eq!(Board { squares }, board);
    }

    #[test]
    fn test_rank_values() {
        let mut value = 1;
        board::Rank::iter().for_each(|rank| {
            assert_eq!(value, rank.value());
            value += 1;
        });
    }

    #[test]
    fn test_file_values() {
        let mut value = 1;
        board::File::iter().for_each(|file| {
            assert_eq!(value, file.value());
            value += 1;
        });
    }
}

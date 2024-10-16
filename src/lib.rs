mod board;
mod piece;
mod movement;
mod player;


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

    #[test]
    fn test_piece_values() {
        let values = vec![1, 3, 3, 5, 9, 0];
        piece::PieceType::iter().enumerate().for_each(|(i, piece)| {
            assert_eq!(values[i], piece.value());
        });
    }

    #[test]
    fn test_count_material_default() {
        let board = Board::default();
        assert_eq!(39, board.count_material(Color::Black));
        assert_eq!(39, board.count_material(Color::White));
    }

    #[test]
    fn test_get_square() {
        let mut board = Board::default();
        let file = File::E;
        let rank = Rank::Four;
        let mut square = Square {
            file,
            rank,
            piece: None,
        };

        let result = board.get_square(file, rank);
        assert_eq!(&mut square, result.unwrap());
    }

    #[test]
    fn test_make_move() {
        let mut board = Board::default();
        let mut e2 = Square {
            file: File::E,
            rank: Rank::Two,
            piece: None,
        };
        let mut e4 = Square {
            file: File::E,
            rank: Rank::Four,
            piece: Some(Piece {
                piece: PieceType::Pawn,
                color: Color::White,
            }),
        };
        board.make_move(&e4, &e2, None);
        assert_eq!(&mut e2, board.get_square(File::E, Rank::Two).unwrap());
        assert_eq!(&mut e4, board.get_square(File::E, Rank::Four).unwrap());
    }
}

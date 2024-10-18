use chess_lib::board::Board;
use chess_lib::square::{Square, SquareError};
use chess_lib::file::File;
use chess_lib::rank::Rank;
use chess_lib::piece::{Piece, PieceType, Color};
use chess_lib::movement::Move;

#[test]
fn test_get_piece() {
    let board = Board::default();
    let e2 = Square {
        file: File::E,
        rank: Rank::Two,
    };
    let piece = Piece {
        unit: PieceType::Pawn,
        color: Color::White,
    };

    let result = board.get_piece(&e2);
    assert_eq!(&Some(piece), result.unwrap());

    let e4 = Square {
        file: File::E,
        rank: Rank::Four,
    };
    let piece = None;

    let result = board.get_piece(&e4);
    assert_eq!(&piece, result.unwrap());

    let e8 = Square {
        file: File::E,
        rank: Rank::Eight,
    };
    let piece = Piece {
        unit: PieceType::King,
        color: Color::Black,
    };

    let result = board.get_piece(&e8);
    assert_eq!(&Some(piece), result.unwrap());
}

#[test]
fn test_move_piece() {
    let mut board = Board::default();
    let e2 = Square {
        file: File::E,
        rank: Rank::Two,
    };
    let e4 = Square {
        file: File::E,
        rank: Rank::Four,
    };
    let m = Move {
        from: e2,
        to: e4,
        special: None,
    };
    board.make_move(m).unwrap();
    assert_eq!(&None, board.get_piece(&e2).unwrap());
    assert_eq!(&Some(Piece {
        unit: PieceType::Pawn,
        color: Color::White,
    }), board.get_piece(&e4).unwrap());
}

#[test]
fn test_square_from_str() {
    let a1 = Square::from_str("a1").unwrap();
    assert_eq!(Square::new(File::A, Rank::One), a1);
    let e4 = Square::from_str("e4").unwrap();
    assert_eq!(Square::new(File::E, Rank::Four), e4);
    let d5 = Square::from_str("d5").unwrap();
    assert_eq!(Square::new(File::D, Rank::Five), d5);

    assert_eq!(Err(SquareError::CreateFromStr("j9".to_string())), Square::from_str("j9"));
    assert_eq!(Err(SquareError::CreateFromStr("e9".to_string())), Square::from_str("e9"));
    assert_eq!(Err(SquareError::CreateFromStr("x7".to_string())), Square::from_str("x7"));
}

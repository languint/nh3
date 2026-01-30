mod bitboard;
mod castling_rights;

pub use bitboard::Bitboard;
pub use castling_rights::CastlingRights;

use crate::{color::Color, piece::Piece, square::Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    pub bb_pieces: [Bitboard; Piece::ALL.len()],
    pub bb_colors: [Bitboard; Color::ALL.len()],

    pub color_to_move: Color,
    pub en_passant_target_square: Option<Square>,
    pub castling_rights: CastlingRights,
    pub halfmove_clock: u8,
    pub fullmove_clock: u8,
}
impl Board {
    pub const EMPTY: Board = Board {
        bb_pieces: [Bitboard::EMPTY; Piece::ALL.len()],
        bb_colors: [Bitboard::EMPTY; Color::ALL.len()],

        color_to_move: Color::White,
        castling_rights: CastlingRights::NONE,
        en_passant_target_square: None,
        fullmove_clock: 0,
        halfmove_clock: 0,
    };
}

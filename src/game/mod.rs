use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
/// A piece in chess
struct Piece {
    piece: PieceType,
    color: Color,
}

#[derive(Clone, Copy, Debug)]
/// The type of piece in chess
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug)]
/// The color of the piece in chess
enum Color {
    Black,
    White,
}

#[derive(Clone, Debug)]
/// Will contain the pieces on the board and the methods to interact
/// with them
struct Board {
    /// Board will always be 8x8.
    ///
    /// Index 0 represents top-left of the board. Increasing index goes
    /// across the board. Index 8 represents the first tile on the 2nd
    /// row down (but the 7th rank). Everything is from white's perspective.
    position: [Option<Piece>; 64],
}

impl Board {
    /// Changes a coordinate (`x` in `0..8`, `y` in `0..8`) into an index
    /// in `0..64`.
    fn c_to_i(x: usize, y: usize) -> usize {
        y * 8 + x
    }

    /// Checks for a valid coordinate
    fn c_is_valid(x: usize, y: usize) -> bool {
        (0..8).contains(&x) && (0..8).contains(&y)
    }

    pub fn blank() -> Self {
        Self {
            position: [None; 64],
        }
    }

    /// Generate a board position from a Forsyth-Edwards Notation (FEN)
    /// standard string.
    /// Example string:
    /// ```
    /// "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    /// ```
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let mut position = [None; 64];
        let (mut x, mut y) = (0, 0);
        let mut fen: VecDeque<&str> = fen.split(' ').collect();

        let pos = if let Some(pos) = fen.pop_front() {
            pos
        } else {
            return Err("No FEN?".to_owned());
        };

        let mut last_invalid = false;

        for c in pos.bytes() {
            // TODO: verify valid index
            let index = Self::c_to_i(x, y);
            let maybe_piece;
            (maybe_piece, x, y) = match c.to_ascii_lowercase() {
                b'r' => (
                    Some(Piece {
                        piece: PieceType::Rook,
                        color: if c.is_ascii_lowercase() {
                            Color::Black
                        } else {
                            Color::White
                        },
                    }),
                    x + 1,
                    y,
                ),
                b'n' => (
                    Some(Piece {
                        piece: PieceType::Knight,
                        color: if c.is_ascii_lowercase() {
                            Color::Black
                        } else {
                            Color::White
                        },
                    }),
                    x + 1,
                    y,
                ),
                b'b' => (
                    Some(Piece {
                        piece: PieceType::Bishop,
                        color: if c.is_ascii_lowercase() {
                            Color::Black
                        } else {
                            Color::White
                        },
                    }),
                    x + 1,
                    y,
                ),
                b'q' => (
                    Some(Piece {
                        piece: PieceType::Queen,
                        color: if c.is_ascii_lowercase() {
                            Color::Black
                        } else {
                            Color::White
                        },
                    }),
                    x + 1,
                    y,
                ),
                b'k' => (
                    Some(Piece {
                        piece: PieceType::King,
                        color: if c.is_ascii_lowercase() {
                            Color::Black
                        } else {
                            Color::White
                        },
                    }),
                    x + 1,
                    y,
                ),
                b'p' => (
                    Some(Piece {
                        piece: PieceType::Pawn,
                        color: if c.is_ascii_lowercase() {
                            Color::Black
                        } else {
                            Color::White
                        },
                    }),
                    x + 1,
                    y,
                ),
                b'/' => (None, 0, y + 1),
                b'1'..=b'8' => (None, (c - b'1' + 1) as usize, y),
                _ => return Err("Invalid char in FEN".to_owned()),
            };
            if let Some(_) = maybe_piece {
                if last_invalid {
                    return Err(format!("Invalid coordinate reached: {}, {}", x, y));
                } else {
                    position[index] = maybe_piece;
                }
            }
            // Check new x and y for validity
            last_invalid = if !Self::c_is_valid(x, y) {
                true
            } else {
                false
            };
        }
        Ok(Self { position })
    }
}

/// Will contain the move history, position, next turn, etc.
struct Game {
    /// The board position.
    board: Board,
    /// The color of the next player.
    turn: Color,
    /// The numbered full turn the game started on. Defaults to 1.
    start_turn: usize,
    /// The numbered half turn that this turn represents. Starts at 1.
    half_turn_num: usize,
    /// The numbered full turn that this turn represents. Starts at 1 and
    /// increments after black's turn.
    full_turn_num: usize,
    /// A tracker for who can castle where. The [`char`]s must be in
    /// `['K', 'k', 'Q', 'q']`.
    castling: Vec<char>,
    /// A tracker for the moves counting toward the fifty move rule
    fifty_move_rule: usize,
}

/// Generate a chess game from a Forsyth-Edwards Notation (FEN)
/// standard string.
/// Example string:
/// ```
/// "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
/// ```
impl Game {
    pub fn from_fen(fen: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let board = Board::from_fen(fen)?;

        let mut fen: VecDeque<&str> = fen.split(' ').collect();

        // Whose turn it is to play
        let turn = match fen.pop_front() {
            Some(turn) if turn != "b" && turn != "w" => return Err("Invalid turn specifier".into()),
            Some(turn) if turn == "b" => Color::Black,
            _ => Color::White,
        };

        let castling = match fen.pop_front() {
            Some("-") => vec![],
            Some(part) => {
                let mut castling = vec![];
                for c in part.chars() {
                    if "KQkq".contains(c) {
                        castling.push(c);
                    } else {
                        return Err(format!("Invalid char in castling part: {}", c).into());
                    }
                }
                castling
            }
            _ => vec!['K', 'Q', 'k', 'q'],
        };

        // TODO: parse en passant square
        let _en_passant_square = fen.pop_front();

        let fifty_move_rule: usize = fen.pop_front().unwrap_or("0").parse()?;

        let full_turn_num: usize = fen.pop_front().unwrap_or("0").parse()?;
        let half_turn_num = match turn {
            Color::Black => full_turn_num * 2 + 1,
            Color::White => full_turn_num * 2,
        };

        Ok(Self {
            board,
            turn,
            start_turn: full_turn_num,
            half_turn_num,
            full_turn_num,
            fifty_move_rule,
            castling,
        })
    }
}

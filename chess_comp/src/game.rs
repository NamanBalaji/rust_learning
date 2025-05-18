use std::collections::VecDeque;

use bitflags::bitflags;

pub fn index_to_position(index: usize) -> String {
    let col = index % 8;
    let col_char = (b'a' + col as u8) as char;
    let row = index / 8 + 1;

    return format!("{}{}", col_char, row);
}

pub fn position_to_bit(position: &str) -> Result<u64, String> {
    if position.len() != 2 {
        return Err(format!("Invalid length: {}, string: '{position}'", position.len()));
    }

    let bytes = position.as_bytes();
    let b0 = bytes[0];
    if b0 < b'a' || b0 > b'h' {
        return Err(format!("Invalid column charcter: {}, string: '{position}'", b0 as char));
    } 

    let column = (b0 - b'a') as u32;
    let row;

    if let Some(num) = (bytes[1] as char).to_digit(10) {
        if num < 1 || num > 8 {
            return Err(format!("Invalid row character: {}, string: '{position}'", bytes[1] as char));   
        } else {
            row = num - 1;
            let square_num = row * 8 + column;
            let bit = (1 as u64) << square_num;
            return Ok(bit);
        }
    } else {
        return Err(format!("Invalid row character: {}, string: '{position}'", bytes[1] as char));   
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black
}

#[derive(Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

#[derive(Debug, PartialEq)]
pub struct Piece {
    position: u64,
    color: Color,
    piece_type: PieceType
}

impl Piece {
    fn to_string(&self) -> String {
        let mut result = "".to_string();
        result.push_str(match self.piece_type {
            PieceType::Pawn => "p ",
            PieceType::Rook => "r ",
            PieceType::Knight => "n ",
            PieceType::Bishop => "b ",
            PieceType::Queen => "q ",
            PieceType::King => "k ",
        });

        if self.color == Color::White {
            result.make_ascii_uppercase();
        }

        result
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Square {
    Empty,
    Occupied(usize),
}

bitflags! {
    pub struct CastlingRights: u8 {
        const NONE = 0;
        const WHITE_KING_SIDE = 1 << 0;
        const WHITE_QUEEN_SIDE = 1 << 1; 
        const BLACK_KING_SIDE = 1 << 2;
        const BLACK_QUEEN_SIDE = 1 << 3;
        const ALL = Self::WHITE_KING_SIDE.bits() | Self::WHITE_QUEEN_SIDE.bits() | Self::BLACK_KING_SIDE.bits() | Self::BLACK_QUEEN_SIDE.bits();
    }
}

pub struct Game {
    pieces: Vec<Piece>,
    squares: Vec<Square>,
    active_color: Color,
    castling_rights: CastlingRights,
    en_passant: Option<u64>,
    halfmove_clock: usize,
    fullmove_number: usize
}

impl Game {
    fn push_piece_and_square(&mut self, position: usize, color: Color, piece_type: PieceType, index: &mut usize) {
        self.pieces.push(Piece { 
            position: (1 as u64) << position, 
            color: color, 
            piece_type: piece_type 
        });
        self.squares.push(Square::Occupied(*index));
        
        *index += 1;
    }

    fn push_empty_square(&mut self) {
        self.squares.push(Square::Empty);
    }

    pub fn initalize() -> Self {
        Self::read_FEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn to_string(&self) -> String {
        let mut board = "".to_string();
        let mut temp = "".to_string();

        for (i, square) in self.squares.iter().enumerate() {
            match square {
                Square::Empty => { temp.push_str(&index_to_position(i)) },
                Square::Occupied(idx) => temp.push_str(&self.pieces[*idx].to_string()),
            }

            if (i + 1) % 8 == 0 {
                temp.push_str("\n");
                board.insert_str(0, &temp);
                temp.clear();
            } else {
                temp.push(' ');
            }
        }

        board
    }

    #[allow(non_snake_case)]
    pub fn read_FEN(fen: &str) -> Self {
        let mut game = Game {
            pieces: vec![], 
            squares: vec![],
            castling_rights: CastlingRights::ALL,
            active_color: Color::White,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1 
        }; 
    
        let mut split = fen.split(" ");
        let positions = split.next().unwrap();
        
        let mut piece_index = 0;
        let mut piece_position = 64;
        let mut dequeu_squares = VecDeque::new();

        for row in positions.splitn(8, |ch| ch == '/') {
            piece_position -= 8;
            let (pieces, squares) = parse_row(row, piece_index, piece_position);
            for p in pieces {
                game.pieces.push(p);
                piece_index += 1;
            }
            for s in squares {
                dequeu_squares.push_front(s);
            }
        }

        game.squares = Vec::from(dequeu_squares);

        let color_to_move = split.next().unwrap();
        game.active_color = match color_to_move {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Unknown color: {color_to_move}")
        };

        let castling_rights = split.next().unwrap();
        let mut castling = CastlingRights::NONE;
        for ch in castling_rights.chars() {
            match ch {
                'K' => castling |= CastlingRights::WHITE_KING_SIDE,
                'Q' => castling |= CastlingRights::WHITE_QUEEN_SIDE,
                'k' => castling |= CastlingRights::BLACK_KING_SIDE,
                'q' => castling |= CastlingRights::BLACK_QUEEN_SIDE,
                '-' => (),
                _ => panic!("Invalid character in castling rights '{ch}'"),
            }
        }
        game.castling_rights = castling;

        let en_passant = split.next().unwrap();
        match en_passant {
            "-" => game.en_passant = None,
            s => {
                let pos = position_to_bit(s).unwrap_or_else(|e| panic!("{e}"));
                game.en_passant = Some(pos);
            }
        }

        let halfmove_clock = split.next().unwrap();
        game.halfmove_clock = halfmove_clock.parse().unwrap_or_else(|_| panic!("Invalid halfmove: {}", halfmove_clock));
        
        let full_move_number = split.next().unwrap();
        game.fullmove_number = full_move_number.parse().unwrap_or_else(|_| panic!("Invalid ful move number: {}", full_move_number));

        game
    }
}

fn parse_row(row: &str, mut piece_index: usize, mut piece_position: usize) -> (Vec<Piece>, Vec<Square>) {
    let mut pieces = Vec::new();
    let mut squares = VecDeque::new();

    for ch in row.chars() {
        if let Some(num) = ch.to_digit(10) {
            for _ in 0..num {
                squares.push_front(Square::Empty);
                piece_position += 1;
            }
            continue;                     
        }

        // Otherwise it must be a piece letter
        let color = if ch.is_ascii_uppercase() { Color::White } else { Color::Black };
        let piece_type = match ch.to_ascii_lowercase() {
            'r' => PieceType::Rook,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
            'p' => PieceType::Pawn,
            _   => panic!("Invalid piece char: {ch}"),
        };

        let piece = Piece {
            color,
            piece_type,
            position: 1u64 << piece_position,
        };
        pieces.push(piece);
        squares.push_front(Square::Occupied(piece_index));

        piece_index += 1;
        piece_position += 1;
    }

    (pieces, Vec::from(squares))
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_initial_position() -> Game {
        let mut game = Game { pieces: vec![], squares: vec![],
                              active_color: Color::White,
                              castling_rights: CastlingRights::ALL,
                              en_passant: None,
                              halfmove_clock: 0,
                              fullmove_number: 1
        };
        let mut piece_index = 0;

        let color = Color::White;

        game.push_piece_and_square(0, color,
                                   PieceType::Rook, &mut piece_index);
        game.push_piece_and_square(1, color,
                                   PieceType::Knight, &mut piece_index);
        game.push_piece_and_square(2, color,
                                   PieceType::Bishop, &mut piece_index);
        game.push_piece_and_square(3, color,
                                   PieceType::Queen, &mut piece_index);
        game.push_piece_and_square(4, color,
                                   PieceType::King, &mut piece_index);
        game.push_piece_and_square(5, color,
                                   PieceType::Bishop, &mut piece_index);
        game.push_piece_and_square(6, color,
                                   PieceType::Knight, &mut piece_index);
        game.push_piece_and_square(7, color,
                                   PieceType::Rook, &mut piece_index);

        for i in 8..16 {
            game.push_piece_and_square(i, color,
                                       PieceType::Pawn, &mut piece_index);
        }

        for _ in 16..48 {
            game.push_empty_square();
        }

        let color = Color::Black;
        for i in 48..56 {
            game.push_piece_and_square(i, color,
                                       PieceType::Pawn, &mut piece_index);
        }        

        let offset = 56;
        game.push_piece_and_square(0 + offset, color,
                                   PieceType::Rook, &mut piece_index);
        game.push_piece_and_square(1 + offset, color,
                                   PieceType::Knight, &mut piece_index);
        game.push_piece_and_square(2 + offset, color,
                                   PieceType::Bishop, &mut piece_index);
        game.push_piece_and_square(3 + offset, color,
                                   PieceType::Queen, &mut piece_index);
        game.push_piece_and_square(4 + offset, color,
                                   PieceType::King, &mut piece_index);
        game.push_piece_and_square(5 + offset, color,
                                   PieceType::Bishop, &mut piece_index);
        game.push_piece_and_square(6 + offset, color,
                                   PieceType::Knight, &mut piece_index);
        game.push_piece_and_square(7 + offset, color,
                                   PieceType::Rook, &mut piece_index);
                
        
        game
    }


    #[test]
    fn read_initial_position() {
        let game = Game::initalize();
        let default = get_initial_position();
        assert_eq!(game.active_color, Color::White);
        assert_eq!(game.castling_rights.bits(), CastlingRights::ALL.bits());
        assert_eq!(game.en_passant, None);
        assert_eq!(game.halfmove_clock, 0);
        assert_eq!(game.fullmove_number, 1);
        for i in 0..64 {
            match (game.squares[i], default.squares[i]) {
                (Square::Empty, Square::Empty) => (),
                (Square::Occupied(idx1), Square::Occupied(idx2)) => assert_eq!(game.pieces[idx1], default.pieces[idx2]),
                 _ => panic!("Wrong square at index {}", i),
            }
        }
    }

    #[test]
    fn read_fen_black_active() {
        let game = Game::read_FEN("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b - - 1 2");
        assert_eq!(game.active_color, Color::Black);
    }   

    #[test]
    fn read_fen_no_castling() {
        let game = Game::read_FEN("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b - - 1 2");
        assert_eq!(game.castling_rights.bits(), CastlingRights::NONE.bits());
    }

    #[test]
    fn read_fen_en_passant_allowed() {
        let en_passant_square = "g7";
        let game = Game::read_FEN(&format!("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq {} 1 2", en_passant_square));
        assert_eq!(game.en_passant, Some(position_to_bit(en_passant_square).unwrap()));
    }

    #[test]
    fn read_fen_moveclocks() {
        let game = Game::read_FEN("rnbqkbnr/pp1ppppp/7P/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b - g7 1 2");
        assert_eq!(game.halfmove_clock, 1);
        assert_eq!(game.fullmove_number, 2);
    }

    #[test]
    fn read_all_possible_castling_rights() {
        let mut rights = "".to_owned(); 
        let right_chars = ["K", "Q", "k", "q"];
        for i in 0..(u8::pow(2, 4)) {
            let bitflag_rights = CastlingRights::from_bits(i).unwrap();
            for j in 0..4 {
                if (i >> j) & 1 != 0 {
                    rights.push_str(right_chars[j]);
                }
            }
            let fen = format!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w {} - 0 1", rights);
            let game = Game::read_FEN(&fen);
            assert_eq!(game.castling_rights.bits(), bitflag_rights.bits(), "FEN: {}\n\n i: {}", fen, i);
            rights.clear();
        }
    }
}

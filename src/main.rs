use std::{i32, ops::AddAssign, str::FromStr, time::Instant};

use chess::{Board, BoardStatus, CacheTable, ChessMove, Color, MoveGen, Piece, EMPTY};




fn _count_material(board : Board, color : Color) -> i32 {
    let mut total = 0;

    let pieces = board.color_combined(color);
    let pawns = board.pieces(Piece::Pawn);
    let knights = board.pieces(Piece::Knight);
    let bishops = board.pieces(Piece::Bishop);
    let rooks = board.pieces(Piece::Rook);
    let queens = board.pieces(Piece::Queen);

    total += (pieces & pawns).popcnt() * 1;
    total += (pieces & knights).popcnt() * 3;
    total += (pieces & bishops).popcnt() * 3;
    total += (pieces & rooks).popcnt() * 5;
    total += (pieces & queens).popcnt() * 8;
    
    return total as i32
}

const BLACK_PAWN_TABLE : [i8; 64] = [
0, 0, 0, 0, 0, 0, 0, 0,
50, 50, 50, 50, 50, 50, 50, 50,
15, 15, 20, 25, 25, 20, 15, 15,
5, 5, 5, 15, 15, 5, 5, 5,
0, 0, 0, 20, 20, 0, 0, 0,
5, 5, 5, 10, 10, 5, 5, 5,
5, 5, 10, -5, -5, 10, 5, 5,
0, 0, 0, 0, 0, 0, 0, 0,
];
const WHITE_PAWN_TABLE : [i8; 64] = [
0, 0, 0, 0, 0, 0, 0, 0,
5, 5, 10, -5, -5, 10, 5, 5,
5, 5, 5, 10, 10, 5, 5, 5,
0, 0, 0, 20, 20, 0, 0, 0,
5, 5, 5, 15, 15, 5, 5, 5,
15, 15, 20, 25, 25, 20, 15, 15,
50, 50, 50, 50, 50, 50, 50, 50,
0, 0, 0, 0, 0, 0, 0, 0,
];

const BLACK_KNIGHT_TABLE : [i8; 64] = [
-25, -20, -15, -10, -10, -15, -20, -25,
-20, -5, 0, 0, 0, 0, -5, -20,
-15, 0, 5, 10, 10, 5, 0, -15,
-10, 0, 0, 20, 20, 0, 0, -10,
-10, 0, 5, 20, 20, 5, 5, -10,
-15, 5, 15, 10, 10, 15, 5, -15,
-20, -5, 0, 5, 5, 0, -5, -20,
-25, -5, 0, 0, 0, 0, -5, -25,
];
const WHITE_KNIGHT_TABLE : [i8; 64] = [
-25, -5, 0, 0, 0, 0, -5, -25,
-20, -5, 0, 5, 5, 0, -5, -20,
-15, 5, 15, 10, 10, 15, 5, -15,
-10, 0, 5, 20, 20, 5, 5, -10,
-10, 0, 0, 20, 20, 0, 0, -10,
-15, 0, 5, 10, 10, 5, 0, -15,
-20, -5, 0, 0, 0, 0, -5, -20,
-25, -20, -15, -10, -10, -15, -20, -25,
];
const BLACK_BISHOP_TABLE : [i8; 64] = [
0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0,
-10, 0, -5, 0, 0, -5, 0, -10,
-5, 10, 5, 10, 10, 5, 10, -5,
-5, 0, 15, 10, 10, 15, 0, -5,
0, 5, 5, 10, 10, 5, 5, 0,
5, 10, 0, 5, 5, 0, 10, 5,
0, -5, -10, 0, 0, -10, -5, 0,
];
const WHITE_BISHOP_TABLE : [i8; 64] = [
0, -5, -10, 0, 0, -10, -5, 0,
5, 10, 0, 5, 5, 0, 10, 5,
0, 5, 5, 10, 10, 5, 5, 0,
-5, 0, 15, 10, 10, 15, 0, -5,
-5, 10, 5, 10, 10, 5, 10, -5,
-10, 0, -5, 0, 0, -5, 0, -10,
0, 0, 0, 0, 0, 0, 0, 0,
0, 0, 0, 0, 0, 0, 0, 0,
];
const BLACK_ROOK_TABLE : [i8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    20, 20, 20, 20, 20, 20, 20, 20,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 10, 10, 5, 0, 0
];
const WHITE_ROOK_TABLE : [i8; 64] = [
    0, 0, 0, 10, 10, 5, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    20, 20, 20, 20, 20, 20, 20, 20,
    0, 0, 0, 0, 0, 0, 0, 0
];

fn eval(board : Board) -> i32 {

    let mut eval= 0i32;

    let white_pieces = board.color_combined(Color::White);
    let white_pawns = board.pieces(Piece::Pawn) & white_pieces;
    let white_knights = board.pieces(Piece::Knight) & white_pieces;
    let white_bishops = board.pieces(Piece::Bishop) & white_pieces;
    let white_rooks = board.pieces(Piece::Rook) & white_pieces;
    let white_queens = board.pieces(Piece::Queen) & white_pieces;


    for square in white_pawns {
        eval += 100 + WHITE_PAWN_TABLE[square.to_index()] as i32;
    }
    for square in white_knights {
        eval += 300 + WHITE_KNIGHT_TABLE[square.to_index()] as i32;
    }
    for square in white_bishops {
        eval += 300 + WHITE_BISHOP_TABLE[square.to_index()] as i32;
    }
    for square in white_rooks {
        eval += 500 + WHITE_ROOK_TABLE[square.to_index()] as i32;
    }
    eval += white_queens.popcnt() as i32 * 800;

    let black_pieces = board.color_combined(Color::Black);
    let black_pawns = board.pieces(Piece::Pawn) & black_pieces;
    let black_knights = board.pieces(Piece::Knight) & black_pieces;
    let black_bishops = board.pieces(Piece::Bishop) & black_pieces;
    let black_rooks = board.pieces(Piece::Rook) & black_pieces;
    let black_queens = board.pieces(Piece::Queen) & black_pieces;


    for square in black_pawns {
        eval -= 100 + BLACK_PAWN_TABLE[square.to_index()] as i32;
    }
    for square in black_knights {
        eval -= 300 + BLACK_KNIGHT_TABLE[square.to_index()] as i32;
    }
    for square in black_bishops {
        eval -= 300 + BLACK_BISHOP_TABLE[square.to_index()] as i32;
    }
    for square in black_rooks {
        eval += 500 + BLACK_ROOK_TABLE[square.to_index()] as i32;
    }
    eval -= black_queens.popcnt() as i32 * 800;


    return eval
}

fn minimax(board : Board, depth : i8, _alpha : Option<i32>, beta : Option<i32>, prev_best : ChessMove, color : Color, time : Instant, target: u128, _capture : bool, cache : &mut CacheTable<(ChessMove, i32, i8)>, i : &mut u32) -> (ChessMove, i32) {
    
    // let bs = board.to_string();
    // let debug = bs.as_str();
    // if depth == 0 {
    //     println!("{debug}");
    // }
    if let Some((_c_mv, c_eval, c_depth)) = cache.get(board.get_hash()) {
        // i.add_assign(1);
        // let new_depth_eval = negamax(board.make_move_new(mv), depth - 1, alpha, beta, ChessMove::default(), color, time, target, false, cache, i);
        if c_depth == depth {
            return (ChessMove::default(), c_eval);
        }
    }
    if board.status() == BoardStatus::Checkmate {
        // println!("{}", board.to_string());
        let sign = match color {
            Color::White => 1,
            Color::Black => -1,
        };
        match board.side_to_move() {
            Color::White => return (ChessMove::default(), sign * -999),
            Color::Black => return (ChessMove::default(), sign * 999),
        }
    }
    if board.status() == BoardStatus::Stalemate {
        return (ChessMove::default(), 0)
    }
    if (depth <= 0) || time.elapsed().as_millis() > 1000000 {
        match color {
            Color::White => return (ChessMove::default(), eval(board)),
            Color::Black => return (ChessMove::default(), -eval(board)),
        }
    }

    let mut moves = MoveGen::new_legal(&board);

    match board.side_to_move() {
        Color::White => {
            moves.set_iterator_mask(*board.color_combined(Color::Black));
        },
        Color::Black => {
            moves.set_iterator_mask(*board.color_combined(Color::White));
        }
    }

    let mut maxxed = -999;
    let mut candidate = ChessMove::default();
    let mut minned;
    let mut best_response = ChessMove::default();
    let mut response;
    for mv in moves.by_ref() {
        // println!("{}", mv.to_string());
        i.add_assign(1);
        let res_board = board.make_move_new(mv);
        (response, minned) = negamax(res_board, depth - 1, Some(maxxed), beta, best_response, color, time, target, true, cache, i);
        if minned > maxxed {
            maxxed = minned;
            candidate = mv;
            best_response = response;
            if beta.is_some_and(|val| maxxed > val) {
                // print!("A");
                return (best_response, maxxed)
            }
        }
    }
    if depth > 0 {
        moves.set_iterator_mask(!EMPTY);
    }
    let mut rem  = moves.collect::<Vec<ChessMove>>();
    if rem.contains(&prev_best) {
        rem.push(prev_best);
    }
    rem.reverse();
    best_response = ChessMove::default();
    for mv in rem {
        // println!("{}", mv.to_string());
        i.add_assign(1);
        (response, minned) = negamax(board.make_move_new(mv), depth - 1, Some(maxxed), beta, best_response, color, time, target, false, cache, i);
        if minned > maxxed {
            maxxed = minned;
            candidate = mv;
            best_response = response;
            if beta.is_some_and(|val| maxxed > val) {
                // print!("A");
                return (best_response, maxxed)
            }
        }
    }
    cache.add(board.get_hash(), (candidate, maxxed, depth));
    // let s = debug.as_ptr();
    return (candidate, maxxed)
}

fn negamax(board : Board, depth : i8, alpha : Option<i32>, _beta : Option<i32>, prev_best : ChessMove, color : Color, time : Instant, target: u128, _capture : bool, cache : &mut CacheTable<(ChessMove, i32, i8)>, i : &mut u32) -> (ChessMove, i32) {
    // let bs = board.to_string();
    // let debug = bs.as_str();
    if let Some((_c_mv, c_eval, c_depth)) = cache.get(board.get_hash()) {
        // i.add_assign(1);
        if c_depth == depth {
            return (ChessMove::default(), c_eval);
        }
    }
    if board.status() == BoardStatus::Checkmate {
        // println!("{}", board.to_string());
        let sign = match color {
            Color::White => 1,
            Color::Black => -1,
        };
        match board.side_to_move() {
            Color::White => return (ChessMove::default(), sign * -999),
            Color::Black => return (ChessMove::default(), sign * 999),
        }
    }
    if board.status() == BoardStatus::Stalemate {
        return (ChessMove::default(), 0)
    }
    if (depth <= 0) || time.elapsed().as_millis() > target {
        match color {
            Color::White => return (ChessMove::default(), eval(board)),
            Color::Black => return (ChessMove::default(), -eval(board)),
        }
    }
    if depth <= 0 {
        // println!("{}", board.to_string());
    }

    let mut moves = MoveGen::new_legal(&board);

    match board.side_to_move() {
        Color::White => {
            moves.set_iterator_mask(*board.color_combined(Color::Black));
        },
        Color::Black => {
            moves.set_iterator_mask(*board.color_combined(Color::White));
        }
    }
    
    let mut minned = 999;
    let mut candidate = ChessMove::default();
    let mut maxxed;
    let mut best_response = ChessMove::default();
    let mut response;

    for mv in moves.by_ref() {
        i.add_assign(1);
        // println!("{}", mv.to_string());
        (response, maxxed) = minimax(board.make_move_new(mv), depth - 1, alpha, Some(minned), best_response, color, time, target, true, cache, i);
        if maxxed < minned {
            minned = maxxed;
            candidate = mv;
            best_response = response;
            // if minned <= -500.0 {
            //     println!("{}", mv.to_string());
            // }
            if alpha.is_some_and(|val| minned < val) {
                // print!("B");
                return (candidate, minned)
            }
        }
    }
    if depth > 0 {
        moves.set_iterator_mask(!EMPTY);
    }
    let mut rem  = moves.collect::<Vec<ChessMove>>();
    if rem.contains(&prev_best) {
        rem.push(prev_best);
    }
    rem.reverse();
    best_response = ChessMove::default();
    for mv in rem {
        i.add_assign(1);
        // println!("{}", mv.to_string());
        (response, maxxed) = minimax(board.make_move_new(mv), depth - 1, alpha, Some(minned), best_response, color, time, target, false, cache, i);
        if maxxed < minned {
            minned = maxxed;
            candidate = mv;
            best_response = response;
            // if minned > 3.4 && minned < 3.5 {
            //     println!("{}, {}", mv.to_string(), board.to_string());
            // }
            if alpha.is_some_and(|val| minned < val) {
                // print!("B");
                return (candidate, minned)
            }
        }
    }
    cache.add(board.get_hash(), (candidate, minned, depth));
    // let s = debug.as_ptr();
    return (candidate, minned);
}

fn iter_deepen (board : Board, target: u128) -> ChessMove {
    let now = Instant::now();
    let mut table = CacheTable::new(65536 * 2 * 2 * 2, (ChessMove::default(), 0i32, 0i8));
    let mut i = 0;
    let mut depth = 0;
    
    let mut eval;
    let mut prev_best = ChessMove::default();
    let mut candidate = prev_best;
    loop {
        depth += 1;
        (prev_best, eval) = minimax(board, depth, Some(i32::MIN), Some(i32::MAX), prev_best, board.side_to_move(), now, target, false, &mut table, &mut i);
        // println!("next: {next}");
        if now.elapsed().as_millis() > target {
            println!("depth: {depth}");
            break;
        }
        candidate = prev_best;
    }
    
    println!("eval: {}", eval);
    println!("i: {i}");
    return candidate
}

fn main() {
    let board = Board::default();
    // let move_gen = MoveGen::new_legal(&board);
    let depth = 7;
    
    let mut table = CacheTable::new(4194304 * 2 * 2, (ChessMove::default(), 0i32, 0i8));
    let total = Instant::now();
    let now = Instant::now();
    let target = 10000;
    let mut i = 0;
    let test_board = board.make_move_new(ChessMove::from_str("d2d4").unwrap()).make_move_new(ChessMove::from_str("g8f6").unwrap()).make_move_new(ChessMove::from_str("d4d5").unwrap()).make_move_new(ChessMove::from_str("d7d6").unwrap());
    // let test_board = Board::from_str("rnb1k2r/ppp2ppp/4qn2/2b1p3/3P4/2N2N2/PPP1BPPP/R1BQ1RK1 b kq - 0 7").unwrap();
    // let test_board = board.make_move_new(ChessMove::from_str("d2d4").unwrap());
    println!("{}", minimax(test_board, depth, Some(i32::MIN), Some(i32::MAX), ChessMove::default(), Color::White, now, target, false, &mut table, &mut i).1);
    println!("{}", i);
    
    // println!("{}", now.elapsed().as_millis());
    println!("{}", total.elapsed().as_millis());

    let now = Instant::now();
    println!("{}", iter_deepen(board, target));

    println!("{}", now.elapsed().as_millis());
}

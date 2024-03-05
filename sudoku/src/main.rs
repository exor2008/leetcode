use std::collections::{HashMap, HashSet};

macro_rules! min_idx {
    ($vc: ident) => {
        $vc.iter()
            .enumerate()
            .min_by_key(|(_i, &val)| if val == 0 { 10 } else { val })
            .map(|(idx, _value)| idx)
            .unwrap()
    };
}

macro_rules! min {
    ($vc: ident) => {
        $vc.iter()
            .min_by_key(|&val| if *val == 0 { 10 } else { *val })
            .unwrap()
    };
}

macro_rules! all_zero {
    ($vc: ident) => {
        $vc.iter().all(|element| *element == 0)
    };
}

struct WalkHor {
    hor: usize,
    vert: usize,
}

impl WalkHor {
    fn new(vert: usize) -> Self {
        Self { hor: 0, vert }
    }

    fn from_ij(i: usize, _j: usize) -> Self {
        Self::new(i)
    }
}

impl Iterator for WalkHor {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let hor = self.hor;
        match hor {
            0..=8 => {
                self.hor += 1;
                Some((self.vert, hor))
            }
            _ => None,
        }
    }
}

struct WalkVert {
    hor: usize,
    vert: usize,
}

impl WalkVert {
    fn new(hor: usize) -> Self {
        Self { hor, vert: 0 }
    }

    fn from_ij(_i: usize, j: usize) -> Self {
        Self::new(j)
    }
}

impl Iterator for WalkVert {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let vert = self.vert;
        match vert {
            0..=8 => {
                self.vert += 1;
                Some((vert, self.hor))
            }
            _ => None,
        }
    }
}

struct WalkSquare {
    idx: usize,
    counter: usize,
}

impl WalkSquare {
    fn new(idx: usize) -> Self {
        Self { idx, counter: 0 }
    }

    fn from_ij(i: usize, j: usize) -> Self {
        let idx = (i / 3) * 3 + j / 3;
        Self::new(idx)
    }
}

impl Iterator for WalkSquare {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let counter = self.counter;
        match counter {
            0..=8 => {
                self.counter += 1;
                let vert = self.idx / 3 * 3 + counter % 3;
                let hor = self.idx % 3 * 3 + counter / 3;

                Some((vert, hor))
            }
            _ => None,
        }
    }
}

enum Walker {
    Hor(WalkHor),
    Vert(WalkVert),
    Square(WalkSquare),
}

impl Iterator for Walker {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Hor(hor) => hor.next(),
            Self::Vert(vert) => vert.next(),
            Self::Square(square) => square.next(),
        }
    }
}

fn main() {
    let mut sudoku = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    solve_sudoku(&mut sudoku)
}

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut h_empty = [0u8; 9];
    let mut v_empty = [0u8; 9];
    let mut s_empty = [0u8; 9];
    let mut memory = HashMap::new();

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == '.' {
                h_empty[i] += 1;
                v_empty[j] += 1;
                s_empty[(i / 3) * 3 + j / 3] += 1;
            }
        }
    }

    print_matrix(board);

    while !all_zero!(h_empty) && !all_zero!(v_empty) && !all_zero!(s_empty) {
        // for _ in 0..5 {
        let h_fullness = min!(h_empty);
        let v_fullness = min!(v_empty);
        let s_fullness = min!(s_empty);

        let fullness = [*h_fullness, *v_fullness, *s_fullness];

        let walker = match min_idx!(fullness) {
            0 => {
                let idx = min_idx!(h_empty);
                println!("Walk hor {}", idx);
                Walker::Hor(WalkHor::new(idx))
            }
            1 => {
                let idx = min_idx!(v_empty);
                println!("Walk vert {}", idx);
                Walker::Vert(WalkVert::new(idx))
            }
            2 => {
                let idx = min_idx!(s_empty);
                println!("Walk square {}", idx);
                Walker::Square(WalkSquare::new(idx))
            }
            _ => unreachable!(),
        };

        for (i, j) in walker {
            if board[i][j] == '.' && !memory.contains_key(&(i, j)) {
                let pretendents = get_pretendents(&board, i, j);

                match pretendents.len() {
                    1 => {
                        let p = pretendents.into_iter().next().unwrap();
                        update_board(p, i, j, board, &mut memory);
                    }
                    _ => {
                        println!("Memory insert {:?}, into {} {}", pretendents, i, j);
                        memory.insert((i, j), pretendents);
                    }
                };
                update_empty(i, j, &mut h_empty, &mut v_empty, &mut s_empty);
            }
        }
        print_matrix(board);
        println!("{:?}", memory);
    }
}

fn get_pretendents(board: &Vec<Vec<char>>, i: usize, j: usize) -> HashSet<char> {
    let mut pretendents: HashSet<char> = ('1'..='9').collect();

    for walker in [
        Walker::Hor(WalkHor::from_ij(i, j)),
        Walker::Vert(WalkVert::from_ij(i, j)),
        Walker::Square(WalkSquare::from_ij(i, j)),
    ] {
        reduce_pretendents(&mut pretendents, walker, &board, i, j);
    }

    pretendents
}

fn reduce_pretendents(
    pretendents: &mut HashSet<char>,
    walker: Walker,
    board: &Vec<Vec<char>>,
    i_orig: usize,
    j_orig: usize,
) {
    for (i, j) in walker {
        if (i_orig, j_orig) != (i, j) {
            pretendents.remove(&board[i][j]);
        }
    }
}

fn update_empty(
    i: usize,
    j: usize,
    h_empty: &mut [u8; 9],
    v_empty: &mut [u8; 9],
    s_empty: &mut [u8; 9],
) {
    h_empty[i] -= 1;
    v_empty[j] -= 1;
    s_empty[(i / 3) * 3 + j / 3] -= 1;

    println!(
        "Update empty - h: {:?} v: {:?}, s:{:?}",
        h_empty, v_empty, s_empty
    );
}

fn print_matrix(m: &Vec<Vec<char>>) {
    for v in m.iter() {
        println!("{:?}", v);
    }
}

fn update_board(
    c: char,
    i: usize,
    j: usize,
    board: &mut Vec<Vec<char>>,
    memory: &mut HashMap<(usize, usize), HashSet<char>>,
) {
    println!("update board {} into {} {}", c, i, j);
    board[i][j] = c;

    for walker in [
        Walker::Hor(WalkHor::from_ij(i, j)),
        Walker::Vert(WalkVert::from_ij(i, j)),
        Walker::Square(WalkSquare::from_ij(i, j)),
    ] {
        for (i, j) in walker {
            if let Some(mut pretendents) = memory.remove(&(i, j)) {
                if pretendents.len() > 1 {
                    println!("Removing {} from {} {} {:?}", c, i, j, pretendents);
                    pretendents.remove(&c);
                }

                if pretendents.len() == 1 {
                    let p = pretendents.iter().next().unwrap();
                    memory.remove(&(i, j));
                    update_board(*p, i, j, board, memory);
                } else {
                    memory.insert((i, j), pretendents);
                }
            }
        }
    }
}

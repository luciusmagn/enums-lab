#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Hidden,
    Flagged,
    Revealed(u8),
    Exploded,
}

#[derive(Debug, PartialEq, Eq)]
enum RevealResult {
    Safe(u8),
    Mine,
}

#[derive(Debug, PartialEq, Eq)]
enum MoveError {
    OutOfBounds,
    Flagged,
    AlreadyRevealed,
}

struct Board {
    width: usize,
    height: usize,
    mines: Vec<bool>,
    cells: Vec<Cell>,
}

impl Board {
    fn new(width: usize, height: usize, mines: &[(usize, usize)]) -> Board {
        assert!(width > 0 && height > 0);
        let mut board = Board {
            width,
            height,
            mines: vec![false; width * height],
            cells: vec![Cell::Hidden; width * height],
        };
        for &(x, y) in mines {
            let index = board.index(x, y).expect("mine inside board");
            board.mines[index] = true;
        }
        board
    }

    fn index(&self, x: usize, y: usize) -> Result<usize, MoveError> {
        if x >= self.width || y >= self.height {
            Err(MoveError::OutOfBounds)
        } else {
            Ok(y * self.width + x)
        }
    }

    fn neighbouring_mines(&self, _x: usize, _y: usize) -> Result<u8, MoveError> {
        todo!()
    }

    fn toggle_flag(&mut self, _x: usize, _y: usize) -> Result<(), MoveError> {
        todo!()
    }

    fn reveal(&mut self, _x: usize, _y: usize) -> Result<RevealResult, MoveError> {
        todo!()
    }

    fn show(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let symbol = match self.cells[y * self.width + x] {
                    Cell::Hidden => '#',
                    Cell::Flagged => 'F',
                    Cell::Revealed(0) => '.',
                    Cell::Revealed(count) => char::from(b'0' + count),
                    Cell::Exploded => '*',
                };
                print!("{symbol}");
            }
            println!();
        }
    }
}

fn main() {
    let mut board = Board::new(4, 3, &[(0, 0), (2, 1), (3, 2)]);
    let counts = [0, 2, 1, 1, 1, 2, 1, 2, 0, 1, 2, 1];
    for y in 0..3 {
        for x in 0..4 {
            assert_eq!(board.neighbouring_mines(x, y), Ok(counts[y * 4 + x]));
        }
    }
    assert_eq!(board.cells, vec![Cell::Hidden; 12]);

    assert_eq!(board.toggle_flag(0, 0), Ok(()));
    assert_eq!(board.cells[0], Cell::Flagged);
    assert_eq!(board.neighbouring_mines(1, 0), Ok(2));
    let before = board.cells.clone();
    assert_eq!(board.reveal(0, 0), Err(MoveError::Flagged));
    assert_eq!(board.cells, before);

    assert_eq!(board.toggle_flag(1, 2), Ok(()));
    assert_eq!(board.cells[9], Cell::Flagged);
    assert_eq!(board.toggle_flag(1, 2), Ok(()));
    assert_eq!(board.cells[9], Cell::Hidden);
    assert_eq!(board.reveal(0, 2), Ok(RevealResult::Safe(0)));
    assert_eq!(board.reveal(1, 0), Ok(RevealResult::Safe(2)));
    assert_eq!(board.reveal(3, 1), Ok(RevealResult::Safe(2)));
    assert_eq!(board.reveal(2, 1), Ok(RevealResult::Mine));
    assert_eq!(
        board.cells,
        vec![
            Cell::Flagged,
            Cell::Revealed(2),
            Cell::Hidden,
            Cell::Hidden,
            Cell::Hidden,
            Cell::Hidden,
            Cell::Exploded,
            Cell::Revealed(2),
            Cell::Revealed(0),
            Cell::Hidden,
            Cell::Hidden,
            Cell::Hidden,
        ]
    );

    let before = board.cells.clone();
    for (x, y) in [(0, 2), (1, 0), (3, 1), (2, 1)] {
        assert_eq!(board.reveal(x, y), Err(MoveError::AlreadyRevealed));
        assert_eq!(board.toggle_flag(x, y), Err(MoveError::AlreadyRevealed));
        assert_eq!(board.cells, before);
    }
    for (x, y) in [(4, 0), (0, 3), (4, 3), (usize::MAX, 0), (0, usize::MAX)] {
        assert_eq!(board.neighbouring_mines(x, y), Err(MoveError::OutOfBounds));
        assert_eq!(board.toggle_flag(x, y), Err(MoveError::OutOfBounds));
        assert_eq!(board.reveal(x, y), Err(MoveError::OutOfBounds));
        assert_eq!(board.cells, before);
    }

    let mut single = Board::new(1, 1, &[]);
    assert_eq!(single.neighbouring_mines(0, 0), Ok(0));
    assert_eq!(single.reveal(0, 0), Ok(RevealResult::Safe(0)));
    assert_eq!(single.cells, vec![Cell::Revealed(0)]);
    assert_eq!(single.reveal(0, 0), Err(MoveError::AlreadyRevealed));

    let mut mine = Board::new(1, 1, &[(0, 0)]);
    assert_eq!(mine.neighbouring_mines(0, 0), Ok(0));
    assert_eq!(mine.toggle_flag(0, 0), Ok(()));
    assert_eq!(mine.reveal(0, 0), Err(MoveError::Flagged));
    assert_eq!(mine.toggle_flag(0, 0), Ok(()));
    assert_eq!(mine.reveal(0, 0), Ok(RevealResult::Mine));
    assert_eq!(mine.cells, vec![Cell::Exploded]);
    assert_eq!(mine.toggle_flag(0, 0), Err(MoveError::AlreadyRevealed));

    let line = Board::new(1, 3, &[(0, 2)]);
    assert_eq!(line.neighbouring_mines(0, 0), Ok(0));
    assert_eq!(line.neighbouring_mines(0, 1), Ok(1));
    assert_eq!(line.neighbouring_mines(0, 2), Ok(0));
    let row = Board::new(3, 1, &[(2, 0)]);
    assert_eq!(row.neighbouring_mines(0, 0), Ok(0));
    assert_eq!(row.neighbouring_mines(1, 0), Ok(1));
    assert_eq!(row.neighbouring_mines(2, 0), Ok(0));
    let surrounded = Board::new(
        3,
        3,
        &[
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ],
    );
    assert_eq!(surrounded.neighbouring_mines(1, 1), Ok(8));

    board.show();
    println!("minesweeper: ok");
}

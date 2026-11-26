use rand::seq::SliceRandom;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CellState {
    Hidden,
    Revealed,
    Detonated,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Cell {
    pub state: CellState,
    pub flagged: bool,
    pub mined: bool,
    pub mined_neightbors_count: u8,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            state: CellState::Hidden,
            flagged: false,
            mined: false,
            mined_neightbors_count: 0,
        }
    }
}

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    pub mine_count: usize,
}

impl Board {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Self {
        Self {
            width,
            height,
            cells: Vec::new(),
            mine_count,
        }
    }

    pub fn generate_cells(&mut self) -> () {
        self.cells.clear();

        let cells_total_count = self.width * self.height;
        let mut mine_placed = 0;

        for _ in 0..cells_total_count {
            let mut cell = Cell::new();

            if mine_placed < self.mine_count {
                cell.mined = true;
                mine_placed += 1;
            }

            self.cells.push(cell)
        }

        self.cells.shuffle(&mut rand::rng());

        for i in 0..self.cells.len() {
            let count = self.count_neightbors_mined(i);
            self.cells[i].mined_neightbors_count = count;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(y * self.width + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.cells.get_mut(y * self.width + x)
    }

    fn get_coor_from_index(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index.div_euclid(self.width);

        (x, y)
    }

    fn count_neightbors_mined(&self, index: usize) -> u8 {
        let (x, y) = self.get_coor_from_index(index);
        let mut mined_neightbors_count: u8 = 0;

        for dx in -1isize..=1 {
            for dy in -1isize..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0 || ny < 0 {
                    continue;
                }

                let (nx, ny) = (nx as usize, ny as usize);

                if let Some(cell) = self.get(nx, ny)
                    && cell.state == CellState::Hidden
                    && cell.mined
                {
                    mined_neightbors_count += 1;
                }
            }
        }

        mined_neightbors_count
    }

    fn get_hidden_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        for dx in -1isize..=1 {
            for dy in -1isize..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0 || ny < 0 {
                    continue;
                }

                let (nx, ny) = (nx as usize, ny as usize);

                if let Some(cell) = self.get(nx, ny)
                    && cell.state == CellState::Hidden
                {
                    neighbors.push((nx, ny));
                }
            }
        }

        neighbors
    }

    pub fn reveal(&mut self, x: usize, y: usize) -> () {
        let mined = match self.get_mut(x, y) {
            Some(cell) => {
                if cell.mined {
                    cell.state = CellState::Detonated;
                    return;
                }

                cell.state = CellState::Revealed;
                cell.mined
            }
            None => return,
        };

        if !mined {
            if let Some(cell) = self.get(x, y)
                && cell.mined_neightbors_count == 0
            {
                let neightbors = self.get_hidden_neighbors(x, y);

                for (nx, ny) in neightbors {
                    self.reveal(nx, ny);
                }
            }
        }
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) -> () {
        if let Some(cell) = self.get_mut(x, y) {
            if cell.state == CellState::Hidden {
                cell.flagged = !cell.flagged;
            }
        }
    }

    pub fn has_win(&self) -> bool {
        let count_hidden = self
            .cells
            .iter()
            .filter(|cell| cell.state == CellState::Hidden)
            .count();

        count_hidden == self.mine_count
    }

    pub fn has_lose(&self) -> bool {
        self.cells
            .iter()
            .find(|&cell| cell.mined && cell.state != CellState::Hidden)
            .is_some()
    }
}

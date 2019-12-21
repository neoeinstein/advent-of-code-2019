use super::{GridPosition, Orientation};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid<T> {
    elements: Vec<T>,
    columns: usize,
    rows: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new(default: T, rows: usize, columns: usize) -> Self {
        Self {
            elements: vec![default; rows * columns],
            rows,
            columns,
        }
    }
}

impl<T> Grid<T> {
    pub fn max(&self) -> GridPosition {
        GridPosition::new(self.rows - 1, self.columns - 1)
    }

    fn idx(&self, position: GridPosition) -> Option<usize> {
        let p = position.limit(self.max())?;
        Some(p.idx(self.columns))
    }

    pub fn get(&self, position: GridPosition) -> Option<&T> {
        let idx = self.idx(position)?;
        Some(&self.elements[idx])
    }

    pub fn get_neighbor(&self, position: GridPosition, direction: Orientation) -> Option<&T> {
        let neighbor = position.neighbor(direction)?;
        let idx = self.idx(neighbor)?;
        Some(&self.elements[idx])
    }

    pub fn set(&mut self, position: GridPosition, element: T) -> Option<T> {
        let idx = self.idx(position)?;
        Some(std::mem::replace(&mut self.elements[idx], element))
    }

    pub fn enumerate(&self) -> GridIterator<T> {
        GridIterator::new(self)
    }

    pub fn column(&self, column: usize) -> ColumnIterator<T> {
        ColumnIterator::new(self, column)
    }

    pub fn row(&self, row: usize) -> RowIterator<T> {
        RowIterator::new(self, row)
    }
}

impl<T> std::str::FromStr for Grid<T>
where
    T: std::str::FromStr,
{
    type Err = T::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elements = Vec::new();
        let mut rows = 0;
        let mut columns = 0;
        for l in s.lines() {
            let trimmed = l;
            if trimmed.is_empty() {
                continue;
            }
            let mut cols = 0;
            for ch in trimmed.chars() {
                let mut buf = [0_u8; 4];
                let e = ch.encode_utf8(&mut buf).parse()?;
                elements.push(e);
                cols += 1;
            }
            if columns == 0 {
                columns = cols;
            } else {
                debug_assert_eq!(cols, columns);
            }
            rows += 1;
        }

        debug_assert_eq!(elements.len(), rows * columns);

        Ok(Grid {
            elements,
            columns,
            rows,
        })
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in self.elements.chunks_exact(self.columns as usize) {
            for element in line {
                write!(f, "{}", element)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    pos: GridPosition,
    idx: usize,
}

impl<'a, T> GridIterator<'a, T> {
    const fn new(grid: &'a Grid<T>) -> Self {
        Self {
            grid,
            pos: GridPosition::ORIGIN,
            idx: 0,
        }
    }
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = (GridPosition, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.grid.elements.get(self.idx)?;
        let pos = self.pos;
        self.idx += 1;
        self.pos.col += 1;
        if self.pos.col >= self.grid.columns {
            self.pos.col = 0;
            self.pos.row += 1;
        }
        debug_assert_eq!(self.pos.idx(self.grid.columns), self.idx);
        Some((pos, item))
    }
}

pub struct ColumnIterator<'a, T> {
    grid: &'a Grid<T>,
    pos: GridPosition,
    idx: usize,
}

impl<'a, T> ColumnIterator<'a, T> {
    const fn new(grid: &'a Grid<T>, column: usize) -> Self {
        Self {
            grid,
            pos: GridPosition {
                row: 0,
                col: column,
            },
            idx: column,
        }
    }
}

impl<'a, T> Iterator for ColumnIterator<'a, T> {
    type Item = (GridPosition, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.grid.elements.get(self.idx)?;
        let pos = self.pos;
        self.idx += self.grid.columns;
        self.pos.row += 1;
        debug_assert_eq!(self.pos.idx(self.grid.columns), self.idx);
        Some((pos, item))
    }
}

pub struct RowIterator<'a, T> {
    grid: &'a Grid<T>,
    pos: GridPosition,
    idx: usize,
}

impl<'a, T> RowIterator<'a, T> {
    const fn new(grid: &'a Grid<T>, row: usize) -> Self {
        Self {
            grid,
            pos: GridPosition { row, col: 0 },
            idx: grid.columns * row,
        }
    }
}

impl<'a, T> Iterator for RowIterator<'a, T> {
    type Item = (GridPosition, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.grid.elements.get(self.idx)?;
        let pos = self.pos;
        self.idx += 1;
        self.pos.col += 1;
        debug_assert_eq!(self.pos.idx(self.grid.columns), self.idx);
        Some((pos, item))
    }
}

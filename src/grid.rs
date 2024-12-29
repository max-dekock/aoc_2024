use crate::coord::Coord;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    data: Vec<T>,
    pub width: usize,
    pub height: usize
}

impl<T> Grid<T> {
    fn coord_to_index(&self, coord: Coord) -> Option<usize> {
        coord.bound_checked(Coord(0, 0), Coord(self.width as i64, self.height as i64))
            .map(|Coord(x, y)| y as usize * self.width + x as usize)
    }

    fn index_to_coord(&self, index: usize) -> Coord {
        Coord((index % self.width) as i64, (index / self.width) as i64)
    }

    pub fn get(&self, coord: Coord) -> Option<&T> {
        self.coord_to_index(coord)
            .map(|index| &self.data[index])
    }

    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        self.coord_to_index(coord)
            .map(|index| &mut self.data[index])
    }

    pub fn get_row(&self, row: usize) -> Option<&[T]> {
        if row > self.height {
            None
        } else {
            let index = row * self.width;
            Some(&self.data[index..index+self.width])
        }
    }

    pub fn get_row_mut(&mut self, row: usize) -> Option<&mut [T]> {
        if row > self.height {
            None
        } else {
            let index = row * self.width;
            Some(&mut self.data[index..index+self.width])
        }
    }

    pub fn try_from_vec(data: Vec<T>, width: usize) -> Result<Grid<T>, Vec<T>> {
        if data.len() % width != 0 {
            return Err(data);
        }
        let height = data.len() / width;
        Ok(Grid { data, width, height })
    }

    pub fn try_from_iterable(iter: impl IntoIterator<Item = T>, width: usize) -> Result<Grid<T>, Vec<T>> {
        let data: Vec<T> = iter.into_iter().collect();
        Grid::try_from_vec(data, width)
    }

    pub fn try_from_rows<R>(iter: impl IntoIterator<Item = R>) -> Option<Grid<T>>
    where R: IntoIterator<Item = T>
    {
        let mut data: Vec<T> = vec![];
        let mut width_opt: Option<usize> = None;
        let mut height = 0;
        for row in iter {
            height += 1;
            if let Some(width) = width_opt {
                let prev_len = data.len();
                data.extend(row);
                if data.len() - prev_len != width {
                    return None;
                }
            } else {
                data.extend(row);
                width_opt = Some(data.len());
            }
        }
        let width = width_opt.unwrap_or(0);
        Some(Grid { data, width, height })
    }

    pub fn row_iter(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks_exact(self.width)
    }
    
    pub fn cell_iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_with_coords(&self) -> impl Iterator<Item = (Coord, &T)> {
        self.data.iter()
            .enumerate()
            .map(|(idx, cell)| (self.index_to_coord(idx), cell))
    }
}

impl<T> Grid<T>
where T: Default
{
    pub fn new_with_default(width: usize, height: usize) -> Grid<T> {
        let mut data = vec![];
        for _ in 0..width*height {
            data.push(Default::default());
        }
        Grid { data, width, height }
    }
}

impl<T> fmt::Display for Grid<T>
where T: fmt::Display
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.row_iter() {
            for cell in row {
                let cell_string = cell.to_string();
                formatter.pad(&cell_string)?;
            }
            write!(formatter, "\n")?;
        }
        Ok(())
    }
}

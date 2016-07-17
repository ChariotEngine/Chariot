// OpenAOE: An open source reimplementation of Age of Empires (1997)
// Copyright (c) 2016 Kevin Fuller
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use nalgebra::Vector2;

use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct CellKey {
    row: i32,
    col: i32,
}

impl CellKey {
    pub fn new(row: i32, col: i32) -> CellKey {
        CellKey {
            row: row,
            col: col,
        }
    }
}

struct Cell {
    entities: Vec<u32>,
}

impl Cell {
    fn new() -> Cell {
        Cell { entities: Vec::new() }
    }

    fn add(&mut self, entity_id: u32) {
        self.entities.push(entity_id);
    }

    fn remove(&mut self, entity_id: u32) {
        if let Some(index) = self.entities.iter().position(|id| *id == entity_id) {
            self.entities.swap_remove(index);
        }
    }

    #[inline]
    fn entities<'a>(&'a self) -> &Vec<u32> {
        &self.entities
    }
}

pub struct GridPartition {
    cell_width: i32,
    cell_height: i32,
    entities: HashMap<u32, CellKey>,
    cells: HashMap<CellKey, Cell>,
}

/// Infinite grid spatial partition
impl GridPartition {
    pub fn new(cell_width: i32, cell_height: i32) -> GridPartition {
        GridPartition {
            cell_width: cell_width,
            cell_height: cell_height,
            entities: HashMap::new(),
            cells: HashMap::new(),
        }
    }

    /// Tells the grid where an entity is so that it can be queried later
    pub fn update_entity(&mut self, entity_id: u32, position: &Vector2<i32>) {
        let cell_key = self.cell_key(&position);
        if !self.entities.contains_key(&entity_id) {
            self.entities.insert(entity_id, cell_key);
        } else {
            let old_cell_key = *self.entities.get(&entity_id).unwrap();
            self.remove_from_cell(old_cell_key, entity_id);
        }
        self.add_to_cell(cell_key, entity_id);
    }

    /// Returns the entity IDs that lie in the cells overlapped by the given bounds
    /// Note: the returned entity IDs can lie outside of the bounds
    pub fn query(&self,
                 start_position: &Vector2<i32>,
                 end_position: &Vector2<i32>)
                 -> HashSet<u32> {
        let start = self.row_col(start_position);
        let end = self.row_col(end_position);

        let mut entities = HashSet::new();
        for row in start.y..(end.y + 1) {
            for col in start.x..(end.x + 1) {
                if let Some(cell) = self.cell(CellKey::new(row, col)) {
                    entities.extend(cell.entities().iter());
                }
            }
        }
        entities
    }

    pub fn contains(&self, entity_id: u32) -> bool {
        self.entities.contains_key(&entity_id)
    }

    fn add_to_cell(&mut self, cell_key: CellKey, entity_id: u32) {
        self.cell_mut(cell_key).add(entity_id);
    }

    fn remove_from_cell(&mut self, cell_key: CellKey, entity_id: u32) {
        self.cell_mut(cell_key).remove(entity_id);
    }

    #[inline]
    fn cell<'a>(&'a self, cell_key: CellKey) -> Option<&'a Cell> {
        self.cells.get(&cell_key)
    }

    fn cell_mut<'a>(&'a mut self, cell_key: CellKey) -> &'a mut Cell {
        if !self.cells.contains_key(&cell_key) {
            self.cells.insert(cell_key, Cell::new());
        }
        self.cells.get_mut(&cell_key).unwrap()
    }

    fn cell_key(&self, position: &Vector2<i32>) -> CellKey {
        let row_col = self.row_col(position);
        CellKey::new(row_col.y, row_col.x)
    }

    fn row_col(&self, position: &Vector2<i32>) -> Vector2<i32> {
        Vector2::new(position.x / self.cell_width, position.y / self.cell_height)
    }
}

#[cfg(test)]
mod tests {
    use super::{Cell, CellKey, GridPartition};
    use nalgebra::Vector2;
    use std::collections::HashSet;

    macro_rules! ids {
        [ $($id:expr),* ] => {
            {
                let mut set: HashSet<u32> = HashSet::new();
                $(set.insert($id);)*
                set
            }
        }
    }

    fn v(x: i32, y: i32) -> Vector2<i32> {
        Vector2::new(x, y)
    }

    #[test]
    fn test_cell_add_remove() {
        let mut cell = Cell::new();
        cell.remove(5); // shouldn't panic

        cell.add(4);
        assert_eq!(&vec![4], cell.entities());

        cell.add(5);
        cell.add(6);
        assert_eq!(&vec![4, 5, 6], cell.entities());

        cell.remove(4);
        assert_eq!(&vec![6, 5], cell.entities());
    }

    #[test]
    fn test_cell_key_from_position() {
        let grid = GridPartition::new(10, 10);
        assert_eq!(CellKey::new(0, 0), grid.cell_key(&v(0, 0)));
        assert_eq!(CellKey::new(0, 0), grid.cell_key(&v(5, 5)));
        assert_eq!(CellKey::new(0, 1), grid.cell_key(&v(10, 5)));
        assert_eq!(CellKey::new(1, 0), grid.cell_key(&v(5, 10)));
    }

    #[test]
    fn test_grid_update_entity() {
        let mut grid = GridPartition::new(10, 10);
        grid.update_entity(1, &v(5, 5));
        grid.update_entity(2, &v(15, 5));

        assert_eq!(&vec![1], grid.cell_mut(CellKey::new(0, 0)).entities());
        assert_eq!(&vec![2], grid.cell_mut(CellKey::new(0, 1)).entities());

        grid.update_entity(1, &v(25, 15));
        assert!(grid.cell_mut(CellKey::new(0, 0)).entities().is_empty());
        assert_eq!(&vec![2], grid.cell_mut(CellKey::new(0, 1)).entities());
        assert_eq!(&vec![1], grid.cell_mut(CellKey::new(1, 2)).entities());
    }

    #[test]
    fn test_grid_query() {
        let mut grid = GridPartition::new(10, 10);
        grid.update_entity(1, &v(5, 5));
        grid.update_entity(2, &v(6, 5));
        grid.update_entity(3, &v(15, 5));
        grid.update_entity(4, &v(5, 15));

        assert_eq!(ids![1, 2], grid.query(&v(1, 1), &v(9, 9)));
        assert_eq!(ids![1, 2, 3, 4], grid.query(&v(0, 0), &v(20, 20)));
        assert_eq!(ids![1, 2, 3, 4], grid.query(&v(9, 0), &v(20, 10)));
        assert_eq!(ids![3], grid.query(&v(10, 0), &v(20, 10)));
    }
}

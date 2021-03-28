use std::ops::{Index, IndexMut};
use thiserror::Error;

type Coordinates = (usize, usize);

pub struct Array2D<T, const W: usize, const H: usize>
where
    [T; W * H]: ,
{
    elements: [T; W * H],
}

impl<T, const W: usize, const H: usize> Array2D<T, W, H>
where
    [T; W * H]: ,
{
    const SIZE: usize = W * H;
    pub fn new(elements: [T; W * H]) -> Self {
        let a = [1, 2, 3, 4_u32];
        Self { elements }
    }
    pub fn get(&self, (column, row): Coordinates) -> Option<&T> {
        if (row + 1) * (column + 1) > Self::SIZE {
            None
        } else {
            Some(&self[(column, row)])
        }
    }

    pub fn get_mut(&mut self, (column, row): Coordinates) -> Option<&mut T> {
        if (row + 1) * (column + 1) > Self::SIZE {
            None
        } else {
            Some(&mut self[(column, row)])
        }
    }
}

impl<T, const W: usize, const H: usize> Index<Coordinates> for Array2D<T, W, H>
where
    [T; W * H]: ,
{
    type Output = T;
    fn index(&self, (column, row): Coordinates) -> &Self::Output {
        let index = column + row * W;
        println!("index - - {}", index);
        &self.elements[index]
    }
}

impl<T, const W: usize, const H: usize> IndexMut<Coordinates> for Array2D<T, W, H>
where
    [T; W * H]: ,
{
    fn index_mut(&mut self, (column, row): Coordinates) -> &mut Self::Output {
        let index = row * column + column;
        &mut self.elements[index]
    }
}

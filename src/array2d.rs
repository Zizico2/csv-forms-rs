use std::ops::{Index, IndexMut};
use thiserror::Error;

type Coordinates = (usize, usize);

pub struct Array2D<T, const H: usize, const W: usize>
where
    [T; H * W]: ,
{
    elements: [T; H * W],
}

impl<T, const H: usize, const W: usize> Array2D<T, H, W>
where
    [T; H * W]: ,
{
    pub const SIZE: usize = H * W;
    pub fn new(elements: [T; H * W]) -> Self {
        Self { elements }
    }
    pub fn get(&self, (row, column): Coordinates) -> Option<&T> {
        if (row + 1) * (column + 1) > Self::SIZE {
            None
        } else {
            Some(&self[(row, column)])
        }
    }

    pub fn get_mut(&mut self, (row, column): Coordinates) -> Option<&mut T> {
        if (row + 1) * (column + 1) > Self::SIZE {
            None
        } else {
            Some(&mut self[(row, column)])
        }
    }
    pub fn into_row_majored(self) -> [T; H * W] {
        self.elements
    }
    pub fn get_row_majored(&self) -> &[T; H * W] {
        &self.elements
    }
}

impl<T, const H: usize, const W: usize> Index<Coordinates> for Array2D<T, H, W>
where
    [T; H * W]: ,
{
    type Output = T;
    fn index(&self, (row, column): Coordinates) -> &Self::Output {
        let index = row + column * W;
        &self.elements[index]
    }
}

impl<T, const H: usize, const W: usize> IndexMut<Coordinates> for Array2D<T, H, W>
where
    [T; H * W]: ,
{
    fn index_mut(&mut self, (row, column): Coordinates) -> &mut Self::Output {
        let index = row + column * W;
        &mut self.elements[index]
    }
}

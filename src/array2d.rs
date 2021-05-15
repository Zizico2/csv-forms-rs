use std::ops::DerefMut;
use std::ops::{Index, IndexMut};
use std::{fmt::Display, ops::Deref};
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
    pub fn get(&self, (row, column): Coordinates) -> Option<&T> {
        if (row + 1) * (column + 1) > Self::SIZE {
            None
        } else {
            Some(&self[(row, column)])
        }
    }

    pub fn from(elements: [T; H * W]) -> Self {
        Self { elements }
    }

    pub fn get_mut(&mut self, (row, column): Coordinates) -> Option<&mut T> {
        if (row + 1) * (column + 1) > Self::SIZE {
            None
        } else {
            Some(&mut self[(row, column)])
        }
    }
    // TODO when impl Trait with const generics is usable
    // TODO this should be swaped for an implementation of "From<Array2d> for [T; H * W]" (see end of file)
    // TODO should this be named "into" ? for future compatibility when we can implement those traits
    // TODO we could name it "into_array" for differenciation but it would eventually be deprecated and removed
    pub fn into_array(self) -> [T; H * W] {
        self.elements
    }
    pub fn into_array_of_arrays(self) -> [[T; H]; W]
    where
        T: Clone,
        [[T; H]; W]: Default,
    {
        let mut state: [[T; H]; W] = Default::default();
        for x in 0..W {
            for y in 0..H {
                state[x][y] = (&self)[(x, y)].clone();
            }
        }
        state
    }
}
/*
impl<T, const H: usize, const W: usize> From<[T; H * W]> for Array2D<T, H, W> {
    fn from(elements: [T; H * W]) -> Self {
        Self { elements }
    }
}
*/
impl<T, const H: usize, const W: usize> Index<Coordinates> for Array2D<T, H, W>
where
    [T; H * W]: ,
{
    type Output = T;
    fn index(&self, (row, column): Coordinates) -> &Self::Output {
        let index = column + row * H;
        &self.elements[index]
    }
}

impl<T, const H: usize, const W: usize> IndexMut<Coordinates> for Array2D<T, H, W>
where
    [T; H * W]: ,
{
    fn index_mut(&mut self, (row, column): Coordinates) -> &mut Self::Output {
        let index = column + row * H;
        &mut self.elements[index]
    }
}

impl<T, const H: usize, const W: usize> Deref for Array2D<T, H, W>
where
    [T; H * W]: ,
{
    type Target = [T; H * W];

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl<T, const H: usize, const W: usize> DerefMut for Array2D<T, H, W>
where
    [T; H * W]: ,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
    }
}

impl<T, const H: usize, const W: usize> Display for Array2D<T, H, W>
where
    T: Display,
    [T; H * W]: ,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..W {
            for y in 0..H {
                write!(f, "| {} ", self[(x, y)])?;
            }
            write!(f, "|")?;

            if x < W - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

// TODO this should work, eventually - incomplete features n dat
/*
impl<T, const H: usize, const W: usize> From<Array2D<T, H, W>> for [T; H * W]
where
    [T; H * W]: ,
{
    fn from(array2d: Array2D<T, H, W>) -> Self {
        array2d.elements
    }
}
*/

// TODO if this is usable before the above option, it should be used temporarily
/*
impl<T, const H: usize, const W: usize> Into<[T; H * W]> for Array2D<T, H, W> {
    fn into(self) -> [T; H * W] {
        self.elements
    }
}
*/

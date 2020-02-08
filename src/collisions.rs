use crate::shape::{Vector};
use std::vec::IntoIter;
use std::slice::Iter;
use std::mem;
use amethyst::core::ecs::{prelude::*, Component};


pub struct Collision<Tag> {
    pub resolve: Vector,
    pub tag: Tag
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Collisions<Tag: 'static + Send + Sync> {
    contents: Vec<Collision<Tag>>,
}
impl<T: 'static + Send + Sync> Collisions<T> {
    pub fn new() -> Self {
        Collisions{
            contents: Vec::new()
        }
    }
    pub fn drain(&mut self) -> IntoIter<Collision<T>> {
        mem::replace(&mut self.contents, Vec::new()).into_iter()
    }
    pub fn add(&mut self, item: Collision<T>) {
        self.contents.push(item)
    }
}
impl<T: 'static + Send + Sync> IntoIterator for Collisions<T> {
    type IntoIter = IntoIter<Collision<T>>;
    type Item = Collision<T>;

    fn into_iter(self) -> IntoIter<Collision<T>> {
        self.contents.into_iter()
    }
}
impl<'a, T: 'static + Send + Sync> IntoIterator for &'a Collisions<T> {
    type IntoIter = Iter<'a, Collision<T>>;
    type Item = &'a Collision<T>;

    fn into_iter(self) -> Iter<'a, Collision<T>> {
        self.contents.iter()
    }
}
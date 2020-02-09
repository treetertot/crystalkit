use amethyst::core::{ecs::{prelude::*, Join}, transform::Transform};
use std::marker::PhantomData;
use crate::{shape::*, collisions::*};

pub struct CollisionSystem<Tag>(PhantomData<Tag>);
impl<T> CollisionSystem<T> {
    pub fn new() -> Self {
        CollisionSystem(PhantomData::new())
    }
}
impl<'a, T: Component + Clone + Send + Sync> System<'a> for CollisionSystem<T> {
    type SystemData = (ReadStorage<'a, Shape>, ReadStorage<'a, Transform>, ReadStorage<'a, T>, WriteStorage<'a, Collisions<T>>);

    fn run(&mut self, (shapes, transforms, tags, mut collisions): Self::SystemData) {
        for (i, (shapea, transa, taga, outputa)) in (&shapes, &transforms, &tags, &mut collisions).join().enumerate() {
            for (shapeb, transb, tagb) in (&shapes, &transforms, &tags).join().skip(i+1) {
                if shapea as *const Shape == shapeb as *const Shape {
                    continue;
                }
                if let Some(res) = shapea.resolve(transa, shapeb, transb) {
                    outputa.add(Collision{
                        resolve: res,
                        tag: tagb.clone()
                    });
                }
            }
        }
    }
}
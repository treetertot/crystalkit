use amethyst::core::{ecs::{prelude::*, Join}, transform::Transform};
use std::marker::PhantomData;
use crate::{shape::*, collisions::*};

pub struct CollisionSystem<Tag>(PhantomData<Tag>);
impl<'a, T: Component + Clone + Send + Sync> System<'a> for CollisionSystem<T> {
    type SystemData = (ReadStorage<'a, Shape>, ReadStorage<'a, Transform>, ReadStorage<'a, T>, WriteStorage<'a, Collisions<T>>);

    fn run(&mut self, (shapes, transforms, tags, mut collisions): Self::SystemData) {
        let mut colcache: Vec<(&Shape, Collision<T>)> = Vec::new();
        for (i, (shapea, transa, taga, outputa)) in (&shapes, &transforms, &tags, &mut collisions).join().enumerate() {
            {
                let mut i = 0;
                while i < colcache.len() {
                    let shape = colcache[i].0;
                    if shapea as *const Shape == shape as *const Shape {
                        outputa.add(colcache.remove(i).1);
                    }
                    i += 1;
                }
            }
            for (shapeb, transb, tagb) in (&shapes, &transforms, &tags).join().skip(i+1) {
                if shapea as *const Shape == shapeb as *const Shape {
                    continue;
                }
                if let Some(res) = shapea.resolve(transa, shapeb, transb) {
                    outputa.add(Collision{
                        resolve: res,
                        tag: tagb.clone()
                    });
                    colcache.push((shapeb, Collision{
                        resolve: res * -1.0,
                        tag: taga.clone(),
                    }));
                }
            }
        }
    }
}
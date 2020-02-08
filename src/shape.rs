use amethyst::core::{transform::Transform, math::{Point2, Vector2, Vector3}};
use amethyst::core::ecs::{prelude::*, Component};

pub type Point = Point2<f32>;
pub type Vector = Vector2<f32>;

mod shape_iters;
use shape_iters::*;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Shape {
    points: Vec<Point>,
    center: Point,
}

impl Shape {
    pub fn new<I: IntoIterator<Item=Point>>(points: I) -> Shape {
        let mut iter = points.into_iter().peekable();
        let mut points = Vec::with_capacity(iter.size_hint().0);
        let first = iter.peek();
        if let Some(&first) = first {
            while let Some(point) = iter.next() {
                match iter.peek() {
                    Some(&next) => {
                        points.push(point);
                        points.push(Point::from((Vector::new(point.x, point.y) + Vector::new(next.x, next.y)) / 2.0));
                    }
                    None => {
                        points.push(point);
                        points.push(Point::from((Vector::new(point.x, point.y) + Vector::new(first.x, first.y)) / 2.0));
                    }
                }
            }
        }
        let mut avg = Vector2::new(0.0, 0.0);
        for &point in &points {
            let v = Vector3::from(point); 
            avg += Vector2::new(v.x, v.y);
        }
        if points.len() != 0 {
            avg = avg / (points.len() as f32);
        }
        Shape {
            center: Point::from(avg),
            points,
        }
    }
    pub fn iter_points<'a>(&'a self, transform: &'a Transform) -> PointsIter<'a> {
        PointsIter::new(self.points.iter(), transform)
    }
    fn iter_sides<'a>(&'a self, transform: &'a Transform) -> SidesIter<'a> {
        SidesIter::new(self.iter_points(transform), self.center)
    }
    fn dist_inside(&self, transform: &Transform, point: Point) -> Option<Vector> {
        let mut out: Option<(Vector, f32)> = None;
        for side in self.iter_sides(transform) {
            let dist = side.distance(point)?;
            let mag = dist.magnitude();
            match out {
                Some(val) => {
                    if mag < val.1 {
                        out = Some((dist, mag))
                    }
                }
                None => out = Some((dist, mag)),
            }
        }
        Some(out?.0)
    }
    pub fn resolve(&self, strans: &Transform, other: &Shape, otrans: &Transform) -> Option<Vector> {
        Some(
            self.iter_points(strans)
                .filter_map(|point| other.dist_inside(otrans, point))
                .chain(
                    other
                        .iter_points(otrans)
                        .filter_map(|point| Some(self.dist_inside(strans, point)? * -1.0)),
                )
                .fold(None, |prev, new_pt| match prev {
                    Some((mag, vec)) => {
                        let new_mag = new_pt.magnitude();
                        if mag < new_mag {
                            Some((new_mag, new_pt))
                        } else {
                            Some((mag, vec))
                        }
                    }
                    None => Some((new_pt.magnitude(), new_pt)),
                })?
                .1,
        )
    }
}
use crate::shape::Point;
use crate::lines::{InEq, Line};
use amethyst::core::{transform::Transform, math::Point3};
use std::iter::Peekable;
use std::slice::Iter;

fn transform_point(trans: &Transform, point: &Point) -> Point {
    let three = trans.isometry().transform_point(&Point3::new(point.x, point.y, 0.0));
    Point::new(three.x, three.y)
}

pub struct PointsIter<'a> {
    points: Iter<'a, Point>,
    transform: &'a Transform,
}
impl<'a> Iterator for PointsIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        let &a = self.points.next()?;
        Some(transform_point(&self.transform, &a))
    }
}
impl<'a> PointsIter<'a> {
    pub fn new(points: Iter<'a, Point>, transform: &'a Transform) -> Self {
        PointsIter{
            points,
            transform,
        }
    }
}

pub struct SidesIter<'a> {
    points: Peekable<PointsIter<'a>>,
    center: Point,
    first: Point,
}
impl<'a> Iterator for SidesIter<'a> {
    type Item = InEq;

    fn next(&mut self) -> Option<InEq> {
        let a = self.points.next()?;
        match self.points.peek() {
            Some(&b) => Some(Line::through(a, b).initialize(self.center)),
            None => Some(Line::through(a, self.first).initialize(self.center)),
        }
    }
}
impl<'a> SidesIter<'a> {
    pub fn new(points: PointsIter<'a>, center: Point) -> Self {
        let mut points = points.peekable();
        let first = match points.peek() {
            Some(&first) => first,
            None => Point::new(0.0, 0.0),
        };
        SidesIter {
            points,
            center,
            first,
        }
    }
}
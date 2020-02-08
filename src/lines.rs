use crate::shape::{Point, Vector};

#[derive(Clone, Copy)]
pub struct Line {
    slope: Option<f32>,
    constant: f32,
}
impl Line {
    pub fn through(a: Point, b: Point) -> Line {
        if a.x == b.x {
            Line {
                slope: None,
                constant: a.x,
            }
        } else {
            let m = (a.y - b.y) / (a.x - b.x);
            Line {
                slope: Some(m),
                constant: a.y - (m * a.x),
            }
        }
    }
    pub fn y(&self, x: f32) -> Option<f32> {
        Some((self.slope? * x) + self.constant)
    }
    pub fn initialize(self, point: Point) -> InEq {
        InEq {
            greater: match self.y(point.x) {
                Some(val) => point.y > val,
                None => point.x > self.constant,
            },
            line: self,
        }
    }
    pub fn normal_through(&self, point: Point) -> Line {
        match self.slope {
            Some(val) => {
                if val == 0.0 {
                    Line {
                        slope: None,
                        constant: point.x,
                    }
                } else {
                    let m = 1.0 / val;
                    Line {
                        slope: Some(m),
                        constant: point.y - (m * point.x),
                    }
                }
            }
            None => Line {
                slope: Some(0.0),
                constant: point.y,
            },
        }
    }
    pub fn intersection(self, b: &Line) -> Option<Point> {
        match self.slope {
            Some(m) => match b.slope {
                Some(m2) => {
                    let x = (b.constant - self.constant) / (m - m2);
                    Some(Point::new(
                        x,
                        self.y(x)?,
                    ))
                }
                None => Some(Point::new(
                    b.constant,
                    self.y(b.constant)?,
                )),
            },
            None => match b.slope {
                Some(_m) => Some(Point::new(
                    self.constant,
                    b.y(self.constant)?,
                )),
                None => None,
            },
        }
    }
    //pub fn intersection_segment(self, other: &Line, start: Point, end: Point) -> Option<Point> {
    //    let isect = self.intersection(other)?;
    //    if ((start.x <= isect.x && isect.x <= end.x) || (start.x >= isect.x && isect.x >= end.x))
    //        && ((start.y <= isect.y && isect.y <= end.y)
    //            || (start.y >= isect.y && isect.y >= end.y))
    //    {
    //        return Some(isect);
    //    }
    //    None
    //}
}

#[derive(Clone, Copy)]
pub struct InEq {
    line: Line,
    greater: bool,
}
impl InEq {
    pub fn contains(&self, point: Point) -> bool {
        match self.line.y(point.x) {
            Some(val) => {
                if self.greater {
                    point.y > val
                } else {
                    point.y < val
                }
            }
            None => {
                if self.greater {
                    point.x > self.line.constant
                } else {
                    point.x < self.line.constant
                }
            }
        }
    }
    pub fn distance(&self, point: Point) -> Option<Vector> {
        if self.contains(point) {
            return Some(
                self.line
                    .normal_through(point)
                    .intersection(&self.line)
                    .unwrap()
                    - point
            );
        }
        None
    }
}

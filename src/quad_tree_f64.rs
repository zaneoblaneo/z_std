use std::ops::{ Add, Sub };
use std::cmp::Ordering;

// This implementation was heavily based off of the pseudocode at
// https://en.wikipedia.org/wiki/Quadtree

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl From<(f64, f64)> for Point {
    fn from(i: (f64, f64)) -> Self {
        Self::new(i.0, i.1)
    }
}

impl From<(i64, i64)> for Point {
    fn from(i: (i64, i64)) -> Self {
        Self::new(i.0 as f64, i.1 as f64)
    }
}

impl Point {
    fn new(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    fn sqr_dist(&self, other: Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx*dx + dy*dy
    }

    fn dist(&self, other: Self) -> f64 {
        f64::sqrt(self.sqr_dist(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x > other.x && self.y > other.y {
            Some(Ordering::Greater)
        } else if self.x < other.x && self.y < other.y {
            Some(Ordering::Less)
        } else if self.x == other.x && self.y == other.y {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct CornerRect {
    tl: Point,
    br: Point,
}

#[derive(Debug, Clone, Copy)]
struct CenterSquare {
    center: Point,
    radius: f64,
}

impl CenterSquare {
    fn contains_point(&self, point: Point) -> bool {
        let bl = Point { 
            x: self.center.x - self.radius,
            y: self.center.y - self.radius,
        };
        let tr = Point {
            x: self.center.x + self.radius,
            y: self.center.y + self.radius,
        };
        bl < point && point < tr
    }

    fn corners(&self) -> CornerRect {
        CornerRect {
            tl: Point::new(
                self.center.x - self.radius, 
                self.center.y - self.radius
            ),
            br: Point::new(
                self.center.x + self.radius, 
                self.center.y + self.radius
            ),
        }
    }

    // This algo was stolen from https://www.javaspring.net/blog/how-do-i-see-if-two-rectangles-intersect-in-javascript-or-pseudocode/#the-proper-algorithm-checking-for-non-separation
    fn intersects_center_square(&self, other: Self) -> bool {
        let s1 = self.corners();
        let s2 = other.corners();
        !((s1.br.x <= s2.tl.x) || (s1.tl.x >= s2.br.x) || (s1.br.y <= s2.tl.y) || (s1.tl.y >= s2.br.y))
    }
}

struct QuadTreeChildren {
    nw: Option<Box<QuadTree>>,
    ne: Option<Box<QuadTree>>,
    sw: Option<Box<QuadTree>>,
    se: Option<Box<QuadTree>>,
}

impl QuadTreeChildren {
    fn new() -> Self {
        Self {
            nw: None,
            ne: None,
            sw: None,
            se: None,
        }
    }
}

const QUAD_TREE_NODE_CAPACITY: usize = 4;

struct QuadTree {
    boundary: CenterSquare,
    points: Vec<Point>,
    children: QuadTreeChildren,
}

impl QuadTree {
    fn new(boundary: CenterSquare) -> Self {
        Self {
            boundary: boundary,
            points: Vec::new(),
            children: QuadTreeChildren::new(),
        }
    }

    fn subdivide(&mut self) {
        assert!(self.children.nw.is_none());
        let nw_boundary = CenterSquare {
            center: Point {
                x: self.boundary.center.x - (self.boundary.radius / 2f64),
                y: self.boundary.center.y + (self.boundary.radius / 2f64),
            },
            radius: self.boundary.radius / 2f64,
        };
        let ne_boundary = CenterSquare {
            center: Point {
                x: self.boundary.center.x + (self.boundary.radius / 2f64),
                y: self.boundary.center.y + (self.boundary.radius / 2f64),
            },
            radius: self.boundary.radius / 2f64,
        };
        let sw_boundary = CenterSquare {
            center: Point {
                x: self.boundary.center.x - (self.boundary.radius / 2f64),
                y: self.boundary.center.y - (self.boundary.radius / 2f64),
            },
            radius: self.boundary.radius / 2f64,
        };
        let se_boundary = CenterSquare {
            center: Point {
                x: self.boundary.center.x + (self.boundary.radius / 2f64),
                y: self.boundary.center.y - (self.boundary.radius / 2f64),
            },
            radius: self.boundary.radius / 2f64,
        };
        self.children = QuadTreeChildren {
            nw: Some(Box::new(QuadTree::new(nw_boundary))),
            ne: Some(Box::new(QuadTree::new(ne_boundary))),
            sw: Some(Box::new(QuadTree::new(sw_boundary))),
            se: Some(Box::new(QuadTree::new(se_boundary))),
        }
    }

    fn insert(&mut self, p: Point) -> bool {
        if !self.boundary.contains_point(p) {
            return false;
        }

        if self.points.len() < QUAD_TREE_NODE_CAPACITY && self.children.nw.is_none() {
            self.points.push(p);
            return true;
        }

        if self.children.nw.is_none() {
            self.subdivide();
        }
        
        // assume that if self.subdivide has been called, all children are 
        // populated.

        if self.children.nw.as_mut().unwrap().insert(p) { 
            return true; 
        }
        if self.children.ne.as_mut().unwrap().insert(p) {
            return true;
        }
        if self.children.sw.as_mut().unwrap().insert(p) {
            return true;
        }
        if self.children.se.as_mut().unwrap().insert(p) {
            return true;
        }
        return false;
    }

    fn query_range(&self, range: CenterSquare) -> Vec<Point> {
        let mut out: Vec<Point> = Vec::new();
        if !self.boundary.intersects_center_square(range) {
            return out;
        }

        for p in 0..self.points.len() {
            if range.contains_point(self.points[p]) {
                out.push(self.points[p]);
            }
        }

        if self.children.nw.is_none() {
            return out;
        }

        let nw = self.children.nw.as_ref().unwrap().query_range(range);
        for p in nw {
            out.push(p);
        }
        let ne = self.children.ne.as_ref().unwrap().query_range(range);
        for p in ne {
            out.push(p);
        }
        let sw = self.children.sw.as_ref().unwrap().query_range(range);
        for p in sw {
            out.push(p);
        }
        let se = self.children.se.as_ref().unwrap().query_range(range);
        for p in se {
            out.push(p);
        }
        out
    }
}



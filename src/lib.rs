pub mod string_view;

pub mod quad_tree;
pub mod quad_tree_f64;
pub mod quad_tree_f32;

#[cfg(test)]
mod tests {
    use super::quad_tree::{ CenterSquare, Point, QuadTree };

    #[derive(Debug, Clone, Copy)]
    struct Vec2 {
        pub _x: f32,
        pub _y: f32,
    }

    impl From<(f32, f32)> for Vec2 {
        fn from(tup: (f32, f32)) -> Self {
            Self {
                _x: tup.0,
                _y: tup.1,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct Ball {
        _pos: Point,
        _vel: Vec2,
        _acc: Vec2,
    }

    impl Into<Point> for Ball {
        fn into(self) -> Point {
            self._pos
        }
    }

    impl From<(i32, i32)> for Ball {
        fn from(tup: (i32, i32)) -> Self {
            Self {
                _pos: tup.into(),
                _vel: (1f32, 1f32).into(),
                _acc: (0f32, 0f32).into(),
            }
        }
    }

    #[test]
    fn quad_tree_test() {
        let mut qt: QuadTree<Ball> = QuadTree::new(
            CenterSquare { 
                center: Point::new(0f64, 0f64), 
                radius: 14f64 
            }
        );
        qt.insert((9, 14).into());
        qt.insert((-14, 9).into());
        qt.insert((-11, 7).into());
        qt.insert((8, 6).into());
        qt.insert((0, 4).into());
        qt.insert((-7, 2).into());
        qt.insert((4, -1).into());
        qt.insert((-1, -7).into());
        qt.insert((-6, -7).into());
        qt.insert((-12, -8).into());
        qt.insert((3, -8).into());
        qt.insert((-7, -10).into());
        qt.insert((6, -11).into());
        qt.insert((3, -12).into());
        qt.insert((-13, -14).into());
        qt.insert((5, -14).into());
        qt.insert((1, 1).into());
        qt.insert((8, 3).into());

        let out = qt.query_range(CenterSquare {
            center: (7f64, 7f64).into(),
            radius: 7f64,
        });
        assert_eq!(format!("{out:?}"), "[Point { x: 8.0, y: 6.0 }, Point { x: 1.0, y: 1.0 }, Point { x: 8.0, y: 3.0 }]");
    }
}

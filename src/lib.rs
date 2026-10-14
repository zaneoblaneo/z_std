pub mod string_view;

pub mod quad_tree_f64;
pub mod quad_tree_f32;

#[cfg(test)]
mod tests {
    use std::io::Write;
    use super::quad_tree_f32::{ CenterSquare, Point, QuadTree };

    #[test]
    fn quad_tree_f32_test() {
        let mut qt: QuadTree = QuadTree::new(
            CenterSquare { 
                center: Point::new(0f32, 0f32), 
                radius: 14f32 
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
            center: (7f32, 7f32).into(),
            radius: 7f32,
        });
        assert_eq!(format!("{out:?}"), "[Point { x: 8.0, y: 6.0 }, Point { x: 1.0, y: 1.0 }, Point { x: 8.0, y: 3.0 }]");
    }
}

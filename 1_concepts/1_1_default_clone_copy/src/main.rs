#[derive(Copy, Clone, Default)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Polyline {
    points: Vec<Point>,
}

impl Polyline {
    fn new(points: Vec<Point>) -> Option<Self> {
        (!points.is_empty()).then_some(Self { points })
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();
    let p3 = p1;
    let p4: Point = Default::default();

    assert_eq!(p2.x, 1);
    assert_eq!(p2.y, 2);

    assert_eq!(p3.x, 1);
    assert_eq!(p3.y, 2);

    assert_eq!(p4.x, 0);
    assert_eq!(p4.y, 0);

    if let Some(poly1) = Polyline::new(vec![p1, p2, p3]) {
        let poly2 = poly1.clone();

        assert_eq!(poly2.points.len(), 3);
    }
}

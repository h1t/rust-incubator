use std::marker::PhantomData;

#[derive(Copy, Clone, Default)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
enum EmptyPolylineState {}

#[derive(Clone)]
enum NonEmptyPolylineState {}

pub trait PolylineState {}

impl PolylineState for EmptyPolylineState {}
impl PolylineState for NonEmptyPolylineState {}

#[derive(Clone)]
pub struct Polyline<S: PolylineState> {
    points: Vec<Point>,
    _state: PhantomData<S>,
}

impl Polyline<EmptyPolylineState> {
    pub fn with_point(p: Point) -> Polyline<NonEmptyPolylineState> {
        Polyline {
            points: vec![p],
            _state: PhantomData,
        }
    }
}

impl Polyline<NonEmptyPolylineState> {
    pub fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }

    pub fn add_points<I: IntoIterator<Item = Point>>(&mut self, iter: I) {
        self.points.extend(iter.into_iter());
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();
    let p3 = p1;
    let p4: Point = Default::default();

    assert_eq!(p1.x, 1);
    assert_eq!(p1.y, 2);

    assert_eq!(p2.x, 1);
    assert_eq!(p2.y, 2);

    assert_eq!(p3.x, 1);
    assert_eq!(p3.y, 2);

    assert_eq!(p4.x, 0);
    assert_eq!(p4.y, 0);

    let mut poly = Polyline::with_point(p1);
    poly.add_points(vec![p2, p3]);
    poly.add_point(p4);

    assert_eq!(poly.points.len(), 4);

    let poly1 = poly.clone();
    assert_eq!(poly1.points.len(), 4);
}

use geom::p2::P2;
use geom::rect::Rect;

#[cfg(test)]
pub mod reference;

pub trait Accel2D {
    type Scalar;
    type Item;

    fn new() -> Self;

    fn new_from_vec(items: Vec<(P2<Self::Scalar>, Self::Item)>) -> Self
    where
        Self: Sized,
    {
        let mut accel2d = Self::new();
        accel2d.insert(items);
        accel2d
    }

    fn insert(&mut self, mut items: Vec<(P2<Self::Scalar>, Self::Item)>) {
        for item in items.drain(..) {
            self.push(item);
        }
    }

    fn push(&mut self, item: (P2<Self::Scalar>, Self::Item));

    fn query_rect(&self, rect: &Rect<Self::Scalar>) -> Vec<&(P2<Self::Scalar>, Self::Item)>;
}

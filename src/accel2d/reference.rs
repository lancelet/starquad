use accel2d::Accel2D;
use geom::interval::IntervalDomain;
use geom::p2::P2;
use geom::rect::Rect;

/// Reference 2D acceleration structure.
///
/// This reference acceleration structure stores values in a `Vec`. It doesn't
/// do any actual acceleration, because all queries traverse all of the items
/// contained in the `Vec`.
///
/// Because it is so slow, this should only be used in tests as a reference,
/// not in actual release code.
pub struct Reference<S, T> {
    items: Vec<(P2<S>, T)>,
}

impl<S, T> Accel2D for Reference<S, T>
where
    S: IntervalDomain,
{
    type Scalar = S;
    type Item = T;

    fn new() -> Self {
        Reference { items: Vec::new() }
    }

    fn push(&mut self, item: (P2<S>, T)) {
        self.items.push(item);
    }

    fn query_rect(&self, rect: &Rect<S>) -> Vec<&(P2<S>, T)> {
        self.items
            .iter()
            .filter(|(point, _item)| rect.contains(point))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use accel2d::reference::Reference;
    use accel2d::Accel2D;
    use geom::p2::P2;
    use geom::rect::Rect;

    #[test]
    fn query_rect() {
        let items = vec![
            (P2::new(1.0, 1.0), String::from("A")),
            (P2::new(2.0, 4.0), String::from("B")),
            (P2::new(2.0, 7.0), String::from("C")),
            (P2::new(4.0, 6.0), String::from("D")),
            (P2::new(5.0, 4.0), String::from("E")),
            (P2::new(6.0, 1.0), String::from("F")),
            (P2::new(7.0, 3.0), String::from("G")),
            (P2::new(9.0, 1.0), String::from("H")),
            (P2::new(9.0, 3.0), String::from("I")),
            (P2::new(9.0, 7.0), String::from("J")),
        ];
        let accel2d = Reference::new_from_vec(items);
        let rect = Rect::new(3.0, 2.0, 5.0, 3.0).unwrap();
        let query = accel2d.query_rect(&rect);
        let mut str_item_vec = query
            .iter()
            .map(|(_point, item)| item)
            .cloned()
            .collect::<Vec<String>>();
        str_item_vec.sort();
        assert_eq!(str_item_vec, vec![String::from("E"), String::from("G")]);
    }
}

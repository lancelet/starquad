use geom::interval::{Interval, IntervalDomain};
use geom::p2::P2;

#[derive(Debug, PartialEq, Clone)]
pub struct Rect<S> {
    x_interval: Interval<S>,
    y_interval: Interval<S>,
}

impl<S> Rect<S>
where
    S: IntervalDomain,
{
    pub fn new(x: S, y: S, width: S, height: S) -> Option<Self> {
        Interval::new(x, width).and_then(|x_interval| {
            Interval::new(y, height).map(|y_interval| Rect {
                x_interval,
                y_interval,
            })
        })
    }

    pub fn new_from_intervals(x_interval: Interval<S>, y_interval: Interval<S>) -> Self {
        Rect {
            x_interval,
            y_interval,
        }
    }

    pub fn x(&self) -> &S {
        self.x_interval.start()
    }

    pub fn y(&self) -> &S {
        self.y_interval.start()
    }

    pub fn width(&self) -> &S {
        self.x_interval.diameter()
    }

    pub fn height(&self) -> &S {
        self.y_interval.diameter()
    }

    pub fn contains(&self, point: &P2<S>) -> bool {
        self.x_interval.contains(&point.x) && self.y_interval.contains(&point.y)
    }

    pub fn intersect(&self, other: &Rect<S>) -> Option<Self> {
        self.x_interval
            .intersect(&other.x_interval)
            .and_then(|intersecting_x_interval| {
                self.y_interval
                    .intersect(&other.y_interval)
                    .map(|intersecting_y_interval| Rect {
                        x_interval: intersecting_x_interval,
                        y_interval: intersecting_y_interval,
                    })
            })
    }
}

#[cfg(test)]
mod test {
    use geom::interval::{Interval, IntervalDomain};
    use geom::p2::P2;
    use geom::rect::Rect;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    #[test]
    fn new() {
        let rect = Rect::new(4, 5, 42, 50).unwrap();
        assert_eq!(rect.x(), &4);
        assert_eq!(rect.y(), &5);
        assert_eq!(rect.width(), &42);
        assert_eq!(rect.height(), &50);
    }

    #[test]
    fn contains() {
        let rect = Rect::new(4, 5, 42, 50).unwrap();
        assert!(!rect.contains(&P2::new(3, 7)));
        assert!(rect.contains(&P2::new(6, 40)));
    }

    #[test]
    fn intersect() {
        let rect_a = Rect::new(2.0, 1.0, 2.0, 4.0).unwrap();
        let rect_b = Rect::new(1.0, 3.0, 5.0, 3.0).unwrap();
        let expected = Rect::new(2.0, 3.0, 2.0, 2.0).unwrap();
        assert_eq!(rect_a.intersect(&rect_b), Some(expected));
    }

    impl<S> Arbitrary for Rect<S>
    where
        Interval<S>: Arbitrary,
        S: Clone + IntervalDomain,
    {
        fn arbitrary<G>(g: &mut G) -> Self
        where
            G: Gen,
        {
            let x_interval = Interval::<S>::arbitrary(g);
            let y_interval = Interval::<S>::arbitrary(g);
            Rect::new_from_intervals(x_interval, y_interval)
        }
    }

    #[quickcheck]
    fn f64_intersection_point_membership(a: Rect<f64>, b: Rect<f64>, point: P2<f64>) {
        let opt_intersection = a.intersect(&b);
        if a.contains(&point) && b.contains(&point) {
            assert!(opt_intersection.unwrap().contains(&point));
        } else {
            if let Some(intersection) = opt_intersection {
                assert!(!intersection.contains(&point));
            }
        }
    }
}

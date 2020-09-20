use num::{CheckedAdd, CheckedSub, Num};

/// Bounded interval.
///
/// An interval consists of a contiguous region of either the integer or the
/// real number line. An interval is characterised by its `start` and its
/// `diameter` (also called the length, width, measure, range or size of the
/// interval).
///
/// When intervals are created, the `diameter` is normalized so that it is
/// always positive. Eg:
///
/// ```
/// # use starquad::geom::interval::Interval;
/// let interval = Interval::new(3, -2).unwrap();
/// assert_eq!(interval.start(), &1);
/// assert_eq!(interval.diameter(), &2);
/// ```
///
/// An interval may only represent values that are reachable for the data type
/// they are parameterised by. For example, the `u8` data type can only
/// represent the values `[0, 255]`, and so it is not valid to create an
/// interval that includes the value 256 (even though 250 and 7 are perfectly
/// valid `u8` values):
///
/// ```
/// # use starquad::geom::interval::Interval;
/// assert_eq!(Interval::<u8>::new(250, 7), None);
/// ```
///
/// # Integer intervals
///
/// Intervals for integer values contain all values between `start` and
/// `start + diameter - 1` inclusive. This is so that an integer interval of
/// width `N` contains exactly `N` integers. For example, an interval of width
/// 2 contains exactly 2 integers:
///
/// ```
/// # use starquad::geom::interval::Interval;
/// let int_interval = Interval::new(2, 2).unwrap();
/// // should include values between 2 and 3 inclusive
/// assert!(!int_interval.contains(&1));  // doesn't contain 1
/// assert!(int_interval.contains(&2));   // does contain 2
/// assert!(int_interval.contains(&3));   // does contain 3
/// assert!(!int_interval.contains(&4));  // doesn't contain 4
/// ```
///
/// # Floating-point intervals
///
/// Intervals for floating-point values are similar to integer intervals, but
/// they are closed on the left (at `start`), and open on the right. They
/// contain values greater than or equal to `start` and less than
/// `start + diameter`:
///
/// ```
/// # use starquad::geom::interval::Interval;
/// let float_interval = Interval::new(2.0, 2.0).unwrap();
/// // should include 2.0, but exclude 4.0
/// assert!(float_interval.contains(&2.0));
/// assert!(float_interval.contains(&3.99999));
/// assert!(!float_interval.contains(&4.0));
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Interval<S> {
    start: S,
    diameter: S,
}

impl<S> Interval<S>
where
    S: IntervalDomain,
{
    /// Create a new interval.
    pub fn new(start: S, diameter: S) -> Option<Interval<S>> {
        S::new_interval(start, diameter)
    }

    pub fn start(&self) -> &S {
        &self.start
    }

    pub fn diameter(&self) -> &S {
        &self.diameter
    }

    /// Check if an interval contains a value.
    pub fn contains(&self, value: &S) -> bool {
        if value < &self.start {
            false
        } else {
            let end = self.start.clone() + self.diameter.clone();
            value < &end
        }
    }

    /// Return the intersection of two intervals if it exists.
    pub fn intersect(&self, other: &Interval<S>) -> Option<Interval<S>> {
        if self.start > other.start {
            other.intersect(self)
        } else {
            let self_end = self.start.clone() + self.diameter.clone();
            if self_end < other.start {
                None
            } else {
                let other_end = other.start.clone() + other.diameter.clone();
                if self_end < other_end {
                    let new_diameter = self_end - other.start.clone();
                    Interval::new(other.start.clone(), new_diameter)
                } else {
                    Some(other.clone())
                }
            }
        }
    }
}

/// A trait for types that can form the domain of an `Interval`.
///
/// The primary requirement of implementations of this trait is that they can
/// construct a new `Interval`. In doing so, they must perform normalization
/// of the `diameter` and also check that the range of the type is not
/// exceeded. See [new_int_interval](new_int_interval) and
/// [new_float_interval](new_float_interval) for examples of how this is done.
pub trait IntervalDomain: Clone + Num + PartialOrd {
    fn new_interval(start: Self, diameter: Self) -> Option<Interval<Self>>
    where
        Self: Sized;
}

//// Intervals of different types

/// Create a new `Interval` on an integer-like domain (with checked operations
/// for addition and subtraction).
pub fn new_int_interval<S>(start: S, diameter: S) -> Option<Interval<S>>
where
    S: Num + Clone + PartialOrd + CheckedAdd + CheckedSub,
{
    if diameter < S::zero() {
        start.checked_add(&diameter).and_then(|new_start| {
            S::zero()
                .checked_sub(&diameter)
                .and_then(|new_diameter| new_int_interval(new_start, new_diameter))
        })
    } else {
        diameter.checked_sub(&S::one()).and_then(|ofs| {
            start
                .checked_add(&ofs)
                .map(|_last_included_value| Interval { start, diameter })
        })
    }
}

/// Create a new `Interval` on a float-like domain (without checked operations
/// for addition and subtraction).
pub fn new_float_interval<S>(start: S, diameter: S) -> Option<Interval<S>>
where
    S: Num + Clone + PartialOrd,
{
    if diameter < S::zero() {
        new_float_interval(start + diameter.clone(), S::zero() - diameter)
    } else {
        Some(Interval { start, diameter })
    }
}

macro_rules! create_float_interval_ops {
    ($t:ty) => {
        impl IntervalDomain for $t {
            fn new_interval(start: $t, diameter: $t) -> Option<Interval<$t>> {
                new_float_interval(start, diameter)
            }
        }
    };
}

macro_rules! create_int_interval_ops {
    ($t:ty) => {
        impl IntervalDomain for $t {
            fn new_interval(start: $t, diameter: $t) -> Option<Interval<$t>> {
                new_int_interval(start, diameter)
            }
        }
    };
}

create_int_interval_ops!(i8);
create_int_interval_ops!(i16);
create_int_interval_ops!(i32);
create_int_interval_ops!(i64);
create_int_interval_ops!(i128);

create_int_interval_ops!(u8);
create_int_interval_ops!(u16);
create_int_interval_ops!(u32);
create_int_interval_ops!(u64);
create_int_interval_ops!(u128);

create_float_interval_ops!(f32);
create_float_interval_ops!(f64);

//// Tests

#[cfg(test)]
pub mod test {
    use geom::interval::{Interval, IntervalDomain};
    use paste::paste;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    #[test]
    fn new_simple() {
        let interval = Interval::<u8>::new(5, 42).expect("new interval");
        assert_eq!(interval.start(), &5);
        assert_eq!(interval.diameter(), &42);
    }

    #[test]
    fn new_normalizes_diameter_int() {
        let interval = Interval::<i8>::new(7, -4).expect("new interval");
        assert_eq!(interval.start(), &3);
        assert_eq!(interval.diameter(), &4);
    }

    #[test]
    fn new_normalizes_diameter_float() {
        let interval = Interval::<f32>::new(7.0, -4.0).expect("new interval");
        assert_eq!(interval.start(), &3.0);
        assert_eq!(interval.diameter(), &4.0);
    }

    #[test]
    fn new_fails_when_out_of_range_positive() {
        assert!(Interval::<i8>::new(127, 1).is_some()); // just in range
        assert!(Interval::<i8>::new(127, 2).is_none()); // just out of range
    }

    #[test]
    fn new_fails_when_out_of_range_negative() {
        assert!(Interval::<i8>::new(-127, -1).is_some()); // just in range
        assert!(Interval::<i8>::new(-127, -2).is_none()); // just out of range
    }

    #[test]
    fn intersect_int() {
        fn intersect_both(
            interval_a: &Interval<u8>,
            interval_b: &Interval<u8>,
            expected: Option<Interval<u8>>,
        ) {
            let ab = interval_a.intersect(&interval_b);
            let ba = interval_b.intersect(&interval_a);
            assert_eq!(ab, expected);
            assert_eq!(ba, expected);
        }

        let a = Interval::new(4, 3).unwrap();
        let t1 = Interval::new(1, 2).unwrap();
        let t2 = Interval::new(1, 3).unwrap();
        let t3 = Interval::new(1, 5).unwrap();
        let t4 = Interval::new(1, 7).unwrap();

        intersect_both(&a, &t1, None);
        intersect_both(&a, &t2, None);
        intersect_both(&a, &t3, Interval::new(4, 2));
        intersect_both(&a, &t4, Interval::new(4, 3));
    }

    #[test]
    fn contains_int() {
        let interval = Interval::<u8>::new(2, 2).unwrap();
        assert!(!interval.contains(&1));
        assert!(interval.contains(&2));
        assert!(interval.contains(&3));
        assert!(!interval.contains(&4));
    }

    #[test]
    fn contains_float() {
        let interval = Interval::<f32>::new(2.0, 2.1).unwrap();
        assert!(!interval.contains(&1.0));
        assert!(interval.contains(&2.0));
        assert!(interval.contains(&4.0));
        assert!(interval.contains(&4.09999));
        assert!(!interval.contains(&4.1));
    }

    macro_rules! create_arbitrary_int_interval {
        ($t:ty) => {
            impl Arbitrary for Interval<$t> {
                fn arbitrary<G>(g: &mut G) -> Self
                where
                    G: Gen,
                {
                    let mut start: $t;
                    let mut end: $t;
                    let diameter: $t;
                    loop {
                        start = <$t>::arbitrary(g);
                        end = <$t>::arbitrary(g);
                        let opt_diameter = end
                            .checked_sub(start)
                            .and_then(|pdiam| pdiam.checked_add(1));
                        if let Some(d) = opt_diameter {
                            diameter = d;
                            break;
                        }
                    }
                    Interval::new(start, diameter).expect("Arbitrary int interval")
                }
            }
        };
    }

    macro_rules! create_arbitrary_float_interval {
        ($t:ty) => {
            impl Arbitrary for Interval<$t> {
                fn arbitrary<G>(g: &mut G) -> Self
                where
                    G: Gen,
                {
                    let start = <$t>::arbitrary(g);
                    let end = <$t>::arbitrary(g);
                    let diameter = end - start;
                    Interval::new(start, diameter).expect("Arbitrary float interval")
                }
            }
        };
    }

    create_arbitrary_int_interval!(i8);
    create_arbitrary_int_interval!(i16);
    create_arbitrary_int_interval!(i32);
    create_arbitrary_int_interval!(i64);
    create_arbitrary_int_interval!(i128);

    create_arbitrary_int_interval!(u8);
    create_arbitrary_int_interval!(u16);
    create_arbitrary_int_interval!(u32);
    create_arbitrary_int_interval!(u64);
    create_arbitrary_int_interval!(u128);

    create_arbitrary_float_interval!(f32);
    create_arbitrary_float_interval!(f64);

    /// Property test for consistency between `contains` and `intersection`.
    ///
    /// If two intervals both contain a value then their intersection must
    /// also contain that value. Conversely, if they do not both contain
    /// the value then, if an intersection exists, it must *not* contain the
    /// value.
    fn intersection_point_membership<S>(a: Interval<S>, b: Interval<S>, value: S)
    where
        S: IntervalDomain,
    {
        let opt_intersection = a.intersect(&b);
        if a.contains(&value) && b.contains(&value) {
            assert!(opt_intersection.unwrap().contains(&value));
        } else {
            if let Some(intersection) = opt_intersection {
                assert!(!intersection.contains(&value));
            }
        }
    }

    macro_rules! check_intersection_point_membership {
        ($t:ty) => {
            paste! {
                #[quickcheck]
                fn [<$t _intersection_point_membership>](
                    a: Interval<$t>,
                    b: Interval<$t>,
                    value: $t
                ) {
                    intersection_point_membership(a, b, value);
                }
            }
        };
    }

    check_intersection_point_membership!(i8);
    check_intersection_point_membership!(i16);
    check_intersection_point_membership!(i32);
    check_intersection_point_membership!(i64);
    check_intersection_point_membership!(i128);
    check_intersection_point_membership!(u8);
    check_intersection_point_membership!(u16);
    check_intersection_point_membership!(u32);
    check_intersection_point_membership!(u64);
    check_intersection_point_membership!(u128);
    check_intersection_point_membership!(f32);
    check_intersection_point_membership!(f64);
}

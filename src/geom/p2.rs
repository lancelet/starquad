#[derive(Debug, Clone, PartialEq)]
pub struct P2<S> {
    pub x: S,
    pub y: S,
}

impl<S> P2<S> {
    pub fn new(x: S, y: S) -> Self {
        P2 { x, y }
    }
}

#[cfg(test)]
mod test {
    use geom::p2::P2;
    use quickcheck::{Arbitrary, Gen};

    impl<S> Arbitrary for P2<S>
    where
        S: Arbitrary,
    {
        fn arbitrary<G>(g: &mut G) -> Self
        where
            G: Gen,
        {
            let x = S::arbitrary(g);
            let y = S::arbitrary(g);
            P2::new(x, y)
        }
    }
}

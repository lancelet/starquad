pub struct P2<S> {
    pub x: S,
    pub y: S,
}

impl<S> P2<S> {
    pub fn new(x: S, y: S) -> Self {
        P2 { x, y }
    }
}

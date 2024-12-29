extern crate overload;
use overload::overload;
use std::ops;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord(pub i64, pub i64);

impl Coord {
    pub fn bound_checked(&self, lower: Coord, upper: Coord) -> Option<Coord> {
        if self.0 >= lower.0
            && self.1 >= lower.1
            && self.0 < upper.0
            && self.1 < upper.1
        {
            Some(*self)
        } else {
            None
        }
    }

    pub fn rotate_left(&self) -> Coord {
        Coord(self.1, -self.0)
    }

    pub fn rotate_right(&self) -> Coord {
        Coord(-self.1, self.0)
    }
}

overload!((a: ?Coord) + (b: ?Coord) -> Coord { Coord(a.0 + b.0, a.1 + b.1) });
overload!((a: ?Coord) - (b: ?Coord) -> Coord { Coord(a.0 - b.0, a.1 - b.1) });
overload!((a: ?i64) * (b: ?Coord) -> Coord { Coord(a * b.0, a * b.1) });
overload!((a: ?Coord) * (b: ?i64) -> Coord { b * a });
overload!((a: &mut Coord) += (b: ?Coord) { a.0 += b.0; a.1 += b.1 });
overload!((a: &mut Coord) *= (b: ?i64) { a.0 *= b; a.1 *= b });
overload!(- (a: ?Coord) -> Coord { Coord(-a.0, -a.1) });

impl From<(i64, i64)> for Coord
{
    fn from(value: (i64, i64)) -> Coord {
        Coord(value.0, value.1)
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

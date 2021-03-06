use alga::general::Real;

/// A torus.
#[derive(PartialEq, Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct Torus<N> {
    major_radius: N,
    minor_radius: N
}

impl<N> Torus<N> {
    /// Creates a new torus with the given radiuses.
    #[inline]
    pub fn new(major_radius: N, minor_radius: N) -> Torus<N> {
        Torus {
            major_radius: major_radius,
            minor_radius: minor_radius
        }
    }
}

impl<N: Real> Torus<N> {
    /// The torus minor radius.
    #[inline]
    pub fn minor_radius(&self) -> N {
        self.minor_radius
    }

    /// The torus major radius.
    #[inline]
    pub fn major_radius(&self) -> N {
        self.major_radius
    }
}

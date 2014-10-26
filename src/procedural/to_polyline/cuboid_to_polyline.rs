use na;
use geom::Cuboid;
use procedural::{Polyline, ToPolyline};
use procedural;
use math::{Scalar, Point, Vect};


impl ToPolyline<()> for Cuboid {
    fn to_polyline(&self, _: ()) -> Polyline<Scalar, Point, Vect> {
        let _2: Scalar = na::cast(2.0f64);

        procedural::rectangle(&(self.half_extents() * _2))
    }
}
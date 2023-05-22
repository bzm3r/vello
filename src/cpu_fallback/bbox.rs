// The annotated bounding box for a path. It has been transformed,
// but contains a link to the active transform, mostly for gradients.
// Coordinates are integer pixels (for the convenience of atomic update)
// but will probably become fixed-point fractions for rectangles.
struct PathBbox {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    linewidth: f32,
    trans_ix: u32,
}

pub struct Bbox<T>
where
    T: Ord + Copy,
{
    x0: T,
    y0: T,
    x1: T,
    y1: T,
}

impl<T> Bbox<T>
where
    T: Ord + Copy,
{
    pub fn intersects(&self, other: &Bbox<T>) -> Bbox<T> {
        Bbox {
            x0: self.x0.max(other.x0),
            y0: self.y0.max(other.y0),
            x1: self.x1.min(other.x1),
            y1: self.y1.min(other.y1),
        }
    }
}

use crate::Extent2d;

#[repr(C, align(16))]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Extent3d {
    pub width: u32,
    pub height: u32,
    pub depth_or_array_layers: u32,
}

impl From<Extent3d> for Extent2d {
    fn from(extend_3d: Extent3d) -> Self {
        Extent2d {
            width: extend_3d.width,
            height: extend_3d.height,
        }
    }
}

pub use direction::*;
pub use frozen::*;
pub use local_world::*;
pub use orientation::*;
pub use parent::*;
pub use position::*;
pub use rotation::*;
pub use scale::*;
pub use scale_ty::*;
pub use space::*;
pub use space_ty::*;
pub use world_local::*;

mod direction;
mod frozen;
mod local_world;
mod orientation;
mod parent;
mod position;
mod rotation;
mod scale;
mod scale_ty;
mod space;
mod space_ty;
mod world_local;


#[cfg(test)]
mod data_test {
    use crate::container::rotation::Rotation;
    use crate::{AxisDirection, Frozen, Orientation, Parent, Position, Scale, Space};

    #[test]
    fn data_size() {
        let direction_size = std::mem::size_of::<AxisDirection>();
        assert_eq!(direction_size & (direction_size - 1), 0);

        let frozen_size = std::mem::size_of::<Frozen>();
        assert!(frozen_size.eq(&0));

        let parent_size = std::mem::size_of::<Parent>();
        assert_eq!(parent_size & (parent_size - 1), 0);

        let space_size = std::mem::size_of::<Space>();
        assert_eq!(space_size & (space_size - 1), 0);

        let position_size = std::mem::size_of::<Position>();
        assert_eq!(position_size & (position_size - 1), 0);

        let rotation_size = std::mem::size_of::<Rotation>();
        assert_eq!(rotation_size & (rotation_size - 1), 0);

        let scale_size = std::mem::size_of::<Scale>();
        assert_eq!(scale_size & (scale_size - 1), 0);

        let orientation_size = std::mem::size_of::<Orientation>();
        assert_eq!(orientation_size & (orientation_size - 1), 0);
    }


    #[test]
    fn data_alignment() {
        let direction_alignment = std::mem::align_of::<AxisDirection>();
        assert_eq!(direction_alignment & (direction_alignment - 1), 0);

        let frozen_alignment = std::mem::align_of::<Frozen>();
        assert_eq!(frozen_alignment & (frozen_alignment - 1), 0);

        let parent_alignment = std::mem::align_of::<Parent>();
        assert_eq!(parent_alignment & (parent_alignment - 1), 0);

        let space_alignment = std::mem::align_of::<Space>();
        assert_eq!(space_alignment & (space_alignment - 1), 0);

        let position_alignment = std::mem::align_of::<Position>();
        assert_eq!(position_alignment & (position_alignment - 1), 0);

        let rotation_alignment = std::mem::align_of::<Rotation>();
        assert_eq!(rotation_alignment & (rotation_alignment - 1), 0);

        let scale_alignment = std::mem::align_of::<Scale>();
        assert_eq!(scale_alignment & (scale_alignment - 1), 0);

        let orientation_alignment = std::mem::align_of::<Orientation>();
        assert_eq!(orientation_alignment & (orientation_alignment - 1), 0);
    }
}

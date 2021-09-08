mod aspect_ratio;
mod camera;
mod clipping_plane;
mod fov;
mod orientation;
mod orthographic;
mod perspective;
mod projection;
mod viewport;

pub use aspect_ratio::*;
pub use camera::*;
pub use clipping_plane::*;
pub use fov::*;
pub use orientation::*;
pub use orthographic::*;
pub use perspective::*;
pub use projection::*;
pub use viewport::*;

#[cfg(test)]
mod data_test {
    use crate::camera::{
        AspectRatio, Camera, ClippingPlane, Fov, FovAxis, FovScalingAlgorithm, Orientation,
        Orthographic, Perspective, Projection, ViewPort,
    };

    #[test]
    fn data_size() {
        let orthographic_size = std::mem::size_of::<Orthographic>();
        println!("Orthographic {}", orthographic_size);

        let perspective_size = std::mem::size_of::<Perspective>();
        println!("Perspective {}", perspective_size);

        let projection_size = std::mem::size_of::<Projection>();
        assert_eq!(projection_size & (projection_size - 1), 0);


        let viewport_rect_size = std::mem::size_of::<ViewPort>();
        println!("Viewport {}", viewport_rect_size);


        let camera_orientation_size = std::mem::size_of::<Orientation>();
        assert_eq!(camera_orientation_size & (camera_orientation_size - 1), 0);

        let camera_matrix_size = std::mem::size_of::<Camera>();
        assert_eq!(camera_matrix_size & (camera_matrix_size - 1), 0);

        let fov_axis_size = std::mem::size_of::<FovAxis>();
        assert_eq!(fov_axis_size & (fov_axis_size - 1), 0);

        let fov_scaling_algorithm_size = std::mem::size_of::<FovScalingAlgorithm>();
        assert_eq!(
            fov_scaling_algorithm_size & (fov_scaling_algorithm_size - 1),
            0
        );

        let fov_size = std::mem::size_of::<Fov>();
        assert_eq!(fov_size & (fov_size - 1), 0);

        let clipping_plane_size = std::mem::size_of::<ClippingPlane>();
        assert_eq!(clipping_plane_size & (clipping_plane_size - 1), 0);

        let aspect_ratio_size = std::mem::size_of::<AspectRatio>();
        assert_eq!(aspect_ratio_size & (aspect_ratio_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let orthographic_alignment = std::mem::align_of::<Orthographic>();
        assert_eq!(orthographic_alignment & (orthographic_alignment - 1), 0);


        let perspective_alignment = std::mem::align_of::<Perspective>();
        assert_eq!(perspective_alignment & (perspective_alignment - 1), 0);


        let projection_alignment = std::mem::align_of::<Projection>();
        assert_eq!(projection_alignment & (projection_alignment - 1), 0);


        let camera_orientation_alignment = std::mem::align_of::<Orientation>();
        assert_eq!(
            camera_orientation_alignment & (camera_orientation_alignment - 1),
            0
        );

        let camera_matrix_alignment = std::mem::align_of::<Camera>();
        assert_eq!(camera_matrix_alignment & (camera_matrix_alignment - 1), 0);

        let viewport_alignment = std::mem::align_of::<ViewPort>();
        assert_eq!(viewport_alignment & (viewport_alignment - 1), 0);

        let fov_axis_alignment = std::mem::align_of::<FovAxis>();
        assert_eq!(fov_axis_alignment & (fov_axis_alignment - 1), 0);

        let fov_scaling_algorithm_alignment = std::mem::align_of::<FovScalingAlgorithm>();
        assert_eq!(
            fov_scaling_algorithm_alignment & (fov_scaling_algorithm_alignment - 1),
            0
        );

        let fov_alignment_alignment = std::mem::align_of::<Fov>();
        assert_eq!(fov_alignment_alignment & (fov_alignment_alignment - 1), 0);


        let clipping_plane_alignment = std::mem::align_of::<ClippingPlane>();
        assert_eq!(clipping_plane_alignment & (clipping_plane_alignment - 1), 0);

        let aspect_ratio_alignment = std::mem::align_of::<AspectRatio>();
        assert_eq!(aspect_ratio_alignment & (aspect_ratio_alignment - 1), 0);
    }
}

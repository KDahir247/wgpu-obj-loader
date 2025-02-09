use crate::camera::{Aperture, FishLens, Fov, FovAxis};

// all angle are in radians

// focal length in millimeter
// frame_aperture in millimeter
// set frame aperture x if fov axis is horizontal, otherwise set aperture y if
// fov axis is vertical (set the other to zero.)
pub fn focal_length_to_fov(
    focal_length: f32,
    frame_aperture: Aperture,
    fov_axis: FovAxis,
    focus_distance: Option<f32>,
    crop_factor: Option<f32>,
    lens_type: FishLens,
) -> (Fov, f32) {
    let frame_size = (frame_aperture.aperture_x_mm * frame_aperture.aperture_x_mm).sqrt();

    let crop_focal_length = focal_length * crop_factor.unwrap_or(1.0);

    let (rad_angle, magnification) =
        internal_focal_to_fov(crop_focal_length, frame_size, focus_distance, lens_type);

    (
        Fov {
            radian: rad_angle,
            axis: fov_axis,
        },
        magnification,
    )
}

pub fn focal_length_to_directional_fov(
    focal_length: f32,
    frame_aperture: Aperture,
    focus_distance: Option<f32>,
    crop_factor: Option<f32>,
    lens_type: FishLens,
) -> (f32, f32) {
    let frame_size = (frame_aperture.aperture_x_mm * frame_aperture.aperture_x_mm
        + frame_aperture.aperture_y_mm * frame_aperture.aperture_y_mm)
        .sqrt();

    internal_focal_to_fov(
        focal_length * crop_factor.unwrap_or(1.0),
        frame_size,
        focus_distance,
        lens_type,
    )
}


fn internal_focal_to_fov(
    focal_length: f32,
    frame_size: f32,
    focus_distance: Option<f32>,
    lens_type: FishLens,
) -> (f32, f32) {
    let magnification = focus_distance
        .map(|focus_distance| compute_approx_magnification(focal_length, focus_distance) + 1.0)
        .unwrap_or(1.0);

    let fov_result = match lens_type {
        FishLens::Rectilinear => (frame_size / (focal_length * 2.0 * magnification)).atan() * 2.0,
        FishLens::Stereographic => (frame_size / (focal_length * 4.0)).atan() * 4.0,
        FishLens::Equidistant => (frame_size / focal_length) * 57.3,
        FishLens::EquisolidAngle => (frame_size / (focal_length * 4.0)).asin() * 4.0,
        FishLens::Orthographic => (frame_size / (focal_length * 2.0)).asin() * 2.0,
    };

    (fov_result, magnification)
}

pub fn fov_to_focal_length(
    field_of_view: f32,
    frame_aperture: Aperture,
    crop_factor: Option<f32>,
    magnification: f32,
    lens_type: FishLens,
) -> f32 {
    let frame_size = (frame_aperture.aperture_x_mm * frame_aperture.aperture_x_mm
        + frame_aperture.aperture_y_mm * frame_aperture.aperture_y_mm)
        .sqrt();

    let focal_length = match lens_type {
        FishLens::Rectilinear => {
            frame_size
                / (2.0 * (field_of_view.to_degrees() * std::f32::consts::PI / 360.0).tan())
                / magnification
        }
        FishLens::Stereographic => {
            frame_size
                / (4.0 * (field_of_view.to_degrees() / 360.0 * std::f32::consts::FRAC_PI_2).tan())
        }
        FishLens::Equidistant => (frame_size * 57.3) / field_of_view,
        FishLens::EquisolidAngle => {
            frame_size
                / (4.0 * (field_of_view.to_degrees() / 360.0 * std::f32::consts::FRAC_PI_2).sin())
        }
        FishLens::Orthographic => {
            frame_size / (2.0 * (field_of_view.to_degrees() / 360.0 * std::f32::consts::PI).sin())
        }
    };


    focal_length / crop_factor.unwrap_or(1.0)
}

/// field of view should be converter to Vertical prior using convert_axis to
/// calling this calculation
pub fn compute_vertical_focal_length(aperture_size_y: f32, field_of_view: Fov) -> f32 {
    aperture_size_y / field_of_view.radian
}

/// field of view should be converted to Horizontal prior using convert_axis to
/// call this calculation
pub fn compute_horizontal_focal_length(aperture_size_x: f32, field_of_view: Fov) -> f32 {
    aperture_size_x / field_of_view.radian
}


pub fn compute_focal_length(
    distance_image_plane: f32,
    optical_axis_angle_rad: f32,
    lens_type: FishLens,
) -> f32 {
    match lens_type {
        FishLens::Rectilinear => distance_image_plane / optical_axis_angle_rad.atan(),
        FishLens::Stereographic => {
            distance_image_plane / 2.0 * (optical_axis_angle_rad / 2.0).tan()
        }
        FishLens::Equidistant => distance_image_plane / optical_axis_angle_rad,
        FishLens::EquisolidAngle => {
            distance_image_plane / (2.0 * (optical_axis_angle_rad / 2.0).sin())
        }
        FishLens::Orthographic => distance_image_plane / optical_axis_angle_rad.sin(),
    }
}

pub fn compute_distance_image_plane_from_optical_axis(
    focal_length: f32,
    optical_axis_angle: f32,
    lens_type: FishLens,
) -> f32 {
    match lens_type {
        FishLens::Rectilinear => focal_length * focal_length.tan(),
        FishLens::Stereographic => 2.0 * focal_length * (optical_axis_angle / 2.0).tan(),
        FishLens::Equidistant => focal_length * optical_axis_angle,
        FishLens::EquisolidAngle => 2.0 * focal_length * (optical_axis_angle / 2.0).sin(),
        FishLens::Orthographic => focal_length * optical_axis_angle.sin(),
    }
}


pub fn compute_exact_magnification(focal_length: f32, focus_distance: f32) -> f32 {
    let r = ((focus_distance * focus_distance) / 4.0 - focal_length * focus_distance).sqrt();
    let object_distance = focus_distance / 2.0 + r;
    let image_distance = focus_distance / 2.0 - r;

    image_distance / object_distance
}

pub fn compute_approx_magnification(focal_length: f32, focus_distance: f32) -> f32 {
    focal_length / (focus_distance - focal_length)
}


#[cfg(test)]
mod len_mapping {
    use crate::camera::{
        compute_approx_magnification, compute_distance_image_plane_from_optical_axis,
        compute_exact_magnification, compute_focal_length, focal_length_to_directional_fov,
        focal_length_to_fov, fov_to_focal_length, Aperture, FishLens, FovAxis,
    };

    #[test]
    fn magnification() {
        const ERROR_THRESHOLD: f32 = 0.09;
        // precise calculation for magnification will return nan if values are
        // incorrect.
        assert!(compute_exact_magnification(90.0, 200.0).is_nan());
        // approximate calculation for magnification will never return nan if if the
        // values are incorrect
        assert!(!compute_approx_magnification(90.0, 200.0).is_nan());

        let precise_magnification = compute_exact_magnification(20.0, 500.0);
        let approx_magnification = compute_approx_magnification(20.0, 500.0);

        assert!((precise_magnification - approx_magnification).abs() < ERROR_THRESHOLD);
    }


    #[test]
    fn focal_to_fov() {
        const ERROR_THRESHOLD: f32 = 0.0001;

        let focal_length = 15.0;

        let (vertical_fov, _) = focal_length_to_fov(
            focal_length,
            Aperture::new(28.0, 0.0),
            FovAxis::Vertical,
            None,
            None,
            FishLens::Rectilinear,
        );

        let (diagonal_fov, _) = focal_length_to_directional_fov(
            focal_length,
            Aperture::new(28.0, 0.0),
            None,
            None,
            FishLens::Rectilinear,
        );

        assert!(vertical_fov.axis.eq(&FovAxis::Vertical));

        assert!((vertical_fov.radian - diagonal_fov).abs() < ERROR_THRESHOLD);
    }

    #[test]
    fn focal_to_direction_fov() {
        const ERROR_THRESHOLD: f32 = 0.0001;

        // Calculated using.
        // http://kmp.pentaxians.eu/technology/fov/
        // result 110.52703743126978 degree
        let result = 110.527_04_f32.to_radians();

        let focal_length = 15.0;

        let (full_frame_fov, magnification) = focal_length_to_directional_fov(
            focal_length,
            Aperture::new(36., 24.),
            None,
            None,
            FishLens::Rectilinear,
        );

        assert!((full_frame_fov - result).abs() < ERROR_THRESHOLD);

        let full_frame_focal_length = fov_to_focal_length(
            full_frame_fov,
            Aperture::new(36., 24.),
            None,
            magnification,
            FishLens::Rectilinear,
        );

        assert!((full_frame_focal_length - focal_length).abs() < ERROR_THRESHOLD);
    }

    #[test]
    fn fov_to_focal() {
        const ERROR_THRESHOLD: f32 = 0.0001;
        const CROP_FACTOR: f32 = 1.5f32;

        let fov = 90.0f32.to_radians();


        let focal_length = fov_to_focal_length(
            fov,
            Aperture::new(36., 24.),
            Some(CROP_FACTOR),
            1.0,
            FishLens::Rectilinear,
        );


        let (resulting_fov, _) = focal_length_to_directional_fov(
            focal_length,
            Aperture::new(36., 24.),
            None,
            Some(CROP_FACTOR),
            FishLens::Rectilinear,
        );

        assert!((fov - resulting_fov).abs() < ERROR_THRESHOLD);
    }

    #[test]
    fn compute_focal() {
        // https://en.wikipedia.org/wiki/Fisheye_lens#cite_note-NotESA-68
        // circular APS-C (r = 8.4 mm)
        // Stereographic=4.2, Equidistant=5.3, Equisolid=5.9, Orthographic=8.4,

        const ERROR_THRESHOLD: f32 = 0.1;

        let circular_aps_c = [
            (FishLens::Stereographic, 4.2),
            (FishLens::Equidistant, 5.3),
            (FishLens::EquisolidAngle, 5.9),
            (FishLens::Orthographic, 8.4),
        ];


        for test_case in circular_aps_c {
            let circular_aps_c_focal_length =
                compute_focal_length(8.4, std::f32::consts::FRAC_PI_2, test_case.0);

            assert!((circular_aps_c_focal_length - test_case.1).abs() < ERROR_THRESHOLD);
        }

        // circular 135 (r = 12 mm)
        // Stereographic=6.0, Equidistant=7.6, Equisolid=8.5, Orthographic=12.0,


        let circular_135 = [
            (FishLens::Stereographic, 6.0),
            (FishLens::Equidistant, 7.6),
            (FishLens::EquisolidAngle, 8.5),
            (FishLens::Orthographic, 12.0),
        ];


        for test_case in circular_135 {
            let circular_135_focal_length =
                compute_focal_length(12.0, std::f32::consts::FRAC_PI_2, test_case.0);

            assert!((circular_135_focal_length - test_case.1).abs() < ERROR_THRESHOLD);
        }

        // circular 6x6 (r = 28 mm)
        // Stereographic=14.0, Equidistant=17.8, Equisolid=19.8,
        // Orthographic=28.0,

        let circular_6x6 = [
            (FishLens::Stereographic, 14.0),
            (FishLens::Equidistant, 17.8),
            (FishLens::EquisolidAngle, 19.8),
            (FishLens::Orthographic, 28.0),
        ];

        for test_case in circular_6x6 {
            let circular_6x6_focal_length =
                compute_focal_length(28.0, std::f32::consts::FRAC_PI_2, test_case.0);

            assert!((circular_6x6_focal_length - test_case.1).abs() < ERROR_THRESHOLD);
        }

        // full-frame APS-C (r = 15.1 mm)
        // Stereographic=7.5, Equidistant=9.6, Equisolid=10.6,
        // Orthographic=15.1,

        let full_frame_aps_c = [
            (FishLens::Stereographic, 7.5),
            (FishLens::Equidistant, 9.6),
            (FishLens::EquisolidAngle, 10.6),
            (FishLens::Orthographic, 15.1),
        ];

        for test_case in full_frame_aps_c {
            let full_frame_asp_c_focal_length =
                compute_focal_length(15.1, std::f32::consts::FRAC_PI_2, test_case.0);

            assert!((full_frame_asp_c_focal_length - test_case.1).abs() < ERROR_THRESHOLD);
        }

        // full-frame 135 (r = 21.7 mm)
        // Stereographic=10.8, Equidistant=13.8, Equisolid=15.3,
        // Orthographic=21.7,

        let full_frame_135 = [
            (FishLens::Stereographic, 10.8),
            (FishLens::Equidistant, 13.8),
            (FishLens::EquisolidAngle, 15.3),
            (FishLens::Orthographic, 21.7),
        ];

        for test_case in full_frame_135 {
            let full_frame_135_focal_length =
                compute_focal_length(21.7, std::f32::consts::FRAC_PI_2, test_case.0);

            assert!((full_frame_135_focal_length - test_case.1).abs() < ERROR_THRESHOLD);
        }

        // full-frame 6x6 (r = 39.6 mm)
        // Stereographic = 19.8, Equidistant=25.2, Equisolid=28.0,
        // Orthographic=39.6

        let full_frame_6x6 = [
            (FishLens::Stereographic, 19.8),
            (FishLens::Equidistant, 25.2),
            (FishLens::EquisolidAngle, 28.0),
            (FishLens::Orthographic, 39.6),
        ];

        for test_case in full_frame_6x6 {
            let full_frame_6x6_focal_length =
                compute_focal_length(39.6, std::f32::consts::FRAC_PI_2, test_case.0);

            assert!((full_frame_6x6_focal_length - test_case.1).abs() < ERROR_THRESHOLD);
        }
    }


    #[test]
    fn compute_distance_image_plane() {
        const ERROR_THRESHOLD: f32 = 0.000002;

        // circular APS-C (r = 8.4 mm)
        // Stereographic 4.2
        // Equidistant 5.3476057
        // EquisolidAngle 5.939697
        // Orthographic 8.4
        let circular_aps_c = [
            (FishLens::Stereographic, 4.2),
            (FishLens::Equidistant, 5.3476057),
            (FishLens::EquisolidAngle, 5.939697),
            (FishLens::Orthographic, 8.4),
        ];


        for test_case in circular_aps_c {
            let distance_image_plane_circular_aps_c =
                compute_distance_image_plane_from_optical_axis(
                    test_case.1,
                    std::f32::consts::FRAC_PI_2,
                    test_case.0,
                );

            assert!((distance_image_plane_circular_aps_c - 8.4).abs() < ERROR_THRESHOLD);
        }

        // circular 135 (r = 12.0 mm)
        // Stereographic 6.0
        // Equidistant 7.639437
        // EquisolidAngle 8.485282
        // Orthographic 12.0
        let circular_135 = [
            (FishLens::Stereographic, 6.0),
            (FishLens::Equidistant, 7.639437),
            (FishLens::EquisolidAngle, 8.485282),
            (FishLens::Orthographic, 12.0),
        ];

        for test_case in circular_135 {
            let distance_image_plane_circular_135 = compute_distance_image_plane_from_optical_axis(
                test_case.1,
                std::f32::consts::FRAC_PI_2,
                test_case.0,
            );


            assert!((distance_image_plane_circular_135 - 12.0) < ERROR_THRESHOLD);
        }

        // circular 6x6 (r = 28.0)
        // Stereographic 14.0
        // Equidistant 17.825354
        // EquisolidAngle 19.79899
        // Orthographic 28.0
        let circular_6x6 = [
            (FishLens::Stereographic, 14.0),
            (FishLens::Equidistant, 17.825354),
            (FishLens::EquisolidAngle, 19.79899),
            (FishLens::Orthographic, 28.0),
        ];

        for test_case in circular_6x6 {
            let distance_image_plane_circular_6x6 = compute_distance_image_plane_from_optical_axis(
                test_case.1,
                std::f32::consts::FRAC_PI_2,
                test_case.0,
            );

            assert!((distance_image_plane_circular_6x6 - 28.0) < ERROR_THRESHOLD);
        }

        // full frame APS-C (r = 15.1 mm)
        // Stereographic 7.55
        // Equidistant 9.612959
        // EquisolidAngle 10.677313
        // Orthographic 15.1
        let full_frame_aps_c = [
            (FishLens::Stereographic, 7.55),
            (FishLens::Equidistant, 9.612959),
            (FishLens::EquisolidAngle, 10.677313),
            (FishLens::Orthographic, 15.1),
        ];

        for test_case in full_frame_aps_c {
            let distance_image_plane_full_frame_aps_c =
                compute_distance_image_plane_from_optical_axis(
                    test_case.1,
                    std::f32::consts::FRAC_PI_2,
                    test_case.0,
                );

            assert!((distance_image_plane_full_frame_aps_c - 15.1) < ERROR_THRESHOLD);
        }

        // full frame 135 (r = 21.7 mm)
        // Stereographic 10.85
        // Equidistant 13.81465
        // EquisolidAngle 15.344218
        // Orthographic 21.7
        let full_frame_135 = [
            (FishLens::Stereographic, 10.85),
            (FishLens::Equidistant, 13.81465),
            (FishLens::EquisolidAngle, 15.344218),
            (FishLens::Orthographic, 21.7),
        ];

        for test_case in full_frame_135 {
            let distance_image_plane_full_frame_135 =
                compute_distance_image_plane_from_optical_axis(
                    test_case.1,
                    std::f32::consts::FRAC_PI_2,
                    test_case.0,
                );

            assert!((distance_image_plane_full_frame_135 - 21.7) < ERROR_THRESHOLD);
        }

        // full frame 6x6 (r = 39.6)
        // Stereographic 19.8
        // Equidistant 25.210142
        // EquisolidAngle 28.001429
        // Orthographic 39.6
        let full_frame_6x6 = [
            (FishLens::Stereographic, 19.8),
            (FishLens::Equidistant, 25.210142),
            (FishLens::EquisolidAngle, 28.001429),
            (FishLens::Orthographic, 39.6),
        ];

        for test_case in full_frame_6x6 {
            let distance_image_plane_full_6x6 = compute_distance_image_plane_from_optical_axis(
                test_case.1,
                std::f32::consts::FRAC_PI_2,
                test_case.0,
            );

            assert!((distance_image_plane_full_6x6 - 39.6) < 39.6);
        }
    }
}

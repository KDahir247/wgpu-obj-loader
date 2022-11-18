use crate::camera::ViewPort;
use fabled_math::matrix4x4_math::inverse_mat4;
use fabled_math::{Matrix4x4, Swizzles4, Vector2, Vector3, Vector4};

pub fn world_to_ndc_space(target: Vector4, model_view_projection: Matrix4x4) -> Vector3 {
    let dc = model_view_projection * target;
    let scalar = dc.w().recip();
    let ndc = dc * scalar;
    ndc.xyz()
}


pub fn ndc_to_world_space(target: Vector4, model_view_projection: Matrix4x4) -> Vector3 {
    let world_intermediate = inverse_mat4(model_view_projection) * target;
    let scalar = world_intermediate.w().recip();
    let world = world_intermediate * scalar;
    world.xyz()
}


pub fn world_to_view(target: Vector4, view: Matrix4x4) -> Vector3 {
    let view = view * target;
    view.xyz()
}


pub fn view_to_world(target: Vector4, view: Matrix4x4) -> Vector3 {
    let world = inverse_mat4(view) * target;
    world.xyz()
}

pub fn view_to_ndc(target: Vector4, projection: Matrix4x4) -> Vector3 {
    let dc = projection * target;
    let scalar = dc.w().recip();
    let ndc = dc * scalar;
    ndc.xyz()
}


pub fn view_to_world_point(
    view_point: Vector3,
    projection_view: Matrix4x4,
    viewport: ViewPort,
) -> Vector3 {
    let point_viewport_space = view_point.trunc_vec2() / Vector2::set(viewport.w, viewport.h);

    let point_viewport_norm = (point_viewport_space + point_viewport_space) - Vector2::ONE;
    let point_cam_space = point_viewport_norm * view_point.z();

    let plane_point = Vector4::set(
        point_cam_space.x(),
        point_cam_space.y(),
        view_point.z(),
        view_point.z(),
    );

    let inv_proj_view = inverse_mat4(projection_view);

    let world_point = inv_proj_view * plane_point;

    world_point.xyz()
}

pub fn world_to_view_point(
    world_point: Vector3,
    projection_view: Matrix4x4,
    viewport: ViewPort,
) -> Vector3 {
    let point = Vector4::set(world_point.x(), world_point.y(), world_point.z(), 1.0);
    let ndc_point = world_to_ndc_space(point, projection_view);

    let viewpoint = (ndc_point.trunc_vec2() + 1.0) * 0.5 * Vector2::set(viewport.w, viewport.h);

    Vector3::set(viewpoint.x(), viewpoint.y(), world_point.z())
}

pub fn view_to_viewport_point(view_point: Vector2, viewport: ViewPort) -> Vector2 {
    let dim_vec2 = Vector2::set(viewport.w, viewport.h);

    view_point / dim_vec2
}

pub fn viewport_to_view_point(viewport_point: Vector2, viewport: ViewPort) -> Vector2 {
    let dim_vec2 = Vector2::set(viewport.w, viewport.h);

    viewport_point * dim_vec2
}

pub fn world_to_viewport_point() {
    todo!()
}

pub fn viewport_to_world_point() {
    todo!()
}

use crate::mesh::{Mesh, Model, Vertex};

#[repr(C, align(16))]
#[derive(Debug)]
struct ConeData {
    pub position: [glam::Vec3A; 2],
    pub normal: [glam::Vec3A; 2],
    pub tangent: [glam::Vec4; 2],
    pub bi_tangent: [glam::Vec4; 2],
    pub uv: [glam::Vec2; 2],
}

#[allow(dead_code)]
const CONE_DATA: ConeData = ConeData {
    position: [
        glam::const_vec3a!([0.0, 1.0, 0.0]), // Apex position
        glam::const_vec3a!([0.0, 0.0, 0.0]), // Base-Center position
    ],
    normal: [
        glam::const_vec3a!([0.0, 1.0, 0.0]),  // Apex normal
        glam::const_vec3a!([0.0, -1.0, 0.0]), // Base-Center normal
    ],
    tangent: [
        glam::const_vec4!([-1.0, 0.0, 0.0, 1.0]), // Apex tangent
        glam::const_vec4!([-1.0, 0.0, 0.0, 1.0]), // Base-Center tangent
    ],
    bi_tangent: [
        glam::const_vec4!([0.0, 0.0, 1.0, 1.0]),  // Apex bi tangent
        glam::const_vec4!([0.0, 0.0, -1.0, 1.0]), // Base-Center bi tangent
    ],
    uv: [
        glam::const_vec2!([0.0, 1.0]),  // Apex uv
        glam::const_vec2!([0.0, -1.0]), // Base-Center uv
    ],
};

#[derive(Debug)]
pub struct Cone {
    model: Model,
}

impl Default for Cone {
    fn default() -> Self {
        Self::new(5.0, 1, 2., [0., 2., 0.])
    }
}

impl Cone {
    pub fn new(
        radius: f32,
        mut tessellation_slice: usize,
        height: f32,
        apex_position: [f32; 3],
    ) -> Cone {
        let mut indices = Vec::with_capacity(tessellation_slice * 6);
        let mut vertex_buffer: Vec<Vertex> = Vec::with_capacity(tessellation_slice + 2);

        //We can't technically have a cone with less than three tessellation slice for the base. right?
        tessellation_slice = tessellation_slice.max(3);

        let apex_position = glam::const_vec3a!(apex_position);

        //let base_to_apex_dir = apex_position.normalize(); // hard coded. Vec3A::Zero should be base position. (apex_position - glam::Vec3A::ZERO) //center instead of zero.
        let forward_dir = glam::Vec3A::X.cross(apex_position.normalize());
        let center = apex_position + (-apex_position.normalize() * height);

        // 360.0 / tessellation * PI_DIV_180(0.017_453_292) == 2.0 * PI /tessellation
        let angle_inc = 2.0 * std::f32::consts::PI / tessellation_slice as f32;

        // Apex Vertex
        vertex_buffer.push(Vertex {
            position: apex_position.to_array(),
            tex_coord: [0.0, 1.0],
            normal: [0.0, 1.0, 0.0],
            tangent: [-1.0, 0.0, 0.0, 1.0],
            bi_tangent: [0.0, 0.0, 1.0, 1.0],
        });

        // Base Center Vertex
        vertex_buffer.push(Vertex {
            position: center.to_array(),
            tex_coord: [0.0, -1.0],
            normal: [0.0, -1.0, 0.0],
            tangent: [-1.0, 0.0, 0.0, 1.0],
            bi_tangent: [0.0, 0.0, -1.0, 1.0],
        });

        // Cone vertex
        for side in 0..=tessellation_slice {
            let (rad_sin, rad_cos) = (angle_inc * side as f32).sin_cos();

            let vertex = center + (glam::Vec3A::X * rad_cos + forward_dir * rad_sin) * radius;
            let slant_height = glam::Vec3A::Y - vertex;

            let tangent = vertex.normalize().cross(glam::Vec3A::Y);
            let normal = slant_height.cross(tangent).normalize();
            let bi_tangent = normal.cross(tangent);

            vertex_buffer.push(Vertex {
                position: vertex.to_array(),
                tex_coord: [side as f32 / tessellation_slice as f32, 0.0],
                normal: normal.to_array(),
                tangent: tangent.extend(1.0).to_array(),
                bi_tangent: bi_tangent.extend(1.0).to_array(),
            });
        }

        //indices
        for point in 2..tessellation_slice + 2 {
            indices.push(0); // top
            indices.push(point + 1); // left
            indices.push(point); // right

            indices.push(1); // bottom
            indices.push(point); // right
            indices.push(point + 1); // left
        }

        let mesh = Mesh {
            vertices: vertex_buffer,
            material_id: 0,
            indices,
        };

        Cone {
            model: Model { meshes: vec![mesh] },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::mesh::primitive::cone::Cone;

    #[test]
    fn test() {
        Cone::new(5.0, 12, 2., [0.0, 1.0, 0.0]);
    }
}

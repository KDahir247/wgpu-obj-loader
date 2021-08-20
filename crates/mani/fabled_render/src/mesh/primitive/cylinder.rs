use crate::mesh::{Mesh, Model, Vertex};
//TODO Note don't use still Experimenting and it doesn't work yet.
/*
    Thought of Variables.
    Planning Phase
    Top radius
    bottom radius
    height
    radial segment
    rings
    Axis,
    Center,
    Radius,

    Sanity check for Top radius and bottom radius to prevent less than or equal to zero.
*/
pub struct Cylinder {
    /// The radius of the cylinder at y = height/2.
    pub top_radius: f32,

    /// The radius of the cylinder at y = -height/2.
    pub base_radius: f32,

    /// The height of the cylinder along the y-axis.
    pub height: f32,

    /// The number of slices of the base and top caps
    pub tessellation_sector: usize,

    /// the number of subdivision along y-axis
    pub tessellation_stack: usize,
}

impl Default for Cylinder {
    fn default() -> Self {
        Self::new(1.0, 1.0, 2.0, 36, 8)
    }
}

impl Cylinder {
    pub fn new(
        mut top_radius: f32,
        mut bottom_radius: f32,
        height: f32,
        mut tessellation_sector: usize,
        mut tessellation_stack: usize,
    ) -> Cylinder {
        /*

        Sanity check to ensure the top radius and bottom radius is never less than zero.
        Check and ensuring that tessellation stack and tessellation sector is never less then the
        threshold.

        */

        top_radius = top_radius.max(0.1);
        bottom_radius = bottom_radius.max(0.1);
        tessellation_sector = tessellation_sector.max(3);
        tessellation_stack = tessellation_stack.max(1);

        Self {
            top_radius,
            base_radius: bottom_radius,
            height,
            tessellation_sector,
            tessellation_stack,
        }
    }
}

/*


        let inv_slice = 1.0 / tessellation_sector as f32;
        let inv_stack = 1.0 / tessellation_stack as f32;

        let slice_step = std::f32::consts::TAU * inv_slice;
        let height_step = height * inv_stack;

        let radius_step = (top_radius - base_radius) * inv_stack;
        let current_height = -height * 0.5;

        let vertex_count = (tessellation_stack + 1) * tessellation_sector + 2;
        let indices_count = (((tessellation_stack + 1) * tessellation_sector) << 1) * 3;


*/
impl From<Cylinder> for Model {
    fn from(cylinder: Cylinder) -> Self {
        let Cylinder {
            top_radius,
            base_radius,
            height,
            tessellation_sector,
            tessellation_stack,
        } = cylinder;

        let mut vertex_buffer: Vec<Vertex> = Vec::new();
        let mut circle_vertex_buffer: Vec<f32> = Vec::new();
        let mut indices: Vec<usize> = Vec::new();

        let inv_sector = 1.0 / tessellation_sector as f32;
        let inv_stack = 1.0 / tessellation_stack as f32;

        // Generate a unit circle on the XY-plane
        let sector_step = std::f32::consts::TAU * inv_sector;

        for i in 0..=tessellation_sector {
            let sector_angle = i as f32 * sector_step;
            circle_vertex_buffer.push(sector_angle.cos());
            circle_vertex_buffer.push(sector_angle.sin());
            circle_vertex_buffer.push(0.0);
        }

        // Put side vertices to arrays
        for i in 0..2 {
            let i_f = i as f32;
            let h = -height * 0.5 + i_f * height;
            let t = 1.0 - i_f;

            for j in 0..=tessellation_sector {
                let ux = circle_vertex_buffer[j * 3];
                let uy = circle_vertex_buffer[j * 3 + 1];
                let uz = circle_vertex_buffer[j * 3 + 2];

                vertex_buffer.push(Vertex {
                    position: [ux, uy, h],
                    tex_coord: [j as f32 * inv_sector, t],
                    normal: [ux, uy, uz],
                    tangent: [0.0; 4],
                    bi_tangent: [0.0; 4],
                });
            }
        }

        let base_center_index = vertex_buffer.len() / 3;
        let top_center_index = base_center_index + tessellation_sector + 1;
        for i in 0..2 {
            let h = -height * 0.5 + i as f32 * -height;
            let nz = -1 + i * 2;

            vertex_buffer.push(Vertex {
                position: [0.0, 0.0, h],
                tex_coord: [0.5, 0.5],
                normal: [0.0, 0.0, nz as f32],
                tangent: [0.0; 4],
                bi_tangent: [0.0; 4],
            });

            for j in 0..tessellation_sector {
                let ux = circle_vertex_buffer[j * 3];
                let uy = circle_vertex_buffer[j * 3 + 1];

                vertex_buffer.push(Vertex {
                    position: [ux, uy, h],
                    tex_coord: [-ux * 0.5 + 0.5, -uy * 0.5 + 0.5],
                    normal: [0.0, 0.0, nz as f32],
                    tangent: [0.0; 4],
                    bi_tangent: [0.0; 4],
                });
            }
        }

        //Indices.
        let mut k1 = 0;
        let mut k2 = tessellation_sector + 1;

        for i in 0..tessellation_sector {
            indices.push(k1);
            indices.push(k1 + 1);
            indices.push(k2);

            indices.push(k2);
            indices.push(k1 + 1);
            indices.push(k2 + 1);

            k1 += 1;
            k2 += 1;
        }

        let mut k = base_center_index + 1;
        for i in 0..tessellation_sector {
            if i < tessellation_sector - 1 {
                indices.push(base_center_index);
                indices.push(k + 1);
                indices.push(k);
            } else {
                indices.push(base_center_index);
                indices.push(base_center_index + 1);
                indices.push(k);
            }

            k += 1;
        }

        let mut k = top_center_index + 1;
        for i in 0..tessellation_sector {
            if i < tessellation_sector - 1 {
                indices.push(top_center_index);
                indices.push(k);
                indices.push(k + 1);
            } else {
                indices.push(top_center_index);
                indices.push(k);
                indices.push(top_center_index + 1);
            }

            k += 1;
        }

        let mesh = Mesh {
            vertices: vertex_buffer,
            material_id: 0,
            indices,
        };

        Model { meshes: vec![mesh] }
    }
}

#[cfg(test)]
mod test {
    use crate::mesh::primitive::cylinder::Cylinder;
    use crate::mesh::Model;

    #[test]
    fn test() {
        let cylinder = Cylinder::default();
        let cylinder_model: Model = cylinder.into();

        for vertex in &cylinder_model.meshes[0].vertices {
            println!(
                "new Vector2({}f, {}f),",
                vertex.tex_coord[0], vertex.tex_coord[1]
            );
        }
        println!("{:?}", cylinder_model.meshes[0].indices);
    }
}

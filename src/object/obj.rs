use raylib::math::{Vector2, Vector3};
use tobj;

pub struct Obj {
    pub vertices: Vec<Vector3>,
    pub indices: Vec<u32>,
}

impl Obj {
    pub fn load(path: &str) -> Result<Self, tobj::LoadError> {
        let (models, _materials) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)?;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            for i in 0..num_vertices {
                let x = mesh.positions[i * 3];
                let y = mesh.positions[i * 3 + 1];
                let z = mesh.positions[i * 3 + 2];
                let position = Vector3::new(x, y, z);
                vertices.push(position);
            }
            indices.extend_from_slice(&mesh.indices);
        }

        Ok(Obj { vertices, indices })
    }

    pub fn get_vertex_array(&self) -> Vec<Vector3> {
        let mut vertex_array = Vec::new();
        for &index in &self.indices {
            vertex_array.push(self.vertices[index as usize].clone());
        }
        vertex_array
    }

    pub fn get_triangles(&self) -> Vec<(Vector3, Vector3, Vector3)> {
        let mut tris = Vec::new();
        // indices are assumed to be in multiples of 3 (triangles)
        for chunk in self.indices.chunks(3) {
            if chunk.len() < 3 {
                continue;
            }
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            if i0 < self.vertices.len() && i1 < self.vertices.len() && i2 < self.vertices.len() {
                let v0 = self.vertices[i0];
                let v1 = self.vertices[i1];
                let v2 = self.vertices[i2];
                tris.push((v0, v1, v2));
            }
        }
        tris
    }
}
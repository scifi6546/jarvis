use nalgebra::{Vector2, Vector3};
#[derive(Debug, PartialEq)]
enum VertexType {
    Position,
    Normal,
    UV,
}
impl VertexType {
    fn size(&self) -> usize {
        match self {
            Self::Position => 3 * std::mem::size_of::<f32>(),
            Self::Normal => 3 * std::mem::size_of::<f32>(),
            Self::UV => 2 * std::mem::size_of::<f32>(),
        }
    }
}
#[derive(Debug, PartialEq)]
struct VertexDesc {
    verticies: Vec<VertexType>,
}
impl VertexDesc {
    fn get_size(&self) -> usize {
        self.verticies.iter().map(|v| v.size()).sum()
    }
}
pub struct Mesh {
    data: Vec<f32>,
    vertex_description: VertexDesc,
}
impl Mesh {
    /// Makes new 3d model with items Position,Normal,UV in order (Position,Normal,UV)
    pub fn new(verticies: Vec<(Vector3<f32>, Vector3<f32>, Vector2<f32>)>) -> Self {
        let append_bytes = |data: &[f32], v: &mut Vec<f32>| {
            v.append(&mut data.to_vec());
        };
        let mut data = vec![];
        data.reserve(verticies.len() * (3 + 3 + 2));
        for (position, normal, uv) in verticies.iter() {
            append_bytes(position.as_slice(), &mut data);
            append_bytes(normal.as_slice(), &mut data);
            append_bytes(uv.as_slice(), &mut data);
        }
        let vertex_description = VertexDesc {
            verticies: vec![VertexType::Position, VertexType::Normal, VertexType::UV],
        };
        Self {
            data,
            vertex_description,
        }
    }
    /// Returns vertex as model in order (Position,Normal,UV)
    pub fn as_model(&self) -> Vec<(Vector3<f32>, Vector3<f32>, Vector2<f32>)> {
        // checking that the description is in the proper format
        assert_eq!(
            self.vertex_description,
            VertexDesc {
                verticies: vec![VertexType::Position, VertexType::Normal, VertexType::UV]
            }
        );
        // No better way was found
        let mut i = 0;
        let mut out = vec![];
        while i < self.data.len() {
            out.push((
                Vector3::new(self.data[i], self.data[i + 1], self.data[i + 2]),
                Vector3::new(self.data[i + 3], self.data[i + 4], self.data[i + 5]),
                Vector2::new(self.data[i + 6], self.data[i + 7]),
            ));
            i += 3 + 3 + 2;
        }
        return out;
    }
    pub fn get_data(&self) -> &Vec<f32> {
        &self.data
    }
    pub fn get_vertex_size(&self) -> usize {
        self.vertex_description.get_size()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_model() {
        let vertex = vec![(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector2::new(0.0, 0.0),
        )];
        let vertex_mesh = Mesh::new(vertex.clone());
        assert_eq!(vertex_mesh.as_model(), vertex);
        assert_eq!(
            vertex_mesh.get_vertex_size(),
            (3 + 3 + 2) * std::mem::size_of::<f32>()
        );
        assert_eq!(
            vertex_mesh.get_data(),
            &vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        );
    }
}

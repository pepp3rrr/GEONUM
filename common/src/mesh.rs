use crate::types::Point;
use blue_engine::{UnsignedIntType, Vector2, Vector3, Vertex};

type Mesh = (Vec<Vertex>, Vec<UnsignedIntType>);

pub trait IntoMesh {
    fn into_mesh(self) -> Mesh;
}

impl IntoMesh for Vec<Vec<Point<3>>> {
    fn into_mesh(self) -> Mesh {
        let mut i = 0;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for (x, row) in self.iter().enumerate() {
            for (y, b_x_y) in row.iter().enumerate() {
                if x + 1 < self.len() && y + 1 < row.len() {
                    let b_xp1_y = self[x + 1][y];
                    let b_x_yp1 = row[y + 1];
                    let b_xp1_yp1 = self[x + 1][y + 1];

                    vertices.push(Vertex {
                        position: Vector3::new(b_x_y.x(), b_x_y.z(), b_x_y.y()),
                        uv: Vector2::ZERO,
                        normal: Vector3::ZERO,
                    });
                    vertices.push(Vertex {
                        position: Vector3::new(b_x_yp1.x(), b_x_yp1.z(), b_x_yp1.y()),
                        uv: Vector2::ZERO,
                        normal: Vector3::ZERO,
                    });
                    vertices.push(Vertex {
                        position: Vector3::new(b_xp1_y.x(), b_xp1_y.z(), b_xp1_y.y()),
                        uv: Vector2::ZERO,
                        normal: Vector3::ZERO,
                    });
                    vertices.push(Vertex {
                        position: Vector3::new(b_xp1_yp1.x(), b_xp1_yp1.z(), b_xp1_yp1.y()),
                        uv: Vector2::ZERO,
                        normal: Vector3::ZERO,
                    });
                    indices.push(i);
                    indices.push(i + 1);
                    indices.push(i + 2);
                    indices.push(i + 2);
                    indices.push(i + 1);
                    indices.push(i + 3);
                    i += 4;
                }
            }
        }

        (vertices, indices)
    }
}

impl IntoMesh for Vec<Mesh> {
    fn into_mesh(self) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for mut mesh in self.into_iter() {
            let offset = vertices.len() as u16;
            vertices.append(&mut mesh.0);
            indices.append(&mut mesh.1.into_iter().map(|i| i + offset).collect());
        }

        (vertices, indices)
    }
}

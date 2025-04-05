use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
};

use blue_engine::{Vector2, Vector3, Vertex};
use geonum_common::{FromCSV, IntoMesh, Mesh, Point};

type Edge = (usize, usize);
type Face = (usize, usize, usize);

#[derive(Debug)]
pub struct TriangleMesh {
    vertices: Vec<Point<3>>,
    indices: Vec<Face>,
}

impl TriangleMesh {
    pub fn subdivide(&self) -> Self {
        let m = self.indices.len();
        let mut n = self.vertices.len();
        let mut new_vertices = Vec::new();
        let mut new_indices = Vec::with_capacity(m * 4);

        // Compute new midpoint vertices for each unique edge
        let mut visited_edges = HashMap::<Edge, usize>::new();
        for face in &self.indices {
            let a = (face.0, face.1);
            let b = (face.1, face.2);
            let c = (face.2, face.0);

            let new_points = [a, b, c].map(|edge| {
                if visited_edges.contains_key(&edge) {
                    return visited_edges.get(&edge).cloned().unwrap();
                }

                let edge_points = (self.vertices[edge.0], self.vertices[edge.1]);
                let adjacent_faces = self.get_adjacent_faces(&edge);
                assert_eq!(adjacent_faces.len(), 2);

                // Points of the adjacent faces not part of the current edge
                let extreme_indices = adjacent_faces
                    .into_iter()
                    .map(|face| {
                        [face.0, face.1, face.2]
                            .into_iter()
                            .find(|i| i != &edge.0 && i != &edge.1)
                            .unwrap()
                    })
                    .collect::<Vec<_>>();
                let extreme_points = (
                    self.vertices[extreme_indices[0]],
                    self.vertices[extreme_indices[1]],
                );

                let midpoint = (3.0 / 8.0 * (edge_points.0 + edge_points.1)
                    + 1.0 / 8.0 * (extreme_points.0 + extreme_points.1))
                    .into_point();
                let midpoint_index = n;

                new_vertices.push(midpoint);
                visited_edges.insert(edge, n);
                n += 1;

                midpoint_index
            });

            // Construct new triangles for the old face
            new_indices.push((new_points[0], new_points[2], face.0));
            new_indices.push((new_points[1], new_points[0], face.1));
            new_indices.push((new_points[2], new_points[1], face.2));
            new_indices.push((new_points[0], new_points[1], new_points[2]));
        }

        // Contains a list of each unique edge in our mesh
        let edges = visited_edges.into_keys().collect::<Vec<_>>();

        // Update old vertices positions and append new vertices
        let final_vertices = self
            .vertices
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, vertex)| {
                let neighbours = edges
                    .iter()
                    .filter_map(|edge| {
                        if edge.0 == index {
                            Some(edge.1)
                        } else if edge.1 == index {
                            Some(edge.0)
                        } else {
                            None
                        }
                    })
                    .filter(|x| x != &index)
                    .collect::<Vec<_>>();

                let n = neighbours.len() as f32;
                let beta = 1. / n * (5. / 8. - (3. / 8. + 1. / 4. * (2. * PI / n).cos()).powi(2));

                neighbours
                    .into_iter()
                    .fold(vertex.clone() * (1.0 - n * beta), |acc, x| {
                        acc + self.vertices[x] * beta
                    })
                    .into_point()
            })
            .chain(new_vertices)
            .collect();

        Self {
            vertices: final_vertices,
            indices: new_indices,
        }
    }

    fn get_adjacent_faces(&self, edge: &Edge) -> Vec<Face> {
        let mut adjacent = Vec::new();

        for face in &self.indices {
            if [face.0, face.1, face.2].contains(&edge.0)
                && [face.0, face.1, face.2].contains(&edge.1)
            {
                adjacent.push(face.clone());
            }
        }

        adjacent
    }
}

impl IntoMesh for TriangleMesh {
    fn into_mesh(self) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for vertex in self.vertices.into_iter() {
            vertices.push(Vertex {
                position: Vector3::new(vertex.x(), vertex.z(), vertex.y()),
                uv: Vector2::ZERO,
                normal: Vector3::ZERO,
            });
        }

        for index in self.indices {
            indices.push(index.0 as u16);
            indices.push(index.1 as u16);
            indices.push(index.2 as u16);
        }

        (vertices, indices)
    }
}

impl FromCSV for TriangleMesh {
    fn read(mut reader: geonum_common::CSVReader) -> Self {
        let mut iter = reader.records();
        let meta = iter.next().unwrap().unwrap();

        let num_vertices = *&meta[0].parse::<usize>().unwrap();
        let num_indices = *&meta[1].parse::<usize>().unwrap();

        let mut vertices = Vec::with_capacity(num_vertices);

        for _ in 0..num_vertices {
            let line = iter.next().unwrap().unwrap();

            let x: f32 = *&line[0].parse().unwrap();
            let y: f32 = *&line[1].parse().unwrap();
            let z: f32 = *&line[2].parse().unwrap();

            vertices.push(Point::<3>::new(x, y, z));
        }

        let mut indices = Vec::with_capacity(num_indices);

        for _ in 0..num_indices {
            let line = iter.next().unwrap().unwrap();

            let a: usize = *&line[1].parse().unwrap();
            let b: usize = *&line[2].parse().unwrap();
            let c: usize = *&line[3].parse().unwrap();

            indices.push((a, b, c));
        }

        Self { vertices, indices }
    }
}

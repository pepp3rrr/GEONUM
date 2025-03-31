use geonum_common::{FromCSV, Point};

#[derive(Debug)]
pub struct TriangleMesh {
    vertices: Vec<Point<3>>,
    indices: Vec<(usize, usize, usize)>,
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

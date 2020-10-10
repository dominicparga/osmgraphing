use nd_triangulation::Triangulation;

/// Test what happens, when same points are added multiple time, receiving different vertex-ids.
fn main() {
    check_identical_points();
}

fn check_identical_points() {
    println!("CHECK IDENTICAL POINTS");

    let dim = 3;
    let num_points: usize = 5;
    let points: Vec<f64> = (0..(num_points * dim)).map(|_| rand::random()).collect();

    println!(
        "Creating triangulation with {} points in dimension {}",
        num_points, dim
    );

    let mut triangulation = Triangulation::new(dim);

    for _ in 0..2 {
        points.chunks(dim).for_each(|p| {
            let id = triangulation
                .add_vertex(&p)
                .expect("Adding vertex to triangulation is expected to work.");
            println!("  Added point with vertex-id: {}", id);
        });

        println!(
            "Convex hull of triangulation consists of {} cells.",
            triangulation.convex_hull_cells().count()
        );
    }

    let mut ids: Vec<_> = triangulation
        .cells()
        .flat_map(|cell| {
            cell.vertices()
                .map(|vertex| vertex.id())
                .collect::<Vec<_>>()
        })
        .collect();
    ids.sort();
    ids.dedup();
    println!(
        "There are {} different ids ({:?}) remaining in the triangulation.",
        ids.len(),
        ids
    );
}

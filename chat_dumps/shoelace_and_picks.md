Context:
You are working on project that uses Installed crates: clap 4.4.8, itertools 0.12.0, rayon 1.8.0, pretty_assertions 1.4.0.
Prefer Rust 1.74.1 language if the used language and toolset are not defined below or in the user messages.
===========================================================================================================================

User: 
```rust
pub struct Point {
    x: usize,
    y: usize,
}
```

I have a vector of points
. write a function that applies the shoelace formula to calculate the area of these points
Before answering it did these function calls:


Assistant: 
The shoelace formula is a simple and efficient method for determining the area of a polygon given the coordinates of its vertices.

Here is how you can apply the shoelace formula in Rust to a `Vec<Point>` where each `Point` represents a vertex of the polygon. This implementation assumes that the points are given in counter-clockwise order and the polygon is simple (does not intersect itself).

```rust
pub struct Point {
    x: usize,
    y: usize,
}

pub fn polygon_area(points: &[Point]) -> f64 {
    // Add the first point to the end of the points vector to create a closed loop
    let mut points = points.to_vec();
    points.push(points[0].clone());

    let mut sum = 0.0;
    for i in 0..points.len() - 1 {
        let (x1, y1) = (points[i].x as f64, points[i].y as f64);
        let (x2, y2) = (points[i + 1].x as f64, points[i + 1].y as f64);

        sum += x1*y2 - x2*y1;
    }
    (sum / 2.0).abs()
}
```

This function calculates the area based on the given points. It adds the first point to the end to close the polygon. Then, it uses two adjacent points at a time to calculate the part of the formula `x1*y2 - x2*y1` and accumulates it in `sum`. Finally, it divides the absolute value of the sum by 2 (`.abs() / 2.0`) to get the area of the polygon. Note that we convert x and y to f64 to allow floating point division.
Before answering it did these function calls:


User: 
rewrite pick's theorem so that the area is an input, and the output is the number of interior coordiantes
Before answering it did these function calls:


Assistant: 
Pick's theorem provides a simple formula to calculate the area of a lattice polygon: `Area = I + B/2 - 1`, where `I` is the number of lattice points in the interior of the polygon, and `B` is the number of lattice points on the boundary of the polygon.

If we want to rewrite Pick's theorem so that the area A and the number of boundary points B are inputs, and the output is the number of interior points I, we would have: `I = A - B/2 + 1`.

Here is that function in Rust:

```rust
pub fn interior_points(area: f64, boundary_points: usize) -> f64 {
    area - (boundary_points as f64)/2.0 + 1.0
}
```

`area` parameter is the area of a polygon and `boundary_points` is the number of points on the boundary of the polygon. 

Remember that this function, as well as Pick's theorem, is only correct when used with simple (i.e., non-self-intersecting) polygons whose vertices are grid (or lattice) points.
Before answering it did these function calls:



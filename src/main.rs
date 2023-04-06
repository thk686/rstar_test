use geo::{Translate, Polygon, polygon, Point};
use num_traits::Float;
use rstar::RTree;

fn make_hexagon() -> Polygon {
    const S32: f64 = 0.8660254038;
    polygon![
        (x: 0.0,  y: 1.0),
        (x: S32,  y: 0.5),
        (x: S32,  y: -0.5),
        (x: 0.0,  y: -1.0),
        (x: -S32, y: -0.5),
        (x: -S32, y: 0.5),
        (x: 0.0,  y: 1.0)
    ]
}

fn l2norm<T: Float>(ord1: T, ord2: T) -> T {
    ((ord1 * ord1) + (ord2 * ord2)).sqrt()
}

fn make_hexagon_array(radius: f64) -> Vec<Polygon> {
    const S32: f64 = 0.8660254038;
    let mut res = Vec::new();
    let mut x = -radius;
    let mut y = -radius;
    let mut x_adj = 0.0;
    while x <= radius {
        while y <= radius {
            let xx = x + x_adj;
            if l2norm(xx, y) <= radius {
                let h = make_hexagon().translate(xx, y);
                res.push(h);
            }
            y += 1.5;
            x_adj = if x_adj == 0.0 { S32 } else { 0.0 }
        }
        y = -radius;
        x += 2.0 * S32;
        x_adj = 0.0;
    }
    res
}

fn main() {
    let x = make_hexagon_array(20.0);
    let index = RTree::bulk_load(x);
    let found = index.locate_at_point(&Point::new(0.0, 0.0)).unwrap();

    println!("{:?}", found);   
}

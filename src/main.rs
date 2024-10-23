use geo::{BooleanOps, MultiPolygon};

fn main() {
    let base = include_str!("../base.json");
    let union = include_str!("../union_this.json");

    let base: MultiPolygon = serde_json::from_str(&base).unwrap();
    let union: MultiPolygon = serde_json::from_str(&union).unwrap();

    base.union(&union);

    println!("Hello, world!");
}

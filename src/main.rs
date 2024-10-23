use std::io::Write;
use std::str::FromStr;
use geo::{BooleanOps, MultiPolygon};

fn main() {
    let base = include_str!("../base.geojson");
    let union = include_str!("../union.geojson");

    let base: MultiPolygon = geojson::Geometry::from_str(base).expect("valid GeoJSON").try_into().expect("valid MultiPolygon");
    let union: MultiPolygon = geojson::Geometry::from_str(union).expect("valid GeoJSON").try_into().expect("valid MultiPolygon");

    base.union(&union);

    println!("Hello, world!");
}

fn write_geojson_to_file(geometry: &MultiPolygon, path: &str) {
    let converted_to_geojson = geojson::Value::from(geometry);
    let file = std::fs::File::create(path).unwrap();
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(&converted_to_geojson.to_string().into_bytes()).unwrap();
}

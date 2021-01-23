use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ComplexPoint {
    x: i32,
    y: i32,
    z: i32,
    name: String,
    vector: Vec<i32>,
    p: Point,
}


fn main() {
    let point = Point { x: 1, y: 2 };
    let cp = ComplexPoint {
        x: 1,
        y: 3,
        z: 4,
        name: "Rudy".to_string(),
        vector: vec![5, 6, 8],
        p: point,
    };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&cp).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: ComplexPoint = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}
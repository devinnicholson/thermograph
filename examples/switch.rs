//! Runnable approximation check for the switch `{ 1 | -1 }`.

use thermograph::CGTValue;

fn main() {
    let game = CGTValue::GameTree {
        left: vec![CGTValue::Integer(1)],
        right: vec![CGTValue::Integer(-1)],
    };

    let approximation = game.approximate_thermograph();
    let tolerance = 1e-6_f32;

    assert!((approximation.temperature - 1.0).abs() <= tolerance);
    assert!(approximation.mean.abs() <= tolerance);

    println!(
        "approximate temperature={:.6} mean={:.6} tolerance={tolerance:.6}",
        approximation.temperature, approximation.mean
    );
}

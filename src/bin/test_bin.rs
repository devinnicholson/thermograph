use thermograph::CGTValue;

fn main() {
    let gl = CGTValue::GameTree {
        left: vec![CGTValue::Integer(5)],
        right: vec![CGTValue::Integer(1)],
    };
    let gr = CGTValue::GameTree {
        left: vec![CGTValue::Integer(-1)],
        right: vec![CGTValue::Integer(-5)],
    };
    let g = CGTValue::GameTree {
        left: vec![gl],
        right: vec![gr],
    };
    println!("G: {:?}", g.thermograph());

    let gl2 = CGTValue::GameTree {
        left: vec![CGTValue::Integer(5)],
        right: vec![CGTValue::Integer(1)],
    };
    let gr2 = CGTValue::GameTree {
        left: vec![CGTValue::Integer(1)],
        right: vec![CGTValue::Integer(-3)],
    };
    let g2 = CGTValue::GameTree {
        left: vec![gl2],
        right: vec![gr2],
    };
    println!("G2: {:?}", g2.thermograph());
}

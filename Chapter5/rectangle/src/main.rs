fn main() {
    let l = 5;
    let w = 4;
    println!("Area is {}", area(l,w));

    let rec = (l,w);
    println!("Area is {}", area_tuples(rec));

    let scale = 2;
    let rec2 = Rectangle {
        width: dbg!(scale * 5),
        length: 5,
    };

    dbg!(&rec2);

    println!("Area is {}", area_structs(&rec2));
    
    //Println! macro can't print structs, without setting debug mode
    println!("Rectangle is {rec2:?}");
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

//refactoring with tuples
fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

//refactoring with structs
fn area_structs(r: &Rectangle) -> u32 {
    r.width * r.length
}

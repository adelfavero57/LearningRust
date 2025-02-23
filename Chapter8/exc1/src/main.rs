use std::collections::HashMap;

fn stats(v: Vec<i32>) {
    let median = &v[v.len()/2];

    let mut h = HashMap::new();

    for n in &v {
        let counter = h.entry(n).or_insert(0);
        *counter +=1;
    }

    let mut maximum = 0;
    let mut mode = 0;
    for (x,y) in &h {
        if y > &maximum {
            mode = **x;
            maximum = *y;
        }
    }

    println!("median is {median}, mode is {mode}")

}

fn main() {
    let list = [1,5,2,1,7,3,23,3,2,33,3,4,4,4,4,4,4,4,4,4];

    let mut v: Vec<i32> = Vec::new();
    for i in list {
        v.push(i);
    }

    stats(v)
   
}

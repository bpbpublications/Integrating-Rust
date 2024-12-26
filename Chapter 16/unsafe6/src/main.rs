// Declaration of a union
#[allow(dead_code)]
union MyUnion {
    i: i32,
    f: f32,
}

#[allow(dead_code)]
fn main() {
    let my_union = MyUnion { i: 10 };

    unsafe {
        // Accessing a field of the union
        println!("Union field value: {}", my_union.i);
    }
}

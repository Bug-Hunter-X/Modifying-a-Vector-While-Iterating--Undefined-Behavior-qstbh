fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Create a copy of the vector to avoid mutation during iteration
    let vec_copy = vec.clone();

    for i in vec_copy.iter(){
        println!("Element: {:?}", i);
    }

    //Modify the original vector after iteration
    vec.push(4);
    println!("Modified Vector: {:?}", vec);
} 
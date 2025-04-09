fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a: [_; 101] = std::array::from_fn(|i| char::from_u32(65 + i as u32).unwrap_or_default());

    for char in a {
        println!("{char}")
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}

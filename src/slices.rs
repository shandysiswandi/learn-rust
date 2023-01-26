pub fn run() {
    println!("===== Slice =====");
    // Read only atau shared reference, operator yang digunakan adalah `&`.
    // Mutable reference, operator yang digunakan adalah `&mut`.

    // read only
    let array_number = [12, 16, 8, 3];
    let slice_a = &array_number[0..3]; // get data from array_number from index 0 to index before 3 (exclude)
    let slice_b = &slice_a[1..=2]; // get data from array_number from index 1 to index 2 (include)

    println!(
        "slice_a: {:?}, size: {} | slice_b: {:?}, size: {}",
        slice_a,
        slice_a.len(),
        slice_b,
        slice_b.len()
    );

    // Mutable reference
    let mut arr_num = [12, 16, 8, 3];
    println!("arr_num : {:?}", arr_num);

    let slice_c = &mut arr_num[..=2];
    println!("Before => slice_c: {:?}", slice_c);

    slice_c[1] = 99;
    println!("After  => slice_c: {:?}", slice_c);

    // looping
    for score in &mut slice_c[..] {
        *score += 1;
    }
    println!("After  => slice_c: {:?}", slice_c);

    println!();
}

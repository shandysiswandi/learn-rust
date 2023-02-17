#![allow(clippy::all)]

pub fn run() {
  println!("===== Vector =====");

  // how to write vector
  let mut data_one = vec!["batman", "superman", "lobo"];
  println!(
    "data: {:?} length: {}, capacity: {}",
    data_one,
    data_one.len(),
    data_one.capacity()
  );

  // delete last element of vector
  data_one.pop();
  println!(
    "data: {:?} length: {}, capacity: {}",
    data_one,
    data_one.len(),
    data_one.capacity()
  );

  // delete element in vector using index
  data_one.remove(1);
  println!(
    "data: {:?} length: {}, capacity: {}",
    data_one,
    data_one.len(),
    data_one.capacity()
  );

  // add an element to the back of vector
  data_one.push("constantine");
  data_one.push("trigon");
  data_one.push("darkseid");
  println!(
    "data: {:?} length: {}, capacity: {}",
    data_one,
    data_one.len(),
    data_one.capacity()
  );

  // modify value element of vector using index
  data_one[2] = "red hood";
  println!(
    "data: {:?} length: {}, capacity: {}",
    data_one,
    data_one.len(),
    data_one.capacity()
  );

  // to sort vector ascending
  data_one.sort();
  println!(
    "data: {:?} length: {}, capacity: {}",
    data_one,
    data_one.len(),
    data_one.capacity()
  );

  // to check if vector is empty
  println!("is data_one empty: {:?}", data_one.is_empty());

  // removing all elements of vector
  data_one.clear();
  println!(
    "data: {:?} length: {}, capacity: {}",
    data_one,
    data_one.len(),
    data_one.capacity()
  );

  // used to combine two vectors
  let mut data_two = vec!["ant", "pig", "bird"];
  data_two.append(&mut vec!["horse", "fish", "duck"]);
  println!(
    "data: {:?} length: {}, capacity: {}",
    data_two,
    data_two.len(),
    data_two.capacity()
  );

  // vector does not have default value, we have to explicit tell the type
  // let vector_empty = vec![];                   // bad
  // let vector_new = Vec::new();                 // bad
  // let vector_explicit: Vec<i8> = Vec::new();   // good

  // iteration of vector
  let vec_loop = vec![1, 2, 3];
  print!("Iteration: ");
  for e in vec_loop {
    print!("{e} ");
  }
  print!("\n");

  // ownership vector
  // when the for in keyword is used to iterate over vector `vec_owner`, it makes the owner of the vector data move to variable `e`
  // The effect of this is that when we try to access the same variable after the loop is finished, an error appears, because the value has changed.
  // bad example:
  let vec_owner = vec![1, 2, 3];
  // for e in vec_owner {
  //     print!("{e} ");
  // }
  // for e in vec_owner {
  //     print!("{e} ");
  // }
  // good example
  // using borowwing
  print!("Iteration borowwing: ");
  for e in &vec_owner {
    print!("{e} ");
  }
  print!("\n");
  // using iter trait
  print!("Iteration using iter trait: ");
  for e in vec_owner.iter() {
    print!("{e} ");
  }
  print!("\n");

  // advance vector we can use VecDeque<T>
  // use std::collections::VecDeque;
  // let mut vec_10 = VecDeque::from(vec!["a", "b", "c"]);
  // vec_10.pop_front();
  // vec_10.push_front("z");
  // println!("data: {:?}", vec_10);
  // vec_10.pop_back();
  // vec_10.push_back("h");
  // println!("data: {:?}", vec_10);

  println!();
}

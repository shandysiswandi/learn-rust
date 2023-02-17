pub fn run() {
  println!("===== Tuple =====");
  // maximum element in tuple is 12

  // how to write tuple
  let mut tuple_a = ("jason", 27, ["racing", "working out"], true);
  println!("tuple_a: {:?}", tuple_a);

  // how to read specific tuple
  println!("index 0: {:?}", tuple_a.0);
  println!("index 1: {:?}", tuple_a.1);
  println!("index 2: {:?} {:?}", tuple_a.2[0], tuple_a.2[1]);
  println!("index 3: {:?}", tuple_a.3);

  // how to modify tuple
  tuple_a.0 = "damian";
  tuple_a.1 = 18;
  tuple_a.2 = ["gaming", "adventuring"];
  tuple_a.3 = true;
  println!("tuple_a: {:?}", tuple_a);

  // Packing tuple
  let name = "grayson";
  let age = 29;
  let tuple_b = (name, age);
  println!("tuple_b: {:?}", tuple_b);

  // Unpacking tuple
  let (name, age) = tuple_b;
  println!("name: {name} | age: {age}");

  println!();
}

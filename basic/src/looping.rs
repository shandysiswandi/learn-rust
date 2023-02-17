pub fn run() {
  println!("===== Looping =====");

  // infinite loop
  // loop {
  //     println!("hello rust")
  // }

  // break: escaping loops
  // continue: skip 1 iteration loop
  let mut i_num = 0;
  let maxi = 15;
  loop {
    i_num += 1;
    if i_num % 2 == 1 {
      continue;
    }

    println!("nilai i: {i_num}");
    if i_num > maxi {
      break;
    }
  }

  // while loop
  let mut number = 0;
  while number <= 5 {
    println!("the number is change {}", number);
    number += 1;
  }

  // nested while loop
  let mut i = 0;
  let max = 5;
  while i < max {
    let mut j = 0;
    let max_inner = i;

    while j <= max_inner {
      print!("* ");
      j += 1;
    }

    println!();
    i += 1;
  }

  // label in loop
  let mut iter = 0;
  let maximal = 9;

  'mainLoop: loop {
    iter += 1;
    let mut je = 0;

    loop {
      if iter > maximal {
        break 'mainLoop;
      }

      je += 1;
      if je > iter {
        break;
      }

      print!("{iter} ");
    }

    println!();
  }

  // return from loop
  let mut counter = 0;
  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("result: {result}");

  // for in loop iteration
  // format a..n ex 1..5 => 1,2,3,4
  for i in 1..5 {
    println!("nimber of for in is {i}");
  }

  // format a..=n ex 1..=5 => 1,2,3,4,5
  for i in 1..=5 {
    println!("nimber of for in is {i}");
  }

  let array = ["jason", "grayon", "drake", "damian"];
  for name in array {
    println!("for from array is {name}");
  }

  println!();
}

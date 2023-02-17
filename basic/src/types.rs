pub fn run() {
  println!("===== Scalar Types =====");

  // Numeric Signed
  // when using declaration type inference, the default numeric the data type is i32
  let nsi8: i8 = 127;
  let nsi16: i16 = 32_767;
  let nsi32: i32 = 2_147_483_647;
  let nsi64: i64 = 9_223_372_036_854_775_807;
  let nsi128: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
  println!(
    "Numeric Signed Types: i8={} | i16={} | i32={} | i64={} | i128={}",
    nsi8, nsi16, nsi32, nsi64, nsi128
  );

  // Numeric Unsigned
  let nusi8 = u8::MAX;
  let nusi16 = u16::MAX;
  let nusi32 = u32::MAX;
  let nusi64 = u64::MAX;
  let nusi128 = u128::MAX;
  println!(
    "Numeric Unsigned Types: u8={} | u16={} | u32={} | u64={} | u128={}",
    nusi8, nusi16, nusi32, nusi64, nusi128
  );

  // Floating Point
  // when using declaration type inference, the default float the data type is f64
  // format {.n} to print decimal digits n. ex: 14.12345 {.2} then 14.12
  let fp32: f32 = 3.14;
  let fp64 = 3.1487655;
  println!("Floating Point Types: f32={} | f64={:.4}", fp32, fp64);

  // Booelan
  let (truely, falsy) = (true, false);
  println!("Boolean Types: true={} | false={}", truely, falsy);

  // Character
  let icon = '☀';
  println!("Character Types: icon={}", icon);

  println!();
}

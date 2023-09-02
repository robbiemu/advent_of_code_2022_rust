use std::{cmp::Ordering, env, fmt, fs, iter::Sum, ops::Add, rc::Rc};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BalancedQuinaryDigit {
  Zero,
  One,
  Two,
  MinusOne,
  MinusTwo,
}

impl BalancedQuinaryDigit {
  fn add_with_carry(
    digit1: &BalancedQuinaryDigit,
    digit2: &BalancedQuinaryDigit,
    carry: BalancedQuinaryDigit,
  ) -> (BalancedQuinaryDigit, BalancedQuinaryDigit) {
    let sum_value = digit1.to_value() + digit2.to_value() + carry.to_value();

    match sum_value {
      -5 => (BalancedQuinaryDigit::Zero, BalancedQuinaryDigit::MinusOne),
      -4 => (BalancedQuinaryDigit::One, BalancedQuinaryDigit::MinusOne),
      -3 => (BalancedQuinaryDigit::Two, BalancedQuinaryDigit::MinusOne),
      -2 => (BalancedQuinaryDigit::MinusTwo, BalancedQuinaryDigit::Zero),
      -1 => (BalancedQuinaryDigit::MinusOne, BalancedQuinaryDigit::Zero),
      0 => (BalancedQuinaryDigit::Zero, BalancedQuinaryDigit::Zero),
      1 => (BalancedQuinaryDigit::One, BalancedQuinaryDigit::Zero),
      2 => (BalancedQuinaryDigit::Two, BalancedQuinaryDigit::Zero),
      3 => (BalancedQuinaryDigit::MinusTwo, BalancedQuinaryDigit::One),
      4 => (BalancedQuinaryDigit::MinusOne, BalancedQuinaryDigit::One),
      5 => (BalancedQuinaryDigit::Zero, BalancedQuinaryDigit::One),
      _ => unreachable!(),
    }
  }
  fn to_value(self) -> i128 {
    match self {
      BalancedQuinaryDigit::Zero => 0,
      BalancedQuinaryDigit::One => 1,
      BalancedQuinaryDigit::Two => 2,
      BalancedQuinaryDigit::MinusOne => -1,
      BalancedQuinaryDigit::MinusTwo => -2,
    }
  }
}

impl PartialOrd for BalancedQuinaryDigit {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.to_value().cmp(&other.to_value()))
  }
}

impl Ord for BalancedQuinaryDigit {
  fn cmp(&self, other: &Self) -> Ordering {
    self.to_value().cmp(&other.to_value())
  }

  fn max(self, other: Self) -> Self
  where
    Self: Sized,
  {
    std::cmp::max_by(self, other, Ord::cmp)
  }

  fn min(self, other: Self) -> Self
  where
    Self: Sized,
  {
    std::cmp::min_by(self, other, Ord::cmp)
  }

  fn clamp(self, min: Self, max: Self) -> Self
  where
    Self: Sized,
    Self: PartialOrd,
  {
    assert!(min <= max);
    if self < min {
      min
    } else if self > max {
      max
    } else {
      self
    }
  }
}

impl fmt::Display for BalancedQuinaryDigit {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let display_str = match self {
      BalancedQuinaryDigit::Zero => "0",
      BalancedQuinaryDigit::One => "1",
      BalancedQuinaryDigit::Two => "2",
      BalancedQuinaryDigit::MinusOne => "-",
      BalancedQuinaryDigit::MinusTwo => "=",
    };
    write!(f, "{}", display_str)
  }
}

#[derive(Clone, Debug, PartialEq)]
struct BalancedQuinaryNumber {
  digits: Rc<Vec<BalancedQuinaryDigit>>,
}

impl BalancedQuinaryNumber {
  fn new() -> Self {
    BalancedQuinaryNumber { digits: Rc::new(vec![BalancedQuinaryDigit::Zero]) }
  }

  fn pad_with_zeros(&mut self, length: usize) {
    let digits = Rc::make_mut(&mut self.digits);
    while digits.len() < length {
      digits.insert(0, BalancedQuinaryDigit::Zero);
    }
  }

  fn from_str(input: &str) -> Option<Self> {
    let mut digits = Vec::new();

    for c in input.chars() {
      let digit = match c {
        '0' => BalancedQuinaryDigit::Zero,
        '1' => BalancedQuinaryDigit::One,
        '2' => BalancedQuinaryDigit::Two,
        '-' => BalancedQuinaryDigit::MinusOne,
        '=' => BalancedQuinaryDigit::MinusTwo,
        _ => return None, // Invalid character
      };
      digits.push(digit);
    }

    Some(BalancedQuinaryNumber { digits: Rc::new(digits) })
  }
}

impl PartialOrd for BalancedQuinaryNumber {
  fn partial_cmp(&self, other: &BalancedQuinaryNumber) -> Option<Ordering> {
    match self.digits.len().cmp(&other.digits.len()) {
      Ordering::Less => Some(Ordering::Less),
      Ordering::Greater => Some(Ordering::Greater),
      Ordering::Equal => {
        for (digit1, digit2) in
          self.digits.iter().rev().zip(other.digits.iter().rev())
        {
          match digit1.cmp(digit2) {
            Ordering::Less => return Some(Ordering::Less),
            Ordering::Greater => return Some(Ordering::Greater),
            Ordering::Equal => continue,
          }
        }

        Some(Ordering::Equal)
      }
    }
  }
}

impl Add for BalancedQuinaryNumber {
  type Output = BalancedQuinaryNumber;

  fn add(self, other: BalancedQuinaryNumber) -> BalancedQuinaryNumber {
    // Create mutable vectors for the digits of both numbers.
    let mut result_digits = Vec::new();
    let mut carry = BalancedQuinaryDigit::Zero;

    // Ensure both numbers have the same number of digits by padding the shorter number.
    let max_len = self.digits.len().max(other.digits.len());
    let mut left = self.clone();
    let mut right = other.clone();

    if left > right {
      right.pad_with_zeros(max_len);
    } else {
      left.pad_with_zeros(max_len);
    }

    for i in 0..max_len {
      let digit1 = left.digits[max_len - i - 1];
      let digit2 = right.digits[max_len - i - 1];

      let (column_value, remaining) =
        BalancedQuinaryDigit::add_with_carry(&digit1, &digit2, carry);
      carry = remaining;

      // Push the new digit to the result.
      result_digits.insert(0, column_value);
    }

    // If there's a carry left, add it as the most significant digit.
    if carry != BalancedQuinaryDigit::Zero {
      result_digits.push(carry);
    }

    BalancedQuinaryNumber { digits: Rc::new(result_digits) }
  }
}

impl Sum<Self> for BalancedQuinaryNumber {
  fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
    iter.fold(BalancedQuinaryNumber::new(), |acc, x| acc + x)
  }
}

impl From<BalancedQuinaryNumber> for i128 {
  fn from(val: BalancedQuinaryNumber) -> Self {
    let mut result = 0;
    let mut factor = 1;

    for digit in val.digits.iter().rev() {
      result += factor * digit.to_value();
      factor *= 5;
    }

    result
  }
}

impl fmt::Display for BalancedQuinaryNumber {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for digit in self.digits.iter() {
      write!(f, "{}", digit)?;
    }
    Ok(())
  }
}

fn main() -> Result<(), String> {
  let src = extract()?;
  let results = transform(src)?;

  readout(&results)
}

fn extract() -> Result<String, String> {
  let filename = env::args().nth(1).ok_or("Expected file argument")?;

  fs::read_to_string(filename).map_err(|_| "Failed to read file".to_string())
}

fn transform(src: String) -> Result<Vec<BalancedQuinaryNumber>, String> {
  let mut output: Vec<BalancedQuinaryNumber> = Vec::new();
  for line in src.split('\n') {
    output.push(
      BalancedQuinaryNumber::from_str(line).ok_or("cannot parse number")?,
    );
  }

  Ok(output)
}

fn readout(results: &[BalancedQuinaryNumber]) -> Result<(), String> {
  results.iter().for_each(|result| {
    let value: i128 = result.to_owned().into();
    println!("{}", value);
  });
  println!(
    "total: {}",
    results.iter().cloned().sum::<BalancedQuinaryNumber>()
  );

  Ok(())
}

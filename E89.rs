use std::io::File;
use std::cmp::max;

fn to_decimal(ch: char) -> int {
  match ch {
    'M' => 1000,
    'D' => 500,
    'C' => 100,
    'L' => 50,
    'X' => 10,
    'V' => 5,
    'I' => 1,
    _ => fail!("Invalid roman numeral")
  }
}

fn decode(buf: &str) -> int {
  let mut value = 0;
  let mut accum = 0;
  let mut last = 0;

  for c in buf.chars() {
    let cur = to_decimal(c);
    if cur < last {
      value += accum;
      accum = cur;
    } else if cur == last {
      accum += cur;
    } else {
      value -= accum;
      accum = cur;
    }

    last = cur;
  }

  value + accum
}

fn from_digit(dig: int, subs: &str, max: &str, min: &str) -> ~str {
  let mut buf = ~"";

  if dig < 4 {
    for _ in range(0, dig) {
      buf = buf + subs.to_owned()
    }
  } else if dig == 4 {
    return subs.to_owned() + min.to_owned();
  } else if dig <= 8 {
    buf = min.to_owned();
    for _ in range(0, dig - 5) {
      buf = buf + subs.to_owned()
    }
  } else {
    for _ in range(dig, 10) {
      buf = buf + subs.to_owned()
    }
    buf = buf + max.to_owned()
  }

  buf
}

fn encode(num: int) -> ~str {
  let mut buf = ~"";
  let mut n = num;

  while n >= 1000 {
    n -= 1000;
    buf = buf + "M";
  }

  buf + from_digit(n / 100 % 10,     "C", "M", "D")
      + from_digit(n / 10 % 10, "X", "C", "L")
      + from_digit(n % 10,      "I", "X", "V")
}

fn main() {
  let input = File::open(&Path::new("roman.txt")).read_to_str();
  let mut saved = 0;

  match input {
    Err(_) => fail!("Cannot read file"),
    Ok(content) => {
      for line in content.lines() {
        let num = decode(line);
        saved += line.char_len() - encode(num).char_len();
      }
    }
  }

  println!("{}", saved);
}

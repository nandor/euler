
fn digits(n: int, v: &mut [int]) {
  let mut nn = n;
  while nn > 0 {
    v[nn % 10] += 1;
    nn /= 10;
  }
}

fn is_pandigital(i: int, j: int) -> bool {
  let mut a = [0, ..10];

  digits(i, a);
  digits(j, a);
  digits(i / j, a);

  if a[0] != 0 {
    return false;
  }

  for i in range(1, 10) {
    if a[i] != 1 {
      return false;
    }
  }

  true
}

fn main() {
  let mut sum = 0;
  for i in range(1, 1000000) {
    let mut j = 2;
    while j * j <= i {
      if i % j == 0 {
        if is_pandigital(i, j) {
          sum += i;
          break;
        }
      }

      j += 1;
    }
  }

  println!("{}", sum);
}
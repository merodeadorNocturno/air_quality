pub fn get_primes() -> u64 {
  const N: u64 = 2000000;
  // const N: u64 = 10;

  let mut p: u64 = 2;

  let mut list: Vec<u64> = Vec::new();
  let mut primes: Vec<u64> = Vec::new();
  // let mut done: bool = false;

  primes.push(2);

  while p < N {
    let mut i: u64 = 1;

    while p * i <= N {
      // println!("One: {}, {}, {}", &p, &i, &p * i);
      list.push(p * i);
      i += 1;
    }

    while p < N {
      // println!("Two: {}, {}, {}", &p, &i, &p * i);
      p += 1;
      if !list.contains(&p) {
        primes.push(p);
        break;
      }
    }
  }

  primes[10000]
}

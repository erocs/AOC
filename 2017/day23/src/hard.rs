//          set b 93
//          set c b
//          jnz a 2 prod
//          jnz 1 5 start
//    prod: mul b 100
//          sub b -100000
//          set c b
//          sub c -17000
//   start: set f 1
//          set d 2
//   inner: set e 2
//   tight: set g d
//          mul g e
//          sub g b
//          jnz g 2 notfound
//          set f 0
//notfound: sub e -1
//          set g e
//          sub g b
//          jnz g -8 tight
//          sub d -1
//          set g d
//          sub g b
//          jnz g -13 inner
//          jnz f 2 nosub
//          sub h -1
//   nosub: set g b
//          sub g c
//          jnz g 2 notdone
//          jnz 1 3 halt
// notdone: sub b -17
//          jnz 1 -23 start
//    halt:

fn solve() -> String {
  let mut counter = 0;
  let seed: i64 = 93;
  let mut b = seed * 100 + 100_000;
  let c = b + 17_000;
  while b <= c {
    let mut found = false;
    {
      let mut d = 2;
      while d <= (b / 2) && !found {
        if b % d == 0 {
          found = true;
        }
        d += 1;
      }
    }
    if found {
      counter += 1;
    }
    b += 17;
  }
  format!("{}", counter)
}

// 909 is someone else's answer.

fn main() {
  println!("Result: {}", solve());
}

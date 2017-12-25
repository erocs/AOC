use std::collections::LinkedList;

fn do_left(left: &mut LinkedList<i64>, right: &mut LinkedList<i64>, val: i64) -> i64 {
  right.push_front(val);
  left.pop_back().unwrap_or(0)
}

fn do_right(left: &mut LinkedList<i64>, right: &mut LinkedList<i64>, val: i64) -> i64 {
  left.push_back(val);
  right.pop_front().unwrap_or(0)
}

fn solve() {
  let steps = 12919244;
  let mut state = 'A';
  let mut left = LinkedList::new();
  let mut right = LinkedList::new();
  let mut cur = 0;
  for _ in 0..steps {
    match state {
      'A' => {
         if cur == 0 {
           cur = do_right(&mut left, &mut right, 1);
           state = 'B';
         } else {
           cur = do_left(&mut left, &mut right, 0);
           state = 'C';
         }
      },
      'B' => {
         if cur == 0 {
           cur = do_left(&mut left, &mut right, 1);
           state = 'A';
         } else {
           cur = do_right(&mut left, &mut right, 1);
           state = 'D';
         }
      },
      'C' => {
         if cur == 0 {
           cur = do_right(&mut left, &mut right, 1);
           state = 'A';
         } else {
           cur = do_left(&mut left, &mut right, 0);
           state = 'E';
         }
      },
      'D' => {
         if cur == 0 {
           cur = do_right(&mut left, &mut right, 1);
           state = 'A';
         } else {
           cur = do_right(&mut left, &mut right, 0);
           state = 'B';
         }
      },
      'E' => {
         if cur == 0 {
           cur = do_left(&mut left, &mut right, 1);
           state = 'F';
         } else {
           cur = do_left(&mut left, &mut right, 1);
           state = 'C';
         }
      },
      'F' => {
         if cur == 0 {
           cur = do_right(&mut left, &mut right, 1);
           state = 'D';
         } else {
           cur = do_right(&mut left, &mut right, 1);
           state = 'A';
         }
      },
      _ => panic!("Unknown state: {}", state),
    }
  }
  let chksum = left.iter().fold(0, |acc, n| acc + n) + right.iter().fold(0, |acc, n| acc + n) + cur;
println!("left: {:?}", left);
println!("cur: {:?}", cur);
println!("right: {:?}", right);
  println!("Result: {}", chksum);
}

// wrong: 4286

fn main() {
  solve();
}

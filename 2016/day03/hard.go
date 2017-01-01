package main

import (
  "bufio"
  "fmt"
  "io"
  "log"
  "os"
  "strconv"
)


func IsValidTriangle(a, b, c int64) bool {
  return (a + b > c) && (a + c > b) && (b + c > a)
}


func ScanNextInt64Token(sin *bufio.Scanner) (n int64, err error) {
  if !sin.Scan() {
    if sin.Err() != nil {
      err = sin.Err()
    } else {
      err = io.EOF
    }
    return
  }
  n, err = strconv.ParseInt(sin.Text(), 10, 64)
  return
}


func ParseNextLine(sin *bufio.Scanner) (x, y, z int64, err error) {
  x = 0
  y = 0
  z = 0
  x, err = ScanNextInt64Token(sin)
  if err != nil {
    return
  }
  y, err = ScanNextInt64Token(sin)
  if err != nil {
    return
  }
  z, err = ScanNextInt64Token(sin)
  return
}


func ParseThreeTriangles(sin *bufio.Scanner) (xs, ys, zs []int64, err error) {
  xs = make([]int64, 3)
  ys = make([]int64, 3)
  zs = make([]int64, 3)
  for i := 0; i < 3; i++ {
    xs[i], ys[i], zs[i], err = ParseNextLine(sin)
    if err != nil {
      return
    }
  }
  return
}


func main() {
  fin, err := os.Open("input.txt")
  if err != nil {
    log.Fatalln(err)
  }
  defer fin.Close()
  sin := bufio.NewScanner(fin)
  sin.Split(bufio.ScanWords)
  count := 0
  for {
    xs, ys, zs, err := ParseThreeTriangles(sin)
    if err != nil {
      if err != io.EOF {
        log.Fatalln(err.Error())
      }
      break
    }
    if IsValidTriangle(xs[0], xs[1], xs[2]) {
      count++
    }
    if IsValidTriangle(ys[0], ys[1], ys[2]) {
      count++
    }
    if IsValidTriangle(zs[0], zs[1], zs[2]) {
      count++
    }
  }
  fmt.Printf("%d\n", count)
}

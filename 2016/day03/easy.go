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
    x, y, z, err := ParseNextLine(sin)
    if err != nil {
      if err != io.EOF {
        log.Fatalln(err.Error())
      }
      break
    }
    if IsValidTriangle(x, y, z) {
      count++
    }
  }
  fmt.Printf("%d\n", count)
}

package main

import (
  "bufio"
  "fmt"
  "log"
  "os"
  "strconv"
)

type Walker struct {
  X, Y, Dir int
}

func NewWalker() *Walker {
  return &Walker{
    X: 0,
    Y: 0,
    Dir: 0,
  }
}

func (w *Walker) TurnLeft() {
  w.Dir--
  if w.Dir < 0 {
    w.Dir = 3
  }
}

func (w *Walker) TurnRight() {
  w.Dir++
  if w.Dir > 3 {
    w.Dir = 0
  }
}

func (w *Walker) Move(n int) {
  x, y := w.MoveVector()
  w.X += x * n
  w.Y += y * n
}

func (w *Walker) MoveVector() (x, y int) {
  x = 0
  y = 0
  switch w.Dir {
    case 0:
      y = 1
    case 1:
      x = 1
    case 2:
      y = -1
    case 3:
      x = -1
  }
  return
}

func AbsInt(n int) int {
  if n < 0 {
    return -n
  }
  return n
}

func splitDirections(data []byte, atEOF bool) (advance int, token []byte, err error) {
  advance, token, err = bufio.ScanWords(data, atEOF)
  if err == nil && token != nil {
    if token[len(token)-1] == ',' {
      token = token[:len(token)-1]
    }
  }
  return
}

func main() {
  fin, err := os.Open("input.txt")
  if err != nil {
    fmt.Println("unable to open input.txt: %v", err)
    return
  }
  defer fin.Close()
  sin := bufio.NewScanner(fin)
  // Set the split function for the scanning operation.
  sin.Split(splitDirections)
  w := NewWalker()
  for sin.Scan() {
    tok := sin.Text()
    if tok[0] == 'L' {
      w.TurnLeft()
    } else {
      w.TurnRight()
    }
    mc, err := strconv.ParseInt(tok[1:], 10, 32)
    if err != nil {
      log.Fatalln("Bad count " + err.Error())
    }
    move_count := int(mc)
    w.Move(move_count)
  }
  log.Printf("Answer: %d", AbsInt(w.X) + AbsInt(w.Y))
}

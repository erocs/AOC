package main

import (
  "bufio"
  "fmt"
  "log"
  "os"
)


type Keypad struct {
  X, Y int
  Pad [][]int
}


func NewKeypad() *Keypad {
  pad := &Keypad{}
  pad.Pad = [][]int{}
  pad.Pad = append(pad.Pad, []int{-1, -1, 49, -1, -1})
  pad.Pad = append(pad.Pad, []int{-1, 50, 51, 52, -1})
  pad.Pad = append(pad.Pad, []int{53, 54, 55, 56, 57})
  pad.Pad = append(pad.Pad, []int{-1, 65, 66, 67, -1})
  pad.Pad = append(pad.Pad, []int{-1, -1, 68, -1, -1})
  pad.X = 0
  pad.Y = 2
  return pad
}


func (p *Keypad) Value() int {
  return p.Pad[p.Y][p.X]
}


func (p *Keypad) Move(dir byte) bool {
  x := p.X
  y := p.Y
  switch (dir) {
    case 'U':
      y--
    case 'D':
      y++
    case 'L':
      x--
    case 'R':
      x++
    default:
      return false
  }
  if y < 0 || y >= len(p.Pad) {
    return false
  }
  if x < 0 || x >= len(p.Pad[0]) {
    return false
  }
  if p.Pad[y][x] < 0 {
    return false
  }
  p.X = x
  p.Y = y
  return true
}



func main() {
  fin, err := os.Open("input.txt")
  if err != nil {
    log.Fatalln(err)
  }
  defer fin.Close()
  sin := bufio.NewScanner(fin)
  pad := NewKeypad()
  for sin.Scan() {
    line := sin.Text()
    for i := 0; i < len(line); i++ {
      pad.Move(line[i])
    }
    fmt.Printf("%c", pad.Value())
  }
  fmt.Println()
}

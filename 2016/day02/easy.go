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
  pad.Pad = append(pad.Pad, []int{1, 2, 3})
  pad.Pad = append(pad.Pad, []int{4, 5, 6})
  pad.Pad = append(pad.Pad, []int{7, 8, 9})
  pad.X = len(pad.Pad[0]) / 2
  pad.Y = len(pad.Pad) / 2
  return pad
}


func (p *Keypad) Value() int {
  return p.Pad[p.Y][p.X]
}


func (p *Keypad) Move(dir byte) bool {
  switch (dir) {
    case 'U':
      p.Y--
    case 'D':
      p.Y++
    case 'L':
      p.X--
    case 'R':
      p.X++
    default:
      return false
  }
  if p.Y < 0 {
    p.Y = 0
    return false
  }
  if p.Y >= len(p.Pad) {
    p.Y = len(p.Pad) - 1
    return false
  }
  if p.X < 0 {
    p.X = 0
    return false
  }
  if p.X >= len(p.Pad[0]) {
    p.X = len(p.Pad[0]) - 1
    return false
  }
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
    fmt.Printf("%d", pad.Value())
  }
  fmt.Println()
}

package main

import (
  "bufio"
  "errors"
  "fmt"
  "io"
  "log"
  "os"
  "strconv"
)


func ExtractNameSectionChecksum(s string) (name string, section int, checksum string, err error) {
  last_dash := -1
  for i, ch := range s {
    if ch == '-' {
      last_dash = i
    } else if ch == '[' {
      name = s[:last_dash]
      if len(name) <= 0 {
        err = errors.New("Zero length name")
      }
      var sect64 int64
      sect64, err = strconv.ParseInt(s[last_dash+1:i], 10, 64)
      if err == nil {
        section = int(sect64)
      }
      if s[len(s)-1] != ']' {
        err = errors.New("Missing final bracket")
      }
      checksum = s[i+1:len(s)-1]
      if len(checksum) <= 0 || len(checksum) > 5 {
        err = errors.New("Mis-length checksum")
      }
      return
    }
  }
  err = io.EOF
  return
}


func CountLcLetters(s string) map[rune]int {
  counts := make(map[rune]int)
  for _, ch := range s {
    if ch < 'a' || ch > 'z' {
      continue
    }
    if _, ok := counts[ch]; !ok {
      counts[ch] = int('z' - ch)
    }
    counts[ch] += 0x100
  }
  return counts
}


type CounterPair struct {
  Ch rune
  Count int
}


func InsertIfMax(agg []CounterPair, new_obj CounterPair) {
  for i, item := range agg {
    if new_obj.Count > item.Count {
      agg[i] = new_obj
      InsertIfMax(agg, item)
      return
    }
  }
}


func CalculateChecksum(s string) string {
  agg := make([]CounterPair, 5, 5)
  for k, v := range CountLcLetters(s) {
    new_obj := CounterPair{ Ch: k, Count: v }
    if len(agg) < cap(agg) {
      agg = append(agg, new_obj)
    } else {
      InsertIfMax(agg, new_obj)
    }
  }
  result := make([]rune, 0, 5)
  for _, item := range agg {
    result = append(result, item.Ch)
  }
  return string(result)
}


func main() {
  fin, err := os.Open("input.txt")
  if err != nil {
    log.Fatal(err)
  }
  defer fin.Close()
  total := 0
  sin := bufio.NewScanner(fin)
  for sin.Scan() {
    s := sin.Text()
    name, section, checksum, err := ExtractNameSectionChecksum(s)
    if err != nil {
      log.Fatal(err)
    }
    cksum := CalculateChecksum(name)
    if checksum != cksum {
      continue
    }
    total += section
  }
  fmt.Println(total)
}

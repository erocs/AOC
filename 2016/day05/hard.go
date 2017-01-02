package main

import (
  "crypto/md5"
  "fmt"
  "io"
)

const input = "ugkcyxxp"


type Byte struct {
  B byte
}

func main() {
  i := 0
  found := make([]*Byte, 8)
  count := 0
  for {
    hasher := md5.New()
    io.WriteString(hasher, fmt.Sprintf("%s%d", input, i))
    s := fmt.Sprintf("%x", hasher.Sum(nil))
    if s[:5] == "00000" {
      idx := int(s[5])
      if s[5] >= 'a' && s[5] <= 'f' {
       idx -= int('a')
       idx += 10
      } else {
       idx -= int('0')
      }
      val := s[6]
      if idx >= len(found) || found[idx] != nil {
        i++
        continue
      }
      fmt.Printf("%d @ %d: %s\n", count, i, s)
      found[idx] = &Byte{B:val}
      count++
      if count >= 8 {
        break
      }
    }
    i++
  }
  tmp := make([]byte, 8)
  for i := 0; i < 8; i++ {
    tmp[i] = found[i].B
  }
  fmt.Printf("Password: %s\n", string(tmp))
}

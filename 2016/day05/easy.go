package main

import (
  "crypto/md5"
  "fmt"
  "io"
)

const input = "ugkcyxxp"


func main() {
  i := 0
  found := make([]byte, 0, 8)
  for {
    hasher := md5.New()
    io.WriteString(hasher, fmt.Sprintf("%s%d", input, i))
    val := fmt.Sprintf("%x", hasher.Sum(nil))
    if val[:5] == "00000" {
      fmt.Printf("%d @ %d: %s\n", len(found), i, val)
      found = append(found, val[5])
      if len(found) >= 8 {
        break
      }
    }
    i++
  }
  fmt.Printf("Password: %s\n", string(found))
}

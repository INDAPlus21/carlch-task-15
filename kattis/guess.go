package main

import (
  "fmt"
  "sort"
)


func main() {
  var l, i, n int
  for {
    fmt.Printf("INput")
    if _, err := fmt.Scan(&l); err != nil { return; }
    S := true; Q := true; P := true
    var queue, stack, sorted []int
    for {
      b := 0
      if l == 0 {
        break
      }
      l--
      if _, err := fmt.Scan(&i, &n); err != nil {
        break
      }
      if i == 1 {
        queue = append(queue, n)
        stack = append(stack, n)
        sorted = append(sorted, n)
      } else {
        il := len(stack) - 1
        if b > il {
          S, Q, P = false, false, false
          break
        }
        b++
        sort.Ints(sorted)
        x := queue[0]
        y := stack[il];
        z := sorted[il];
        queue = queue[1:]
        stack = stack[:il]
        sorted = sorted[:il]
        if x != n { Q = false; }
        if y != n { S = false; }
        if z != n { P = false; }
      }
    }

    if Q && P { fmt.Println("not sure");
    } else if S && P { fmt.Println("not sure");
    } else if S && Q { fmt.Println("impossible");
    } else if Q { fmt.Println("queue");
    } else if S { fmt.Println("stack");
    } else if P { fmt.Println("priority queue")
    } else { fmt.Println("impossible"); }
  }
}

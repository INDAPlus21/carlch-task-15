#include <stdio.h>
#include <stdlib.h>

int parseint(void) {
  unsigned int c, n;
  if((n = getchar_unlocked() - '0') == '\0')
    return EOF;
  while ((c = getchar_unlocked()) >= '0')
    n = 10*n + c-'0';
  return n;
}

void swap(int *a, int *b) {
  int tmp = *a;
  *a = *b;
  *b = tmp;
}

void build_heap(int arr[], int n, int i) {
  int max = i;
  int left = 2 * i + 1, right = 2 * i + 2;
  if(left < n && arr[left] > arr[max]) max = left;
  if(right < n && arr[right] > arr[max]) max = right;
  if(max != i) {
    swap(&arr[i], &arr[max]);
    build_heap(arr, n, max);
  }
}

void sort(int arr[], int n) {
  for(int i = n / 2 - 1; i >= 0; i--)
    build_heap(arr, n, i);
  for(int i = n - 1; i >= 0; i--) {
    swap(&arr[0], &arr[i]);
    build_heap(arr, i, 0);
  }
}

int main() {
  int size, index, sindex, *arr, *sorted;
  char* type;
  while(1) {
    int is_s = 1, is_q = 1, is_p = 1;
    scanf("%d", &size);
    if(size == EOF) break;
    else if(size == 1) {
      type = (parseint() == 2)
        ? "impossible\n" : "not sure\n";
    }
    else { 
      arr = (int*)malloc(sizeof(int) * size);
      sorted = (int*)malloc(sizeof(int) * size);
      int qx = 0;
      index = 0;
      sindex = 0;
      for(; size > 0; size--) {
        if(parseint() == 1) {
          int temp = parseint();
          arr[index++] = temp;
          sorted[index++] = temp;
          // Add to bag
          continue; }
        // Take out of bag

        // Check stack
        int temp = parseint();
        sort(sorted, index);
        if(is_q && arr[qx++] != temp) is_q = 0;
        if(is_s && arr[index--] != temp) is_s = 0;
        if(is_p && sorted[sindex++] != temp) is_p = 0;
      }
      if((is_s && is_q) || (is_s && is_p) || (is_q && is_p)) type = "not sure\n";
      else if(is_s) type = "stack\n";
      else if(is_q) type = "queue\n";
      else if(is_p) type = "priority queue\n";
      else type = "impossible\n";
      free(arr);
      free(sorted);
    }
    fputs(type, stdout);
  }
  return 0;
}


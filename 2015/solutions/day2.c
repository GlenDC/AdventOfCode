#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include "cstd/std.h"

#define SIZE_LEN 3

int calc_surface(char const* sum) {
  int dim[SIZE_LEN];
  // can this part be automatically generated based on `SIZE_LEN`?!
  if(sscanf(sum, "%dx%dx%d", &dim[0], &dim[1], &dim[2]) == SIZE_LEN) {
    int total, min, cur;
    total = 0;
    min = INT_MAX;

    for(int i = 0; i < SIZE_LEN; ++i) {
      for(int u = i+1; u < SIZE_LEN; ++u) {
	if(i != u) {
	  cur = dim[i] * dim[u];
	  if(cur < min) min = cur;
	  total += cur * 2;
	}
      }
    }

    return total + min;
  }
  
  return 0;
}

int mul(int x, int aux) {
  return x * aux;
}

int calc_ribbon(char const* sum) {
  int dim[SIZE_LEN];
  if(sscanf(sum, "%dx%dx%d", &dim[0], &dim[1], &dim[2]) == SIZE_LEN) {
    heaps(dim, SIZE_LEN);
    return (dim[0] + dim[1]) * 2 + reduce(dim, SIZE_LEN, 1, mul);
  }
  
  return 0;
}

int main(void) {

  int wrapping, ribbon;
  wrapping = ribbon = 0;

  char *line = NULL;
  size_t size;
  while(getline(&line, &size, stdin) != -1 && size > SIZE_LEN) {
    wrapping += calc_surface(line);
    ribbon += calc_ribbon(line);
  }

  printf("%d\n%d\n", wrapping, ribbon);
  return 0;
}

#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#include "std/std.h"

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
  FILE* input = fopen("../input/day2.txt", "r");

  if(!input) {
    printf("Couldn't open file!\n");
    return 1;
  }

  buffer_t* buffer = new_buffer(8);

  int wrapping, ribbon;
  wrapping = ribbon = 0;
  while(bgetline(input, &buffer)) {
    wrapping += calc_surface(buffer->data);
    ribbon += calc_ribbon(buffer->data);
  }

  give_answer(wrapping, 1, "../solutions/day2.txt");
  give_answer(ribbon, 2, "../solutions/day2.txt");
  
  fclose(input);
  free_buffer(buffer);

  return 0;
}

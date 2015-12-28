#include "algorithms.h"

void heaps(int* array, size_t len) {
  int i, u;
  for(i = 0 ; i < len; ++i) {
    int smallest = array[i];
    int pos = i;

    for(u = i+1 ; u < len; ++u) {
      if(array[u] < smallest) {
	smallest = array[u];
	pos = u;
      }
    }

    if(pos != i) {
      array[pos] = array[i];
      array[i] = smallest;
    }
  }
}

int reduce(int* array, size_t len, int aux, int(*cb)(int, int)) {
  for(int i = 0; i < len; ++i) {
    aux = cb(array[i], aux);
  }
  
  return aux;
}

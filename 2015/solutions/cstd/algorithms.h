#ifndef __AOC_STD_ALGORITHMS__
#define __AOC_STD_ALGORITHMS__

#include <stdlib.h>

/**
 * C99 Implementation of heap sort
 */
void heaps(int* array, size_t len);

/**
 * Simple reduce function for integer arrays
 */
int reduce(int* array, size_t len, int aux, int(*cb)(int, int));

#endif // __AOC_STD_ALGORITHMS__

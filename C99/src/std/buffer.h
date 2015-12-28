#ifndef __AOC_STD_BUFFER__
#define __AOC_STD_BUFFER__

#include <stdio.h>

typedef struct buffer_t {
  char* data;
  size_t size;
  size_t len;
} buffer_t;

buffer_t* new_buffer(size_t size);
void free_buffer(buffer_t* buffer);
buffer_t* bputc(char c, buffer_t* buffer);
int bgetline(FILE* input, buffer_t** pBuffer);

#endif // __AOC_STD_BUFFER__

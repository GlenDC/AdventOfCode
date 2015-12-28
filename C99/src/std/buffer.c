#include "buffer.h"
#include <stdlib.h>

buffer_t* new_buffer(size_t size) {
  buffer_t* buffer = (buffer_t*) malloc(sizeof(buffer_t));
  buffer->data = (char*) malloc(sizeof(char) * size);
  buffer->data[0] = 0;
  buffer->size = size;
  buffer->len = 0;
  return buffer;
}

void free_buffer(buffer_t* buffer) {
  free(buffer->data);
  free(buffer);
}

buffer_t* bputc(char c, buffer_t* buffer) {
  if(buffer->len >= buffer->size) {
    buffer-> size *= 2;
    buffer->data = (char*) realloc(buffer->data, buffer->size); 
  }

  buffer->data[buffer->len++] = c;
  return buffer;
}

int bgetline(FILE* input, buffer_t** pBuffer) {
  buffer_t* buffer = *pBuffer;
  buffer->len = 0;
  char c;
  while((c = fgetc(input)) != EOF && c != '\n') {
    buffer = bputc(c, buffer);
  }

  int result = buffer->len != 0;
  if(result) {
    buffer = bputc(0, buffer);
    buffer->len--;
  }

  pBuffer = &buffer;
  return buffer->len != 0;  
}

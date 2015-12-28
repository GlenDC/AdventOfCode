#include <stdio.h>
#include <stdlib.h>

#include "std/std.h"

int travel(char* path, size_t len) {
  int level = 0;
  for(int i = 0 ; i < len; ++i)
    level += path[i] == '(' ? 1 : -1;

  return level;
}

int travel_to_basement(char* path, size_t len) {
  int level = 0;
  for(int i = 0 ; i < len; ++i) {
    level += path[i] == '(' ? 1 : -1;
    if(level == -1)
      return i+1;
  }

  return -1;
}
  
int main(void)
{
  FILE* input = fopen("../input/day1.txt", "r");
  if (!input) {
    printf("Error: Couldn't open input file \n");
    return 1;
  }

  buffer_t* buffer = new_buffer(8);
  if(bgetline(input, &buffer)) {
    int level = travel(buffer->data, buffer->len);
    give_answer(level, 1, "../solutions/day1.txt");

    int position = travel_to_basement(buffer->data, buffer->len);
    give_answer(position, 2, "../solutions/day1.txt");
  }
  
  fclose(input);
  free_buffer(buffer);
  
  return 0;
}

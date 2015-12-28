#include "std.h"
#include <stdio.h>

int give_feedback(int given, int expected) {
  if(given == expected) {
    printf(":) Your answer is correct.\n");
    return 1;
  }

  if(given < expected)
    printf(":( Your answer is too low.\n");
  else
    printf(":( Your answer is too high.\n");

  return 0;
}

int give_answer(int answer, int part, char* solution) {
  FILE* sol = fopen(solution, "r");

  if(sol) {
    buffer_t* buffer = new_buffer(16);
    int i;
    for(i = 0; i < part && bgetline(sol, &buffer); ++i);
    fclose(sol);

    if(i != part) {
      free_buffer(buffer);
      return 0;
    }

    printf("Your answer for part %d: %d\n", part, answer);
    int result = give_feedback(answer, atoi(buffer->data));
    printf("\n");
    free_buffer(buffer);
    return result;
  }

  return 0;
}

#include <stdio.h>
#include <stdlib.h>

int travel(char* path, size_t len) {
  int level = 0;
  for(int i = 0 ; i < len; ++i) {
    if(path[i] == '(') {
      ++level;
    } else if(path[i] == ')') {
      --level;
    } else {
      continue;
    }
  }

  return level;
}

int travel_to_basement(char* path, size_t len) {
  int level = 0;
  for(int i = 0 ; i < len; ++i) {
    if(path[i] == '(') {
      ++level;
    } else if(path[i] == ')') {
      --level;
    } else {
      continue;
    }
  
    if(level == -1)
      return i+1;
  }

  return -1;
}

int main(void)
{
  char *line = NULL;
  size_t size = 0;
  if(getline(&line, &size, stdin) != -1 && size > 0) {
    int level = travel(line, size);
    int position = travel_to_basement(line, size);
    printf("%d\n%d\n", level, position);
  }
  return 0;
}

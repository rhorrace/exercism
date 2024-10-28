#include "binary.h"

int convert(const char *input) {
  int n = strlen(input), number = 0;
  int exp = n;

  for(int i = 0;i < n;++i) {
    if(input[i] != '0' && input[i] != '1') 
      return INVALID;

    --exp;

    if(input[i] == '0')
      continue;

    number += pow(2, exp);
  }

  return number; 
}

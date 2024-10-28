#include "hamming.h"

int compute(const char *lhs, const char *rhs) {
  int n = strlen(lhs), count = 0;

  if(n != (int) strlen(rhs)) return -1;

  for(int i =0;i < n;++i) {
    count += lhs[i] != rhs[i];
  }

  return count;
}

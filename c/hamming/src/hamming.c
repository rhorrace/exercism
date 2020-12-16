#include "hamming.h"


int compute(const char *lhs, const char *rhs) {
  if(!lhs || !rhs)
    return -1;
  int m = strlen(lhs), n = strlen(rhs);
  if(m == n) {
    int i, sum = 0;
    for(i = 0;i < n;++i) {
      if(lhs[i] != rhs[i])
        ++sum;
    }
    return sum;
  }
  return -1;
}

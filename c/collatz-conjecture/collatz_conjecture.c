#include "collatz_conjecture.h"

int steps(int start) {
  if(start <= 0) return ERROR_VALUE;

  int n = start, step_count = 0;

  while (n != 1) {
    n = (n & 1) ? 3 * n + 1 : n >> 1;

    ++step_count;
  }

  return step_count;
}

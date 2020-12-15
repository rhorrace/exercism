#include "armstrong_numbers.h"

bool is_armstrong_number(int candidate) {
  int count = 0, num = candidate, sum = 0;
  while(num > 0) {
    num /= 10;
    ++count;
  }
  num = candidate;
  while(num > 0) {
    sum += pow(num % 10, count);
    num /= 10;
  }
  return sum == candidate;
}

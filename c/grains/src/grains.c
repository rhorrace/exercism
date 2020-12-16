#include "grains.h"

uint64_t square(uint8_t index) {
  return (index >= 1 && index <= 64) ? (uint64_t) pow(2, index - 1) : 0;
}

uint64_t total(void) {
  uint8_t i;
  uint64_t sum = 0ull;
  for(i = 1;i <= 64;++i)
    sum += square(i);
  return sum;
}

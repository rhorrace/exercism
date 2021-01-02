#include "sieve.h"

uint32_t sieve(uint32_t limit, uint32_t * primes, size_t max_primes) {
  bool is_prime[limit+1];
  size_t pos = 0, i, j;

  for(i = 2;i <= limit;++i)
    is_prime[i] = true;

  for(i = 2;i <= limit;++i) {
    if(!is_prime[i])
      continue;

    primes[pos++] = i;

    if(pos == max_primes)
      break;

    for(j = i = i;j <= limit;j += i)
      is_prime[j] = false;
  }
  return (uint32_t) pos;
}

#include "armstrong_numbers.h"

bool is_armstrong_number(int candidate) {
    if (candidate == 0) return true;
    
    int n = candidate, calc = 0;
    int exp = (n == 0) ? 1 : log10(n) + 1;

    while (n != 0) {
        calc += pow((n % 10), exp);
        n /= 10;
    }

    return calc == candidate;
}

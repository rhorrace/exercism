#include "perfect_numbers.h"

kind classify_number(int num) {
    if(num < 1) return ERROR;

    if(num == 1) return DEFICIENT_NUMBER;

    float root_n = sqrt(num);
    int count = 1;

    for(int i = 2;i <= root_n;++i) {
        if(num % i != 0) {
            continue;
        }

        count += i;

        if (i != root_n) count += num / i;
    }

    if(count < num) return DEFICIENT_NUMBER;

    if(count > num) return ABUNDANT_NUMBER;

    return PERFECT_NUMBER;

}


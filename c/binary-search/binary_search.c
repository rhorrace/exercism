#include "binary_search.h"

const int *binary_search(int value, const int *arr, size_t length) {
    if (!arr || length == 0) return NULL;

    size_t l = 0, r = length - 1;

    while (l <= r) {
        int m = l + (r - l) / 2;

        if (m == -1) return NULL;

        if(arr[m] == value) return &arr[m];

        if (arr[m] < value)
            l = m + 1;
        else
            r = m - 1;
    }

    return NULL;
}
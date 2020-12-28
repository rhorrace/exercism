#include "binary_search.h"

const int *binary_search(int value, const int *arr, size_t length) {
  if(!arr)
    return NULL;
  int fst = 0, lst = length - 1;
  int mid = (fst + lst) / 2;
  while(fst <= lst) {
    if(arr[mid] == value) {
      return &arr[mid];
    } else if(arr[mid] < value)
      fst = mid + 1;
    else
      lst = mid - 1;

    mid = (fst + lst) / 2;
  }
  return NULL;
}

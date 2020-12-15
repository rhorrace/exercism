#include "isogram.h"

bool is_isogram(const char* phrase) {
  if(!phrase)
    return false;

  bool found[26] = {false};

  int i, n = strlen(phrase);
  for(i = 0;i < n;++i) {
    if(!isalpha(phrase[i]))
      continue;
    char letter = toupper(phrase[i]);
    int index = letter - 'A';
    if(found[index])
      return false;
    found[index] = true;
  }
  return true;
}

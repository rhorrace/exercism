#include "acronym.h"

char *abbreviate(const char *phrase) {
  char temp[100] = {'\0'};
  if(!phrase || phrase[0] == '\0')
    return NULL;
  int i, pos = 0, n = strlen(phrase);
  for(i = 0;i < n-1;++i) {
    if(i == 0 && isalnum(phrase[i]))
      temp[pos++] = toupper(phrase[i]);
    else if(issep(phrase[i]) && isalnum(phrase[i+1]))
      temp[pos++] = toupper(phrase[i+1]);
  }
  char* abbr = malloc(sizeof(char) * (pos + 1));
  strcpy(abbr, temp);
  return abbr;
}

bool issep(char c) {
  return c == ' ' || c == '_' || c == '-';
}

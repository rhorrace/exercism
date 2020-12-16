#ifndef ACRONYM_H
#define ACRONYM_H

#include <ctype.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

char *abbreviate(const char *phrase);

bool issep(char c);

#endif

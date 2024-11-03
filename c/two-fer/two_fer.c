#include "two_fer.h"

void two_fer(char *buffer, const char *name) {
    if(!name) {
        strcpy(buffer, "One for you, one for me.");
        return;
    }

    snprintf(buffer, 100, "One for %s, one for me.", name);
}

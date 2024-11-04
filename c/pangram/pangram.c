#include "pangram.h"

bool is_pangram(const char *sentence) {
    if(!sentence) return false;

    int n = strlen(sentence), flags = 0;

    // Activate 26 flags in form of bits
    for(int i = 0;i < 26;++i) {
        flags |= 1 << i;
    }

    for(int i = 0;i < n;++i) {
        if(flags == 0) return true;

        if(!isalpha(sentence[i])) continue;

        // Get flag position
        int val = (int) tolower(sentence[i]) - 97;
        int flag = 1 << val;

        // Clear flag position
        flags &= ~flag;
    }

    return flags == 0;
}
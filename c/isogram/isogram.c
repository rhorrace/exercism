#include "isogram.h"

bool is_isogram(const char phrase[]) {
    if (!phrase) return false;
    int flags = 0;
    int n = strlen(phrase);

    if (n == 0) return true;

    for(int i = 0;i < n;++i) {
        if(!isalpha(phrase[i])) continue;

        int flag = (int) tolower(phrase[i]) % 26;

        if ((flags & (1 << flag)) != 0) return false;

        flags |= 1 << flag;
    }

    return true;
}

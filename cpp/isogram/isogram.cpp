#include "isogram.h"

namespace isogram {

  bool is_isogram(std::string word) {
    std::unordered_set<char> letters;
    for(char& c : word) {
      if(!isalpha(c)) {
        continue;
      } else if(letters.find(tolower(c)) == letters.end()) {
        letters.insert(tolower(c));
      } else {
        return false;
      }
    }
    return true;
  }

}  // namespace isogram

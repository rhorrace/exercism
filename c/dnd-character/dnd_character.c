#include "dnd_character.h"

int ability(void) {
    srand(time(0));
    return (rand() % 18) + 3;
}

int modifier(int score) {
    return floor((double) (score - 10) / 2);
}

dnd_character_t make_dnd_character(void) {
    dnd_character_t new_character = {
        ability(), // strength
        ability(), // dexterity
        ability(), // constitution
        ability(), // intelligence
        ability(), // wisdom
        ability(), // charisma
        10         // base hitpoints
    };
    
    new_character.hitpoints += modifier(new_character.constitution);

    return new_character; 
}
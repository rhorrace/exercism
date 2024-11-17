#include "resistor_color_trio.h"

resistor_value_t color_code(resistor_band_t colors[]) {
    resistor_value_t temp = {0, OHMS};

    temp.value = (colors[0] * 10 + colors[1]);

    switch (colors[2]) {
        case WHITE:
            temp.unit = GIGAOHMS;
            break;
        case GREY:
            temp.value *= 100;
            temp.unit = MEGAOHMS;
            break;
        case VIOLET:
            temp.value *= 10;
            temp.unit = MEGAOHMS;
            break;
        case BLUE:
            temp.unit = MEGAOHMS;
            break;
        case GREEN:
            temp.value *= 100;
            temp.unit = KILOOHMS;
            break;
        case YELLOW:
            temp.value *= 10;
            temp.unit = KILOOHMS;
            break;
        case ORANGE:
            temp.unit = KILOOHMS;
            break;
        case RED:
            temp.value *= 100;
            break;
        case BROWN:
            temp.value *= 10;
            break;
        case BLACK: break;
    }

    if (temp.value >= 1000) {
        temp.value /= 1000;
        ++temp.unit;
    }

    return temp;
}

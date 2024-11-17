#include "robot_simulator.h"

robot_status_t robot_create(robot_direction_t direction, int x, int y) {
    robot_position_t position = {.x = x, .y = y};
    robot_status_t robot = {.direction = direction, .position = position};

    return robot;
}

void robot_move(robot_status_t *robot, const char *commands) {
    if (!robot || !commands) return;

    int n = strlen(commands);

    if (n == 0) return;

    for(int i = 0;i < n;++i) {
        switch (robot->direction) {
            case DIRECTION_NORTH:
                if (commands[i] == 'A') robot->position.y++;
                else if (commands[i] == 'L') robot->direction = DIRECTION_WEST;
                else robot->direction = DIRECTION_EAST;
                break;
            case DIRECTION_EAST:
                if (commands[i] == 'A') robot->position.x++;
                else if (commands[i] == 'L') robot->direction = DIRECTION_NORTH;
                else robot->direction = DIRECTION_SOUTH;
                break;
            case DIRECTION_SOUTH:
                if (commands[i] == 'A') robot->position.y--;
                else if (commands[i] == 'L') robot->direction = DIRECTION_EAST;
                else robot->direction = DIRECTION_WEST;
                break;
            case DIRECTION_WEST:
                if (commands[i] == 'A') robot->position.x--;
                else if (commands[i] == 'L') robot->direction = DIRECTION_SOUTH;
                else robot->direction = DIRECTION_NORTH;
                break;
            default: break;
        }
    }
}

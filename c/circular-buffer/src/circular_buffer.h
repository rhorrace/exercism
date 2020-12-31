#ifndef CIRCULAR_BUFFER_H
#define CIRCULAR_BUFFER_H


#include <errno.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define SIZE_T_MEMBERS 4

typedef int buffer_value_t;

typedef struct {
    size_t capacity;
    size_t filled_cells;
    size_t oldest_index;
    size_t newest_index;
    buffer_value_t buffer[];
} circular_buffer_t;

circular_buffer_t * new_circular_buffer(size_t capacity);
int16_t read(circular_buffer_t *buffer, buffer_value_t *read_value);
int16_t write(circular_buffer_t *buffer, buffer_value_t value);
void clear_buffer(circular_buffer_t *buffer);
int16_t overwrite(circular_buffer_t *buffer, buffer_value_t value);
void delete_buffer(circular_buffer_t *buffer);
void inc_mod(size_t *x, size_t mod);

#endif

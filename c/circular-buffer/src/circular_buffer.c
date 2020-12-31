#include "circular_buffer.h"

#define SIZE_T_MEMBERS 4

circular_buffer_t* new_circular_buffer(size_t capacity) {
    circular_buffer_t* buffer = malloc(sizeof(size_t) * SIZE_T_MEMBERS +
                                       sizeof(buffer_value_t) * capacity);
    buffer->capacity = capacity;
    clear_buffer(buffer);

    return buffer;
}

int16_t read(circular_buffer_t *buffer, buffer_value_t *read_value) {
    if (buffer->filled_cells == 0) {
        errno = ENODATA;
        return EXIT_FAILURE;
    } else {
        *read_value = buffer->buffer[buffer->oldest_index];
        memset(&buffer->buffer[buffer->oldest_index], 0,
               sizeof(buffer_value_t));
        inc_mod(&buffer->oldest_index, buffer->capacity);
        --buffer->filled_cells;
        return EXIT_SUCCESS;
    }
}

int16_t write(circular_buffer_t *buffer, buffer_value_t value) {
    if (buffer->filled_cells == buffer->capacity) {
        errno = ENOBUFS;
        return EXIT_FAILURE;
    } else {
        buffer->buffer[buffer->newest_index] = value;
        inc_mod(&buffer->newest_index, buffer->capacity);
        ++buffer->filled_cells;
        return EXIT_SUCCESS;
    }
}

void clear_buffer(circular_buffer_t *buffer) {
    buffer->filled_cells =
    buffer->oldest_index =
    buffer->newest_index = 0;
    memset(buffer->buffer, 0, buffer->capacity * sizeof(buffer_value_t));
}

int16_t overwrite(circular_buffer_t *buffer, buffer_value_t value) {
    if (buffer->filled_cells != buffer->capacity)
        write(buffer, value);
    else {
        buffer->buffer[buffer->newest_index] = value;
        inc_mod(&buffer->oldest_index, buffer->capacity);
        inc_mod(&buffer->newest_index, buffer->capacity);
    }

    return EXIT_SUCCESS;
}

void delete_buffer(circular_buffer_t *buffer) {
    free(buffer);
    buffer = NULL;
}

void inc_mod(size_t *x, size_t mod) {
    ++(*x);
    *x %= mod;
}

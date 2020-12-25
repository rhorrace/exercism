#ifndef GRADE_SCHOOL_H
#define GRADE_SCHOOL_H

#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

#define MAX_NAME_LENGTH 20
#define MAX_STUDENTS 20

typedef struct {
   uint8_t grade;
   char *name;
} student_t;

typedef struct {
   size_t count;
   student_t students[MAX_STUDENTS];
} roster_t;

bool add_student(char* name, uint8_t grade);

roster_t get_grade(uint8_t grade);

roster_t get_roster();

void clear_roster();

void sort_latest(student_t students[]);

void swap(student_t* a, student_t* b);

#endif

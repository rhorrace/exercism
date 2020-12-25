#include "grade_school.h"

static roster_t roster = { 0 };

bool add_student(char* name, uint8_t grade) {
  if(roster.count == MAX_STUDENTS)
    return false;
  roster.students[roster.count].grade = grade;
  roster.students[roster.count].name = malloc(sizeof(char) * (strlen(name) + 1));
  strcpy(roster.students[roster.count++].name, name);
  sort_latest(roster.students);
  return true;
}

roster_t get_grade(uint8_t grade) {
  roster_t temp = {0};
  size_t i;
  for(i = 0;i < roster.count;++i) {
    if(roster.students[i].grade == grade)
      temp.students[temp.count++] = roster.students[i];
  }
  return temp;
}

roster_t get_roster() {
  return roster;
}

void clear_roster() {
  while(roster.count != 0) {
    roster.students[--roster.count].grade = 0;
    free(roster.students[roster.count].name);
    roster.students[roster.count].name = NULL;
  }
}

void sort_latest(student_t students[]) {
  size_t i;
  for(i = roster.count-1;i > 0;--i) {
    if(students[i].grade > students[i-1].grade)
      break;
    if(students[i].grade < students[i-1].grade || strcmp(students[i].name, students[i-1].name) < 0)
      swap(&students[i], &students[i-1]);
  }
}

void swap(student_t* a, student_t* b) {
  student_t temp = *a;
  *a = *b;
  *b = temp;
}

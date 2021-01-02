#include "list_ops.h"

list_t *new_list(size_t length, list_value_t values[]) {
  list_t* temp = malloc(sizeof(list_t));

  temp->length = length;

  if(!values)
    temp->values = NULL;
  else {
    temp->values = malloc(sizeof(list_value_t) * length);
    size_t i = 0;
    for(i = 0;i < length;++i) {
      temp->values[i] = values[i];
    }
  }

  return temp;
}

list_t *append_list(list_t * list1, list_t * list2) {
  if(!list1 && !list2)
    return NULL;
  else if(list1 && !list2)
    return list1;
  else if(!list1 && list2)
    return list2;
  else {
    list_t* temp = malloc(sizeof(list_t));
    temp->length = list1->length + list2->length;
    temp->values = malloc(sizeof(list_value_t) * temp->length);
    size_t pos = 0, i;
    for(i = 0;i < list1->length;++i)
      temp->values[pos++] = list1->values[i];
    for(i = 0;i < list2->length;++i)
      temp->values[pos++] = list2->values[i];
    return temp;
  }
}

list_t *filter_list(list_t * list, bool(*filter) (list_value_t value)) {
  if(!list)
    return NULL;
  list_value_t filtered[list->length];
  size_t len = 0, i;
  for(i = 0;i < list->length;++i) {
    if(filter(list->values[i]))
      filtered[len++] = list->values[i];
  }
  list_t* filtered_list = malloc(sizeof(list_t));
  filtered_list->length = len;
  filtered_list->values = malloc(sizeof(list_value_t) * len);
  for(i = 0;i < len;++i) {
    filtered_list->values[i] = filtered[i];
  }
  return filtered_list;
}

size_t length_list(list_t * list) {
  return (!list) ? 0 : list->length;
}

list_t *map_list(list_t * list, list_value_t(*map) (list_value_t value)) {
  if(!list)
    return NULL;
  list_t* mapped_list = malloc(sizeof(list_t));
  mapped_list->length = list->length;
  mapped_list->values = malloc(sizeof(list_value_t) * list->length);
  size_t i;
  for(i = 0;i < list->length;++i) {
    mapped_list->values[i] = map(list->values[i]);
  }
  return mapped_list;
}

list_value_t foldl_list(list_t * list, list_value_t initial,
    list_value_t(*foldl) (list_value_t value,
      list_value_t initial)) {
  if(list) {
    size_t i = 0;
    for(i = 0;i < list->length;++i)
      initial = foldl(list->values[i], initial);
  }
  return initial;
}

list_value_t foldr_list(list_t * list, list_value_t initial,
    list_value_t(*foldr) (list_value_t value,
      list_value_t initial)) {
  if(list && list->length > 0) {
    size_t i = list->length - 1;
    do {
      initial = foldr(list->values[i--], initial);
    } while(i > 0);
    initial = foldr(list->values[0], initial);
  }
  return initial;
}

list_t *reverse_list(list_t * list) {
  if(!list)
    return NULL;
  list_t* reversed_list = malloc(sizeof(list_t));
  reversed_list->length = list->length;
  reversed_list->values = malloc(sizeof(list_value_t) * list->length);
  size_t i, pos = list->length - 1;
  for(i = 0;i < list->length;++i)
    reversed_list->values[i] = list->values[pos--];
  return reversed_list;
}

void delete_list(list_t * list) {
  if(list) {
    size_t i;
    for(i = 0;i < list->length;++i)
      list->values[i] = 0;
    free(list->values);
    list->length = 0;
    free(list);
  }
}

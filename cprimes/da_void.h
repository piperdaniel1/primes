// Creator: Grant Conklin
// Date: 24.Oct.2022
// File: da_void.h

#ifndef DYNAMIC_ARY_VOID
#define DYNAMIC_ARY_VOID

#include <stdio.h>

/*********************************************************************
** Struct: Void_Ary
** Description: array struct for void pointers
** Use: storing void pointer data
**********************************************************************/
struct Void_Ary {
    int length;
    int capacity;
    void **ary;
};

struct Void_Ary* ary_void_create();
void ary_void_free(struct Void_Ary*);

void ary_void_push_front(struct Void_Ary* ary, void * data);
void ary_void_push_back(struct Void_Ary* ary, void * data);
void* ary_void_get_index(struct Void_Ary* ary, int index);
void ary_void_set_index(struct Void_Ary* ary, int index, void * data);
void ary_void_insert(struct Void_Ary* ary, int index, void * data);
void ary_void_remove(struct Void_Ary* ary, int index);
int ary_void_get_length(struct Void_Ary* ary);

#endif

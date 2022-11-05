// Creator: Grant Conklin
// Date: 24.Oct.2022
// File: da_void.c

#include <stdio.h>
#include <stdlib.h>

#include "da_void.h"

/*********************************************************************
** Function: da_void_create
** Description: creates void dynamic array struct
** Parameters: none
** Pre-Conditions: none
** Post-Conditions: heap dynamic array struct created
*********************************************************************/
struct Void_Ary *ary_void_create()
{
    struct Void_Ary *ary = malloc(sizeof(struct Void_Ary));
    ary->length = 0;
    ary->capacity = 1;
    ary->ary = malloc(sizeof(void *) * ary->capacity);
    return ary;
}

/*********************************************************************
** Function: ary_void_free
** Description: deletes ary void struct
** Parameters: pointer to dynamic array struct
** Pre-Conditions: da_void_create has been called
** Post-Conditions: heap dynamic array struct deleted
*********************************************************************/
void ary_void_free(struct Void_Ary *ary)
{
    free(ary->ary);
    free(ary);
}

/*********************************************************************
** Function: ary_void_push_front
** Description: item on front of list
** Parameters: data, pointer to dynamic array struct
** Pre-Conditions: none
** Post-Conditions: item added
*********************************************************************/
void ary_void_push_front(struct Void_Ary *ary, void *data)
{
    ary_void_insert(ary, 0, data);
}

/*********************************************************************
** Function: ary_void_push_back
** Description: item on back of list
** Parameters: data, pointer to dynamic array struct
** Pre-Conditions: none
** Post-Conditions: item added
*********************************************************************/
void ary_void_push_back(struct Void_Ary *ary, void *data)
{
    ary_void_insert(ary, ary->length, data);
}

/*********************************************************************
** Function: ary_void_get_index
** Description: retruns item at index
** Parameters: index, pointer to dynamic array struct
** Pre-Conditions: none
** Post-Conditions: item returned
*********************************************************************/
void *ary_void_get_index(struct Void_Ary *ary, int index)
{
    return ary->ary[index];
}

/*********************************************************************
** Function: ary_void_set_index
** Description: sets data at index
** Parameters: index, data, pointer to dynamic array struct
** Pre-Conditions: none
** Post-Conditions: index data changed
*********************************************************************/
void ary_void_set_index(struct Void_Ary *ary, int index, void *data)
{
    ary->ary[index] = data;
}

/*********************************************************************
** Function: ary_void_insert
** Description: inserts item at index
** Parameters: index, data, pointer to dynamic array struct
** Pre-Conditions: none
** Post-Conditions: item inserted at index
*********************************************************************/
void ary_void_insert(struct Void_Ary *ary, int index, void *data)
{
    int i;
    if (ary->length == ary->capacity)
    {
        ary->capacity *= 2;
        ary->ary = realloc(ary->ary, sizeof(void *) * ary->capacity);
    }
    for (i = ary->length; i > index; i--)
    {
        ary->ary[i] = ary->ary[i - 1];
    }
    ary->ary[index] = data;
    ary->length++;
}

/*********************************************************************
** Function: ary_void_remove
** Description: removes index
** Parameters: index, pointer to dynamic array struct
** Pre-Conditions: none
** Post-Conditions: index deleted
*********************************************************************/
void ary_void_remove(struct Void_Ary *ary, int index)
{
    int i;
    for (i = index; i < ary->length - 1; i++)
    {
        ary->ary[i] = ary->ary[i + 1];
    }
    ary->length--;
}

/*********************************************************************
** Function: ary_void_get_length
** Description: returns length of array
** Parameters: pointer to dynamic array struct
** Pre-Conditions: none
** Post-Conditions: int returned
*********************************************************************/
int ary_void_get_length(struct Void_Ary *ary)
{
    return ary->length;
}
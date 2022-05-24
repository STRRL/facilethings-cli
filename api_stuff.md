# FacileThings API

## Retrieve a list

**Description**: Get the items of the list like the web app, allowing to paginate and set filters. It only returns the necessary fields to build the list.

**URL**: `GET https://api.facilethings.com/v1/stuff/get_list?list=N`

Parameters:

- List. Number (see values below). Mandatory!
- Page. Number. Only needed if total >250, in order to show the next pages (see explanation below)
- Tags. List of strings, to filter by tags. &tags=tag1,tag2
- Person. String, to filter by person tag. &person=juan
- Area. Id of an Area of Responsibility. Filter.
- Goal. Id of a Goal. Filter.
- Focus. Boolean, to get only the focused items (important). Only valid on Next Actions. &focus=true
- Time: Number, to get the items with shorter expected time, in minutes. Only valid on Next Actions. &time=60
- Energy: high/low. Filter. Only valid on Next Actions. &energy=high
- Urgent. Boolean, to get only the urgent items (priority). Only valid on Next Actions. &urgent=true

Lists values:

0. Inbox
1. Someday/Maybe
2. Reference
3. Calendar
4. Next Actions
5. Waiting For
6. Trash
7. Done

It returns a JSON structure with two keys:

- total: informs about the total number of items in the list
- list: a list of stuff items

Notes:

- if total<=250, the list contains all the items and no pagination is needed.
- if total>250, list only contains the first 50 items, and you need to paginate. You get the next page by incrementing the "page" parameter.

Example:
```
{
  "total": 13,
  "list": [
  {
    "stuff": {
      "id": 6163580,
      "text": "doing action",
      "created_at": "2019-09-30T17:20:54+02:00",
      "reminder": null,
      "time": null,
      "energy": null,
      "priority": 2,
      "important": 1,
      "goal_id": null,
      "area_id": 15183,
      "checkpoints_count": 0,
      "checkpoints_done": 0,
      "writings_count": 0,
      "project": {
          "name": "Kanban"
      },
      "tags": []
    }
  },
  {
    "stuff": {
      "id": 6156135,
      "text": "kanban 3.0",
      ...
    }
  }
...
}
```

## Stuff object

### Create (Capture)

**Description**: Creates a new stuff.

**URL**: `POST https://api.facilethings.com/v1/stuff`

BODY:
```
{
  "stuff": {
    "text": “New stuff"
  }
}
```

**Returns**: Stuff object created.

### Update

**Description**: Updates the stuff with the ID.

**URL**: `PUT https://api.facilethings.com/v1/stuff/:id`

BODY:
```
{
  "stuff": {
    "text": Updates text"
  }
}
```

**Returns**: Stuff object updated

### Retrieve

**Description**: Gets the stuff with the ID.

**URL**: `GET https://api.facilethings.com/v1/stuff/:id`

**Returns**: Stuff object with the ID.

### Incubate

**Description**: Moves an item to the Someday/Maybe list.

**URL**: `PUT https://api.facilethings.com/v1/stuff/:id/incubate`

**Returns**: The stuff object modified.

### Reference

**Description**: Moves an item to the Reference Material list.

**URL**: `PUT https://api.facilethings.com/v1/stuff/:id/reference`

**Returns**: The stuff object modified.

### Waiting

**Description**: Moves an item to the Waiting For list.

**URL**: `PUT https://api.facilethings.com/v1/stuff/:id/waiting`

**Returns**: The stuff object modified.

### Action

**Description**: Moves an item either to the Next Action list (reminder==nil) or the Calendar list (reminder!=nil)

**URL**: `PUT https://api.facilethings.com/v1/stuff/:id/action`

**Returns**: The stuff object modified.

### Trash

**Description**: Moves an item to the Trash.

**URL**: `PUT https://api.facilethings.com/v1/stuff/:id/trash`

**Returns**: The stuff object modified.

### Done

**Description**: Moves an item to the Done list.

**URL**: `PUT https://api.facilethings.com/v1/stuff/:id/done`

**Returns**: The stuff object modified.

### Undo

**Description**: Undo a Trash or Done movement, moving the item to the previous list

**URL**: `PUT https://api.facilethings.com/v1/stuff/:id/undo`

**Returns**: The stuff object modified.

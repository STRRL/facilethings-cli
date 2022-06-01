# \DefaultApi

All URIs are relative to *https://api.facilethings.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**_stuff_get_list**](DefaultApi.md#_stuff_get_list) | **GET** /stuff/get_list | 



## _stuff_get_list

> crate::models::StuffResponse _stuff_get_list(list, page, tags, person, area, goal, focus, time, energy, urgent)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list** | **String** | The list is an enum, with meaning:  0. Inbox 1. Someday/Maybe 2. Reference 3. Calendar 4. Next Actions 5. Waiting For 6. Trash 7. Done  | [required] |[default to 0]
**page** | Option<**String**> |  Number. Only needed if total >250, in order to show the next pages (see explanation below) |  |
**tags** | Option<**String**> | Filter stuff by tags. Represented in the dot-separated string array. |  |
**person** | Option<**String**> | Filter by the person tag. |  |
**area** | Option<**String**> | Filter by the ID of Area of Responsibility. |  |
**goal** | Option<**String**> | Filter by the ID of a Goal. |  |
**focus** | Option<**String**> | Boolean, to get only the focused items (important). Only valid on Next Actions. |  |
**time** | Option<**String**> | Number, to get the items with a shorter expected time, in minutes. Only valid on Next Actions. &time=60 |  |
**energy** | Option<**String**> | high/low. Filter. Only valid on Next Actions. &energy=high |  |
**urgent** | Option<**String**> | Urgent. Boolean, to get only the urgent items (priority). Only valid on Next Actions. &urgent=true |  |

### Return type

[**crate::models::StuffResponse**](StuffResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


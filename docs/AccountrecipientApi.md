# \AccountrecipientApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**recipient_router_get_all_messages_for_a_recipient**](AccountrecipientApi.md#recipient_router_get_all_messages_for_a_recipient) | **Get** /account/recipient/{recipient}/messages | 
[**recipient_router_get_all_messages_for_a_recipient_from_a_node**](AccountrecipientApi.md#recipient_router_get_all_messages_for_a_recipient_from_a_node) | **Get** /account/recipient/node/{recipient}/messages | 


# **recipient_router_get_all_messages_for_a_recipient**
> Vec<::models::ModelsQEmailMessage> recipient_router_get_all_messages_for_a_recipient(x_account_api_key, recipient)


Find all messages sent to a specific recipient

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **recipient** | **String**| email of the recipient | 

### Return type

[**Vec<::models::ModelsQEmailMessage>**](models.QEmailMessage.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **recipient_router_get_all_messages_for_a_recipient_from_a_node**
> Vec<::models::ModelsQEmailMessage> recipient_router_get_all_messages_for_a_recipient_from_a_node(x_account_api_key, recipient)


Find all message sent to a recipient from a specific node

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **recipient** | **String**| email of the recipient | 

### Return type

[**Vec<::models::ModelsQEmailMessage>**](models.QEmailMessage.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


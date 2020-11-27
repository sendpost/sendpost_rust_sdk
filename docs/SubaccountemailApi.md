# \SubaccountemailApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**email_router_send_email**](SubaccountemailApi.md#email_router_send_email) | **Post** /subaccount/email/ | 
[**email_router_send_email_with_template**](SubaccountemailApi.md#email_router_send_email_with_template) | **Post** /subaccount/email/template | 


# **email_router_send_email**
> Vec<::models::ModelsEmailResponse> email_router_send_email(x_sub_account_api_key, body)


Send Email To Contacts

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **body** | [**ModelsEmailMessage**](ModelsEmailMessage.md)| The Email Message | 

### Return type

[**Vec<::models::ModelsEmailResponse>**](models.EmailResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **email_router_send_email_with_template**
> Vec<::models::ModelsEmailResponse> email_router_send_email_with_template(x_sub_account_api_key, body)


Send Email To Contacts With Template

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_sub_account_api_key** | **String**| Sub-Account API Key | 
  **body** | [**ModelsEmailMessage**](ModelsEmailMessage.md)| The Email Message | 

### Return type

[**Vec<::models::ModelsEmailResponse>**](models.EmailResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \AccountpaymentApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payment_router_create_customer_portal**](AccountpaymentApi.md#payment_router_create_customer_portal) | **Post** /account/payment/portal | 
[**payment_router_create_payment_subscription**](AccountpaymentApi.md#payment_router_create_payment_subscription) | **Post** /account/payment/subscription | 
[**payment_router_handle_payment_webhook**](AccountpaymentApi.md#payment_router_handle_payment_webhook) | **Post** /account/payment/webhook | 


# **payment_router_create_customer_portal**
> ::models::ModelsBillingPortalSession payment_router_create_customer_portal(x_account_api_key)


Create Customer Portal for account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 

### Return type

[**::models::ModelsBillingPortalSession**](models.BillingPortalSession.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **payment_router_create_payment_subscription**
> ::models::ModelsPaymentStatus payment_router_create_payment_subscription(x_account_api_key, body)


Create Payment Subscription for Stripe

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 
  **body** | [**ModelsPaymentOptions**](ModelsPaymentOptions.md)| PaymentOptions content | 

### Return type

[**::models::ModelsPaymentStatus**](models.PaymentStatus.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **payment_router_handle_payment_webhook**
> payment_router_handle_payment_webhook()


Handle Payment Related Webhooks

### Required Parameters
This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


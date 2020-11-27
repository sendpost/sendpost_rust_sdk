# \AccountonboardingApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**onboarding_router_get_onboarding_checklist**](AccountonboardingApi.md#onboarding_router_get_onboarding_checklist) | **Get** /account/onboarding/checklist | 


# **onboarding_router_get_onboarding_checklist**
> ::models::ModelsOnboardingChecklist onboarding_router_get_onboarding_checklist(x_account_api_key)


Gets Onboarding Checklist data for account if not present creates a default entry

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_account_api_key** | **String**| Account API Key | 

### Return type

[**::models::ModelsOnboardingChecklist**](models.OnboardingChecklist.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


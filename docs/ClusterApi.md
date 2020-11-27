# \ClusterApi

All URIs are relative to *https://api.sendpost.io/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cluster_router_delete_item_from_cache_of_every_node_in_cluster**](ClusterApi.md#cluster_router_delete_item_from_cache_of_every_node_in_cluster) | **Delete** /cluster/cache | 
[**cluster_router_delete_item_from_cache_of_specific_node_in_cluster**](ClusterApi.md#cluster_router_delete_item_from_cache_of_specific_node_in_cluster) | **Delete** /cluster/cache/node | 


# **cluster_router_delete_item_from_cache_of_every_node_in_cluster**
> cluster_router_delete_item_from_cache_of_every_node_in_cluster(x_system_api_key, optional)


Delete item from cache of every node in cluster

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_system_api_key** | **String**| System API Key | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_system_api_key** | **String**| System API Key | 
 **name** | **String**| cache name | 
 **key** | **String**| cache item key to delete | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **cluster_router_delete_item_from_cache_of_specific_node_in_cluster**
> cluster_router_delete_item_from_cache_of_specific_node_in_cluster(x_system_api_key, optional)


Delete item from cache of specific node in cluster

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_system_api_key** | **String**| System API Key | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **x_system_api_key** | **String**| System API Key | 
 **name** | **String**| cache name | 
 **key** | **String**| cache item key to delete | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


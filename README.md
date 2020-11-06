# Rust API client for swagger

SendPost API to send transactional emails reliably

## Overview
This API client was generated by the [swagger-codegen](https://github.com/swagger-api/swagger-codegen) project.  By using the [swagger-spec](https://github.com/swagger-api/swagger-spec) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: io.swagger.codegen.languages.RustClientCodegen

## Installation
Put the package under your project folder and add the following in import:
```
    "./swagger"
```

## Documentation for API Endpoints

All URIs are relative to *https://api.sendpost.io/api/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccounteventApi* | [**event_router_count_all_events_from_a_account_for_a_given_time_range**](docs/AccounteventApi.md#event_router_count_all_events_from_a_account_for_a_given_time_range) | **Get** /account/event/count | 
*AccounteventApi* | [**event_router_count_all_events_from_a_node_of_a_sub_account_for_a_given_time_range**](docs/AccounteventApi.md#event_router_count_all_events_from_a_node_of_a_sub_account_for_a_given_time_range) | **Get** /account/event/node/count | 
*AccounteventApi* | [**event_router_get**](docs/AccounteventApi.md#event_router_get) | **Get** /account/event/{eventId} | 
*AccounteventApi* | [**event_router_get_all_event_timestamp_keys_of_a_sub_account_from_a_specific_node_for_a_given_time_range**](docs/AccounteventApi.md#event_router_get_all_event_timestamp_keys_of_a_sub_account_from_a_specific_node_for_a_given_time_range) | **Get** /account/event/node/timestampkeys | 
*AccounteventApi* | [**event_router_get_all_events_of_a_account_from_a_specific_node**](docs/AccounteventApi.md#event_router_get_all_events_of_a_account_from_a_specific_node) | **Post** /account/event/node | 
*AccounteventApi* | [**event_router_get_event_in_node**](docs/AccounteventApi.md#event_router_get_event_in_node) | **Get** /account/event/node/{eventId} | 
*AccountintegrationApi* | [**account_integration_router_count**](docs/AccountintegrationApi.md#account_integration_router_count) | **Get** /account/integration/count | 
*AccountintegrationApi* | [**account_integration_router_create**](docs/AccountintegrationApi.md#account_integration_router_create) | **Post** /account/integration/{itype} | 
*AccountintegrationApi* | [**account_integration_router_delete**](docs/AccountintegrationApi.md#account_integration_router_delete) | **Delete** /account/integration/{itype} | 
*AccountintegrationApi* | [**account_integration_router_disable_glockapps_ip_monitoring**](docs/AccountintegrationApi.md#account_integration_router_disable_glockapps_ip_monitoring) | **Delete** /account/integration/glockapps/monitor/{ipid} | 
*AccountintegrationApi* | [**account_integration_router_enable_glockapps_ip_monitoring**](docs/AccountintegrationApi.md#account_integration_router_enable_glockapps_ip_monitoring) | **Post** /account/integration/glockapps/monitor/{ipid} | 
*AccountintegrationApi* | [**account_integration_router_get_all**](docs/AccountintegrationApi.md#account_integration_router_get_all) | **Get** /account/integration/ | 
*AccountintegrationApi* | [**account_integration_router_get_monitored_ip_stats**](docs/AccountintegrationApi.md#account_integration_router_get_monitored_ip_stats) | **Get** /account/integration/glockapps/monitor/stat/{ipid} | 
*AccountintegrationApi* | [**account_integration_router_update**](docs/AccountintegrationApi.md#account_integration_router_update) | **Put** /account/integration/{itype} | 
*AccountipApi* | [**i_p_router_allocate_ip**](docs/AccountipApi.md#i_p_router_allocate_ip) | **Post** /account/ip/allocate | 
*AccountipApi* | [**i_p_router_count**](docs/AccountipApi.md#i_p_router_count) | **Get** /account/ip/count | 
*AccountipApi* | [**i_p_router_delete**](docs/AccountipApi.md#i_p_router_delete) | **Delete** /account/ip/{ipid} | 
*AccountipApi* | [**i_p_router_get**](docs/AccountipApi.md#i_p_router_get) | **Get** /account/ip/{ipid} | 
*AccountipApi* | [**i_p_router_get_all**](docs/AccountipApi.md#i_p_router_get_all) | **Get** /account/ip/ | 
*AccountipApi* | [**i_p_router_update**](docs/AccountipApi.md#i_p_router_update) | **Put** /account/ip/{ipid} | 
*AccountippoolApi* | [**account_ip_pool_router_count**](docs/AccountippoolApi.md#account_ip_pool_router_count) | **Get** /account/ippool/count | 
*AccountippoolApi* | [**account_ip_pool_router_create**](docs/AccountippoolApi.md#account_ip_pool_router_create) | **Post** /account/ippool/ | 
*AccountippoolApi* | [**account_ip_pool_router_delete**](docs/AccountippoolApi.md#account_ip_pool_router_delete) | **Delete** /account/ippool/{ippoolid} | 
*AccountippoolApi* | [**account_ip_pool_router_get**](docs/AccountippoolApi.md#account_ip_pool_router_get) | **Get** /account/ippool/{ippoolid} | 
*AccountippoolApi* | [**account_ip_pool_router_get_all**](docs/AccountippoolApi.md#account_ip_pool_router_get_all) | **Get** /account/ippool/ | 
*AccountippoolApi* | [**account_ip_pool_router_update**](docs/AccountippoolApi.md#account_ip_pool_router_update) | **Put** /account/ippool/{ippoolid} | 
*AccountipstatApi* | [**i_p_stat_router_get_all_aggregate_ip_stats**](docs/AccountipstatApi.md#i_p_stat_router_get_all_aggregate_ip_stats) | **Get** /account/ip/stat/{ipid}/aggregate | 
*AccountipstatApi* | [**i_p_stat_router_get_all_aggregate_ip_stats_by_group**](docs/AccountipstatApi.md#i_p_stat_router_get_all_aggregate_ip_stats_by_group) | **Get** /account/ip/stat/{ipid}/aggregate/provider | 
*AccountipstatApi* | [**i_p_stat_router_get_all_aggregated_provider_stats_for_a_ip**](docs/AccountipstatApi.md#i_p_stat_router_get_all_aggregated_provider_stats_for_a_ip) | **Get** /account/ip/stat/{ipid}/aggregate/providers | 
*AccountipstatApi* | [**i_p_stat_router_get_all_aggregated_provider_stats_for_a_specific_sub_account_of_a_ip**](docs/AccountipstatApi.md#i_p_stat_router_get_all_aggregated_provider_stats_for_a_specific_sub_account_of_a_ip) | **Get** /account/ip/stat/{ipid}/aggregate/sid/{sid}/providers | 
*AccountipstatApi* | [**i_p_stat_router_get_all_aggregated_sub_account_stats_for_an_ip**](docs/AccountipstatApi.md#i_p_stat_router_get_all_aggregated_sub_account_stats_for_an_ip) | **Get** /account/ip/stat/{ipid}/aggregate/subaccounts | 
*AccountipstatApi* | [**i_p_stat_router_get_all_ip_stats**](docs/AccountipstatApi.md#i_p_stat_router_get_all_ip_stats) | **Get** /account/ip/stat/{ipid} | 
*AccountipstatApi* | [**i_p_stat_router_get_all_ip_stats_by_group**](docs/AccountipstatApi.md#i_p_stat_router_get_all_ip_stats_by_group) | **Get** /account/ip/stat/{ipid}/provider | 
*AccountlabelApi* | [**label_router_count**](docs/AccountlabelApi.md#label_router_count) | **Get** /account/label/count | 
*AccountlabelApi* | [**label_router_create**](docs/AccountlabelApi.md#label_router_create) | **Post** /account/label/ | 
*AccountlabelApi* | [**label_router_delete**](docs/AccountlabelApi.md#label_router_delete) | **Delete** /account/label/{labelId} | 
*AccountlabelApi* | [**label_router_get**](docs/AccountlabelApi.md#label_router_get) | **Get** /account/label/{labelId} | 
*AccountlabelApi* | [**label_router_get_all**](docs/AccountlabelApi.md#label_router_get_all) | **Get** /account/label/ | 
*AccountlabelApi* | [**label_router_update**](docs/AccountlabelApi.md#label_router_update) | **Put** /account/label/{labelId} | 
*AccountmemberApi* | [**member_router_delete**](docs/AccountmemberApi.md#member_router_delete) | **Delete** /account/member/{memberId} | 
*AccountmemberApi* | [**member_router_get**](docs/AccountmemberApi.md#member_router_get) | **Get** /account/member/{memberId} | 
*AccountmemberApi* | [**member_router_get_all**](docs/AccountmemberApi.md#member_router_get_all) | **Get** /account/member/ | 
*AccountmessageApi* | [**message_router_get**](docs/AccountmessageApi.md#message_router_get) | **Get** /account/message/{messageId} | 
*AccountmessageApi* | [**message_router_get_all_events_for_a_message_id**](docs/AccountmessageApi.md#message_router_get_all_events_for_a_message_id) | **Get** /account/message/{messageId}/events | 
*AccountmessageApi* | [**message_router_get_all_events_for_a_message_id_from_a_node**](docs/AccountmessageApi.md#message_router_get_all_events_for_a_message_id_from_a_node) | **Get** /account/message/node/{messageId}/events | 
*AccountmessageApi* | [**message_router_get_message_from_node**](docs/AccountmessageApi.md#message_router_get_message_from_node) | **Get** /account/message/node/{messageId} | 
*AccountrecipientApi* | [**recipient_router_get_all_messages_for_a_recipient**](docs/AccountrecipientApi.md#recipient_router_get_all_messages_for_a_recipient) | **Get** /account/recipient/{recipient}/messages | 
*AccountrecipientApi* | [**recipient_router_get_all_messages_for_a_recipient_from_a_node**](docs/AccountrecipientApi.md#recipient_router_get_all_messages_for_a_recipient_from_a_node) | **Get** /account/recipient/node/{recipient}/messages | 
*AccountsmtpstatApi* | [**s_mtp_stat_router_get_all_aggregate_ip_provider_smtp_stats**](docs/AccountsmtpstatApi.md#s_mtp_stat_router_get_all_aggregate_ip_provider_smtp_stats) | **Get** /account/smtp/stat/ip/{ipid}/provider/{pname}/aggregate | 
*AccountsmtpstatApi* | [**s_mtp_stat_router_get_all_aggregate_ip_smtp_stats**](docs/AccountsmtpstatApi.md#s_mtp_stat_router_get_all_aggregate_ip_smtp_stats) | **Get** /account/smtp/stat/ip/{ipid}/aggregate | 
*AccountsmtpstatApi* | [**s_mtp_stat_router_get_all_aggregate_ip_smtp_stats_for_sub_account**](docs/AccountsmtpstatApi.md#s_mtp_stat_router_get_all_aggregate_ip_smtp_stats_for_sub_account) | **Get** /account/smtp/stat/ip/{ipid}/subaccount/{sid}/aggregate | 
*AccountsmtpstatApi* | [**s_mtp_stat_router_get_all_aggregate_sub_account_provider_smtp_stats**](docs/AccountsmtpstatApi.md#s_mtp_stat_router_get_all_aggregate_sub_account_provider_smtp_stats) | **Get** /account/smtp/stat/subaccount/{sid}/provider/{pname}/aggregate | 
*AccountsmtpstatApi* | [**s_mtp_stat_router_get_all_aggregate_sub_account_smtp_stats**](docs/AccountsmtpstatApi.md#s_mtp_stat_router_get_all_aggregate_sub_account_smtp_stats) | **Get** /account/smtp/stat/subaccount/{sid}/aggregate | 
*AccountsmtpstatApi* | [**s_mtp_stat_router_get_all_aggregate_sub_account_smtp_stats_for_ip**](docs/AccountsmtpstatApi.md#s_mtp_stat_router_get_all_aggregate_sub_account_smtp_stats_for_ip) | **Get** /account/smtp/stat/subaccount/{sid}/ip/{ipid}/aggregate | 
*AccountstatApi* | [**account_stat_router_get_all_account_stats**](docs/AccountstatApi.md#account_stat_router_get_all_account_stats) | **Get** /account/stat/ | 
*AccountstatApi* | [**account_stat_router_get_all_account_stats_by_group**](docs/AccountstatApi.md#account_stat_router_get_all_account_stats_by_group) | **Get** /account/stat/group | 
*AccountstatApi* | [**account_stat_router_get_all_aggregate_account_stats**](docs/AccountstatApi.md#account_stat_router_get_all_aggregate_account_stats) | **Get** /account/stat/aggregate | 
*AccountstatApi* | [**account_stat_router_get_all_aggregate_account_stats_by_group**](docs/AccountstatApi.md#account_stat_router_get_all_aggregate_account_stats_by_group) | **Get** /account/stat/aggregate/group | 
*AccountsubaccountApi* | [**sub_account_router_count**](docs/AccountsubaccountApi.md#sub_account_router_count) | **Get** /account/subaccount/count | 
*AccountsubaccountApi* | [**sub_account_router_create**](docs/AccountsubaccountApi.md#sub_account_router_create) | **Post** /account/subaccount/ | 
*AccountsubaccountApi* | [**sub_account_router_delete**](docs/AccountsubaccountApi.md#sub_account_router_delete) | **Delete** /account/subaccount/{subAccountId} | 
*AccountsubaccountApi* | [**sub_account_router_get**](docs/AccountsubaccountApi.md#sub_account_router_get) | **Get** /account/subaccount/{subAccountId} | 
*AccountsubaccountApi* | [**sub_account_router_get_all**](docs/AccountsubaccountApi.md#sub_account_router_get_all) | **Get** /account/subaccount/ | 
*AccountsubaccountApi* | [**sub_account_router_update**](docs/AccountsubaccountApi.md#sub_account_router_update) | **Put** /account/subaccount/{subAccountId} | 
*AccountvalidationApi* | [**validate_router_validate_email_bulk**](docs/AccountvalidationApi.md#validate_router_validate_email_bulk) | **Post** /account/validation/bulk | 
*AccountvalidationApi* | [**validation_router_count**](docs/AccountvalidationApi.md#validation_router_count) | **Get** /account/validation/count | 
*AccountvalidationApi* | [**validation_router_delete_validation**](docs/AccountvalidationApi.md#validation_router_delete_validation) | **Delete** /account/validation/ | 
*AccountvalidationApi* | [**validation_router_get_all**](docs/AccountvalidationApi.md#validation_router_get_all) | **Get** /account/validation/ | 
*AccountvalidationApi* | [**validation_router_validate_email_list**](docs/AccountvalidationApi.md#validation_router_validate_email_list) | **Post** /account/validation/ | 
*AccountwebhookApi* | [**account_webhook_router_count**](docs/AccountwebhookApi.md#account_webhook_router_count) | **Get** /account/webhook/count | 
*AccountwebhookApi* | [**account_webhook_router_create**](docs/AccountwebhookApi.md#account_webhook_router_create) | **Post** /account/webhook/ | 
*AccountwebhookApi* | [**account_webhook_router_create_account_webhook_in_account_webhook_cache**](docs/AccountwebhookApi.md#account_webhook_router_create_account_webhook_in_account_webhook_cache) | **Post** /account/webhook/cache | 
*AccountwebhookApi* | [**account_webhook_router_delete**](docs/AccountwebhookApi.md#account_webhook_router_delete) | **Delete** /account/webhook/{webhookId} | 
*AccountwebhookApi* | [**account_webhook_router_delete_account_webhook_in_account_webhook_cache**](docs/AccountwebhookApi.md#account_webhook_router_delete_account_webhook_in_account_webhook_cache) | **Delete** /account/webhook/cache | 
*AccountwebhookApi* | [**account_webhook_router_get**](docs/AccountwebhookApi.md#account_webhook_router_get) | **Get** /account/webhook/{webhookId} | 
*AccountwebhookApi* | [**account_webhook_router_get_all**](docs/AccountwebhookApi.md#account_webhook_router_get_all) | **Get** /account/webhook/ | 
*AccountwebhookApi* | [**account_webhook_router_update**](docs/AccountwebhookApi.md#account_webhook_router_update) | **Put** /account/webhook/{webhookId} | 
*SmtpApi* | [**s_mtp_router_receive_webhooks_raised_from_smtp_servers**](docs/SmtpApi.md#s_mtp_router_receive_webhooks_raised_from_smtp_servers) | **Post** /smtp/webhook | 
*SubaccountdomainApi* | [**domain_router_count**](docs/SubaccountdomainApi.md#domain_router_count) | **Get** /subaccount/domain/count | 
*SubaccountdomainApi* | [**domain_router_create**](docs/SubaccountdomainApi.md#domain_router_create) | **Post** /subaccount/domain/ | 
*SubaccountdomainApi* | [**domain_router_delete**](docs/SubaccountdomainApi.md#domain_router_delete) | **Delete** /subaccount/domain/{domainId} | 
*SubaccountdomainApi* | [**domain_router_get**](docs/SubaccountdomainApi.md#domain_router_get) | **Get** /subaccount/domain/{domainId} | 
*SubaccountdomainApi* | [**domain_router_get_all**](docs/SubaccountdomainApi.md#domain_router_get_all) | **Get** /subaccount/domain/ | 
*SubaccountdomainApi* | [**domain_router_update**](docs/SubaccountdomainApi.md#domain_router_update) | **Put** /subaccount/domain/{domainId} | 
*SubaccountdomainApi* | [**domain_router_verify**](docs/SubaccountdomainApi.md#domain_router_verify) | **Post** /subaccount/domain/{domainId}/verify | 
*SubaccountemailApi* | [**email_router_send_email**](docs/SubaccountemailApi.md#email_router_send_email) | **Post** /subaccount/email/ | 
*SubaccountippoolApi* | [**i_p_pool_router_count**](docs/SubaccountippoolApi.md#i_p_pool_router_count) | **Get** /subaccount/ippool/count | 
*SubaccountippoolApi* | [**i_p_pool_router_create**](docs/SubaccountippoolApi.md#i_p_pool_router_create) | **Post** /subaccount/ippool/ | 
*SubaccountippoolApi* | [**i_p_pool_router_delete**](docs/SubaccountippoolApi.md#i_p_pool_router_delete) | **Delete** /subaccount/ippool/{ippoolid} | 
*SubaccountippoolApi* | [**i_p_pool_router_get**](docs/SubaccountippoolApi.md#i_p_pool_router_get) | **Get** /subaccount/ippool/{ippoolid} | 
*SubaccountippoolApi* | [**i_p_pool_router_get_all**](docs/SubaccountippoolApi.md#i_p_pool_router_get_all) | **Get** /subaccount/ippool/ | 
*SubaccountippoolApi* | [**i_p_pool_router_update**](docs/SubaccountippoolApi.md#i_p_pool_router_update) | **Put** /subaccount/ippool/{ippoolid} | 
*SubaccountsenderApi* | [**sender_router_count**](docs/SubaccountsenderApi.md#sender_router_count) | **Get** /subaccount/sender/count | 
*SubaccountsenderApi* | [**sender_router_create**](docs/SubaccountsenderApi.md#sender_router_create) | **Post** /subaccount/sender/ | 
*SubaccountsenderApi* | [**sender_router_delete**](docs/SubaccountsenderApi.md#sender_router_delete) | **Delete** /subaccount/sender/{senderId} | 
*SubaccountsenderApi* | [**sender_router_get**](docs/SubaccountsenderApi.md#sender_router_get) | **Get** /subaccount/sender/{senderId} | 
*SubaccountsenderApi* | [**sender_router_get_all**](docs/SubaccountsenderApi.md#sender_router_get_all) | **Get** /subaccount/sender/ | 
*SubaccountsenderApi* | [**sender_router_update**](docs/SubaccountsenderApi.md#sender_router_update) | **Put** /subaccount/sender/{senderId} | 
*SubaccountstatApi* | [**sub_account_stat_router_get_all_aggregate_sub_account_stats**](docs/SubaccountstatApi.md#sub_account_stat_router_get_all_aggregate_sub_account_stats) | **Get** /subaccount/stat/aggregate | 
*SubaccountstatApi* | [**sub_account_stat_router_get_all_aggregate_sub_account_stats_by_group**](docs/SubaccountstatApi.md#sub_account_stat_router_get_all_aggregate_sub_account_stats_by_group) | **Get** /subaccount/stat/aggregate/group | 
*SubaccountstatApi* | [**sub_account_stat_router_get_all_aggregated_group_stats_for_a_sub_account**](docs/SubaccountstatApi.md#sub_account_stat_router_get_all_aggregated_group_stats_for_a_sub_account) | **Get** /subaccount/stat/aggregate/groups | 
*SubaccountstatApi* | [**sub_account_stat_router_get_all_aggregated_ip_stats_for_a_sub_account**](docs/SubaccountstatApi.md#sub_account_stat_router_get_all_aggregated_ip_stats_for_a_sub_account) | **Get** /subaccount/stat/aggregate/ips | 
*SubaccountstatApi* | [**sub_account_stat_router_get_all_aggregated_provider_stats_for_a_specific_ip_of_a_sub_account**](docs/SubaccountstatApi.md#sub_account_stat_router_get_all_aggregated_provider_stats_for_a_specific_ip_of_a_sub_account) | **Get** /subaccount/stat/aggregate/ip/{ipid}/providers | 
*SubaccountstatApi* | [**sub_account_stat_router_get_all_aggregated_provider_stats_for_a_sub_account**](docs/SubaccountstatApi.md#sub_account_stat_router_get_all_aggregated_provider_stats_for_a_sub_account) | **Get** /subaccount/stat/aggregate/providers | 
*SubaccountstatApi* | [**sub_account_stat_router_get_all_sub_account_stats**](docs/SubaccountstatApi.md#sub_account_stat_router_get_all_sub_account_stats) | **Get** /subaccount/stat/ | 
*SubaccountstatApi* | [**sub_account_stat_router_get_all_sub_account_stats_by_group**](docs/SubaccountstatApi.md#sub_account_stat_router_get_all_sub_account_stats_by_group) | **Get** /subaccount/stat/group | 
*SubaccountsuppressionApi* | [**suppression_router_count**](docs/SubaccountsuppressionApi.md#suppression_router_count) | **Get** /subaccount/suppression/count | 
*SubaccountsuppressionApi* | [**suppression_router_create_suppressions**](docs/SubaccountsuppressionApi.md#suppression_router_create_suppressions) | **Post** /subaccount/suppression/ | 
*SubaccountsuppressionApi* | [**suppression_router_create_suppressions_in_suppression_filter**](docs/SubaccountsuppressionApi.md#suppression_router_create_suppressions_in_suppression_filter) | **Post** /subaccount/suppression/filter | 
*SubaccountsuppressionApi* | [**suppression_router_delete_suppression**](docs/SubaccountsuppressionApi.md#suppression_router_delete_suppression) | **Delete** /subaccount/suppression/ | 
*SubaccountsuppressionApi* | [**suppression_router_delete_suppressions_in_suppression_filter**](docs/SubaccountsuppressionApi.md#suppression_router_delete_suppressions_in_suppression_filter) | **Delete** /subaccount/suppression/filter | 
*SubaccountsuppressionApi* | [**suppression_router_get_all_suppressions**](docs/SubaccountsuppressionApi.md#suppression_router_get_all_suppressions) | **Get** /subaccount/suppression/ | 


## Documentation For Models

 - [Alert](docs/Alert.md)
 - [ModelsAccount](docs/ModelsAccount.md)
 - [ModelsAccountIpPool](docs/ModelsAccountIpPool.md)
 - [ModelsAccountWebhook](docs/ModelsAccountWebhook.md)
 - [ModelsAgStat](docs/ModelsAgStat.md)
 - [ModelsAipStat](docs/ModelsAipStat.md)
 - [ModelsAlertLabel](docs/ModelsAlertLabel.md)
 - [ModelsAlertRequest](docs/ModelsAlertRequest.md)
 - [ModelsAlertResponse](docs/ModelsAlertResponse.md)
 - [ModelsAuthInfo](docs/ModelsAuthInfo.md)
 - [ModelsBackOffConfiguration](docs/ModelsBackOffConfiguration.md)
 - [ModelsBackOffDecreaseType](docs/ModelsBackOffDecreaseType.md)
 - [ModelsBackOffTrigger](docs/ModelsBackOffTrigger.md)
 - [ModelsBlacklistStatus](docs/ModelsBlacklistStatus.md)
 - [ModelsBulkResponse](docs/ModelsBulkResponse.md)
 - [ModelsCity](docs/ModelsCity.md)
 - [ModelsCleanedList](docs/ModelsCleanedList.md)
 - [ModelsCountStat](docs/ModelsCountStat.md)
 - [ModelsDeleteResponse](docs/ModelsDeleteResponse.md)
 - [ModelsDetailedAlert](docs/ModelsDetailedAlert.md)
 - [ModelsDnsRecord](docs/ModelsDnsRecord.md)
 - [ModelsDomain](docs/ModelsDomain.md)
 - [ModelsEAccount](docs/ModelsEAccount.md)
 - [ModelsEAccountMember](docs/ModelsEAccountMember.md)
 - [ModelsEDomain](docs/ModelsEDomain.md)
 - [ModelsEIntegration](docs/ModelsEIntegration.md)
 - [ModelsEInvitation](docs/ModelsEInvitation.md)
 - [ModelsESender](docs/ModelsESender.md)
 - [ModelsESubAccount](docs/ModelsESubAccount.md)
 - [ModelsEValidation](docs/ModelsEValidation.md)
 - [ModelsEWebhook](docs/ModelsEWebhook.md)
 - [ModelsEip](docs/ModelsEip.md)
 - [ModelsEipPool](docs/ModelsEipPool.md)
 - [ModelsEmailErrorCode](docs/ModelsEmailErrorCode.md)
 - [ModelsEmailList](docs/ModelsEmailList.md)
 - [ModelsEmailMessage](docs/ModelsEmailMessage.md)
 - [ModelsEmailResponse](docs/ModelsEmailResponse.md)
 - [ModelsEventMetadata](docs/ModelsEventMetadata.md)
 - [ModelsEventType](docs/ModelsEventType.md)
 - [ModelsFrequencyType](docs/ModelsFrequencyType.md)
 - [ModelsFrom](docs/ModelsFrom.md)
 - [ModelsGlockappsBlacklist](docs/ModelsGlockappsBlacklist.md)
 - [ModelsGlockappsMonitorStat](docs/ModelsGlockappsMonitorStat.md)
 - [ModelsIip](docs/ModelsIip.md)
 - [ModelsInstance](docs/ModelsInstance.md)
 - [ModelsIntegration](docs/ModelsIntegration.md)
 - [ModelsIntegrationSettings](docs/ModelsIntegrationSettings.md)
 - [ModelsIntegrationType](docs/ModelsIntegrationType.md)
 - [ModelsInvitation](docs/ModelsInvitation.md)
 - [ModelsInvitationStatus](docs/ModelsInvitationStatus.md)
 - [ModelsIp](docs/ModelsIp.md)
 - [ModelsIpPool](docs/ModelsIpPool.md)
 - [ModelsIpPoolType](docs/ModelsIpPoolType.md)
 - [ModelsIpStat](docs/ModelsIpStat.md)
 - [ModelsIpType](docs/ModelsIpType.md)
 - [ModelsLabel](docs/ModelsLabel.md)
 - [ModelsMember](docs/ModelsMember.md)
 - [ModelsMemberRole](docs/ModelsMemberRole.md)
 - [ModelsNotificationType](docs/ModelsNotificationType.md)
 - [ModelsPipStat](docs/ModelsPipStat.md)
 - [ModelsQEmailMessage](docs/ModelsQEmailMessage.md)
 - [ModelsQEvent](docs/ModelsQEvent.md)
 - [ModelsRGlockappsMonitorStat](docs/ModelsRGlockappsMonitorStat.md)
 - [ModelsRStat](docs/ModelsRStat.md)
 - [ModelsRSuppression](docs/ModelsRSuppression.md)
 - [ModelsRdSuppression](docs/ModelsRdSuppression.md)
 - [ModelsReplyTo](docs/ModelsReplyTo.md)
 - [ModelsResponse](docs/ModelsResponse.md)
 - [ModelsRipStat](docs/ModelsRipStat.md)
 - [ModelsSender](docs/ModelsSender.md)
 - [ModelsSingleCleanedMail](docs/ModelsSingleCleanedMail.md)
 - [ModelsSipStat](docs/ModelsSipStat.md)
 - [ModelsSmtpAuth](docs/ModelsSmtpAuth.md)
 - [ModelsSmtpStat](docs/ModelsSmtpStat.md)
 - [ModelsStat](docs/ModelsStat.md)
 - [ModelsSubAccount](docs/ModelsSubAccount.md)
 - [ModelsSubAccountType](docs/ModelsSubAccountType.md)
 - [ModelsSuppression](docs/ModelsSuppression.md)
 - [ModelsSuppressionEmail](docs/ModelsSuppressionEmail.md)
 - [ModelsSuppressionReason](docs/ModelsSuppressionReason.md)
 - [ModelsSystemDnsRecord](docs/ModelsSystemDnsRecord.md)
 - [ModelsSystemDomain](docs/ModelsSystemDomain.md)
 - [ModelsTo](docs/ModelsTo.md)
 - [ModelsValidation](docs/ModelsValidation.md)
 - [ModelsValidationReason](docs/ModelsValidationReason.md)
 - [ModelsWMessage](docs/ModelsWMessage.md)
 - [UaparserDevice](docs/UaparserDevice.md)
 - [UaparserOs](docs/UaparserOs.md)
 - [UaparserUserAgent](docs/UaparserUserAgent.md)


## Documentation For Authorization
 Endpoints do not require authorization.


## Author

hello@sendx.io


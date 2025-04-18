// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_get_conversation_output_output_next_token(
    input: &crate::operation::get_conversation::GetConversationOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_conversations_output_output_next_token(
    input: &crate::operation::list_conversations::ListConversationsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_dashboard_metrics_output_output_next_token(
    input: &crate::operation::list_dashboard_metrics::ListDashboardMetricsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_extension_providers_output_output_next_token(
    input: &crate::operation::list_extension_providers::ListExtensionProvidersOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_extensions_output_output_next_token(
    input: &crate::operation::list_extensions::ListExtensionsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_plugin_providers_output_output_next_token(
    input: &crate::operation::list_plugin_providers::ListPluginProvidersOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_plugins_output_output_next_token(
    input: &crate::operation::list_plugins::ListPluginsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_tasks_output_output_next_token(
    input: &crate::operation::list_tasks::ListTasksOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_get_conversation_output_output_messages(
    input: crate::operation::get_conversation::GetConversationOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Message>> {
    let input = input.messages;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_conversations_output_output_conversations(
    input: crate::operation::list_conversations::ListConversationsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::ConversationMetadata>> {
    let input = input.conversations;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_dashboard_metrics_output_output_metrics(
    input: crate::operation::list_dashboard_metrics::ListDashboardMetricsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::DashboardMetric>> {
    let input = input.metrics;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_extension_providers_output_output_extension_providers(
    input: crate::operation::list_extension_providers::ListExtensionProvidersOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::ExtensionProviderMetadata>> {
    let input = input.extension_providers;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_extensions_output_output_extensions(
    input: crate::operation::list_extensions::ListExtensionsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Extension>> {
    let input = input.extensions;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_plugin_providers_output_output_plugin_providers(
    input: crate::operation::list_plugin_providers::ListPluginProvidersOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::PluginProviderMetadata>> {
    let input = input.plugin_providers;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_plugins_output_output_plugins(
    input: crate::operation::list_plugins::ListPluginsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Plugin>> {
    let input = input.plugins;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_tasks_output_output_tasks(
    input: crate::operation::list_tasks::ListTasksOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::TaskSummary>> {
    let input = input.tasks;
    ::std::option::Option::Some(input)
}

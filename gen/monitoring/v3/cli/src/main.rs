use clap::{App, AppSettings, Arg, SubCommand};
use default_boxed::DefaultBoxed;

#[derive(DefaultBoxed)]
struct Outer<'a, 'b> {
    inner: HeapApp<'a, 'b>,
}

struct HeapApp<'a, 'b> {
    app: App<'a, 'b>,
}

impl<'a, 'b> Default for HeapApp<'a, 'b> {
    fn default() -> Self {
        let mut app = App::new("monitoring3")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210316")
            .about("Manages your Cloud Monitoring data and configurations. Most projects must be associated with a Workspace, with a few exceptions as noted on the individual method pages. The table entries below are presented in alphabetical order, not in order of common use. For explanations of the concepts found in the table entries, read the Cloud Monitoring documentation.")
            .after_help("All documentation details can be found at <TODO figure out URL>")
            .arg(Arg::with_name("scope")
                .long("scope")
                .help("Specify the authentication method should be executed in. Each scope requires the user to grant this application permission to use it. If unset, it defaults to the shortest scope url for a particular method.")
                .multiple(true)
                .takes_value(true))
            .arg(Arg::with_name("folder")
                .long("config-dir")
                .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation." )
                .multiple(false)
                .takes_value(true))
            .arg(Arg::with_name("debug")
                .long("debug")
                .help("Provide more output to aid with debugging")
                .multiple(false)
                .takes_value(false));
        let mut folders0 = SubCommand::with_name("folders")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: time_series");
        let mut organizations0 = SubCommand::with_name("organizations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: time_series");
        let mut projects0 = SubCommand::with_name("projects")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: alert_policies, collectd_time_series, groups, metric_descriptors, monitored_resource_descriptors, notification_channel_descriptors, notification_channels, time_series and uptime_check_configs");
        let mut services0 = SubCommand::with_name("services")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Create a Service.");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Soft delete this Service.");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get the named Service.");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List Services for this workspace.");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update this Service.");
            services0 = services0.subcommand(mcmd);
        }
        let mut uptime_check_ips0 = SubCommand::with_name("uptime_check_ips")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of IP addresses that checkers run from");
            uptime_check_ips0 = uptime_check_ips0.subcommand(mcmd);
        }
        let mut time_series1 = SubCommand::with_name("time_series")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists time series that match a filter. This method does not require a Workspace.",
            );
            time_series1 = time_series1.subcommand(mcmd);
        }
        let mut time_series1 = SubCommand::with_name("time_series")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists time series that match a filter. This method does not require a Workspace.",
            );
            time_series1 = time_series1.subcommand(mcmd);
        }
        let mut alert_policies1 = SubCommand::with_name("alert_policies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new alerting policy.");
            alert_policies1 = alert_policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an alerting policy.");
            alert_policies1 = alert_policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single alerting policy.");
            alert_policies1 = alert_policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the existing alerting policies for the workspace.");
            alert_policies1 = alert_policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an alerting policy. You can either replace the entire policy with a new one or replace only certain fields in the current alerting policy by specifying the fields to be updated via updateMask. Returns the updated alerting policy.");
            alert_policies1 = alert_policies1.subcommand(mcmd);
        }
        let mut collectd_time_series1 = SubCommand::with_name("collectd_time_series")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create");
        {
            let mcmd = SubCommand::with_name("create").about("Stackdriver Monitoring Agent only: Creates a new time series.This method is only for use by the Stackdriver Monitoring Agent. Use projects.timeSeries.create instead.");
            collectd_time_series1 = collectd_time_series1.subcommand(mcmd);
        }
        let mut groups1 = SubCommand::with_name("groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new group.");
            groups1 = groups1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an existing group.");
            groups1 = groups1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single group.");
            groups1 = groups1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the existing groups.");
            groups1 = groups1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about(
                "Updates an existing group. You can change any group attributes except name.",
            );
            groups1 = groups1.subcommand(mcmd);
        }
        let mut metric_descriptors1 = SubCommand::with_name("metric_descriptors")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new metric descriptor. User-created metric descriptors define custom metrics (https://cloud.google.com/monitoring/custom-metrics).");
            metric_descriptors1 = metric_descriptors1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a metric descriptor. Only user-created custom metrics (https://cloud.google.com/monitoring/custom-metrics) can be deleted.");
            metric_descriptors1 = metric_descriptors1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Gets a single metric descriptor. This method does not require a Workspace.",
            );
            metric_descriptors1 = metric_descriptors1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists metric descriptors that match a filter. This method does not require a Workspace.");
            metric_descriptors1 = metric_descriptors1.subcommand(mcmd);
        }
        let mut monitored_resource_descriptors1 =
            SubCommand::with_name("monitored_resource_descriptors")
                .setting(AppSettings::ColoredHelp)
                .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single monitored resource descriptor. This method does not require a Workspace.");
            monitored_resource_descriptors1 = monitored_resource_descriptors1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists monitored resource descriptors that match a filter. This method does not require a Workspace.");
            monitored_resource_descriptors1 = monitored_resource_descriptors1.subcommand(mcmd);
        }
        let mut notification_channel_descriptors1 =
            SubCommand::with_name("notification_channel_descriptors")
                .setting(AppSettings::ColoredHelp)
                .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single channel descriptor. The descriptor indicates which fields are expected / permitted for a notification channel of the given type.");
            notification_channel_descriptors1 = notification_channel_descriptors1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the descriptors for supported channel types. The use of descriptors makes it possible for new channel types to be dynamically added.");
            notification_channel_descriptors1 = notification_channel_descriptors1.subcommand(mcmd);
        }
        let mut notification_channels1 = SubCommand::with_name("notification_channels")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_verification_code, list, patch, send_verification_code and verify");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new notification channel, representing a single notification endpoint such as an email address, SMS number, or PagerDuty service.");
            notification_channels1 = notification_channels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a notification channel.");
            notification_channels1 = notification_channels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single notification channel. The channel includes the relevant configuration details with which the channel was created. However, the response may truncate or omit passwords, API keys, or other private key matter and thus the response may not be 100% identical to the information that was supplied in the call to the create method.");
            notification_channels1 = notification_channels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_verification_code").about("Requests a verification code for an already verified channel that can then be used in a call to VerifyNotificationChannel() on a different channel with an equivalent identity in the same or in a different project. This makes it possible to copy a channel between projects without requiring manual reverification of the channel. If the channel is not in the verified state, this method will fail (in other words, this may only be used if the SendNotificationChannelVerificationCode and VerifyNotificationChannel paths have already been used to put the given channel into the verified state).There is no guarantee that the verification codes returned by this method will be of a similar structure or form as the ones that are delivered to the channel via SendNotificationChannelVerificationCode; while VerifyNotificationChannel() will recognize both the codes delivered via SendNotificationChannelVerificationCode() and returned from GetNotificationChannelVerificationCode(), it is typically the case that the verification codes delivered via SendNotificationChannelVerificationCode() will be shorter and also have a shorter expiration (e.g. codes such as \"G-123456\") whereas GetVerificationCode() will typically return a much longer, websafe base 64 encoded string that has a longer expiration time.");
            notification_channels1 = notification_channels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the notification channels that have been created for the project.");
            notification_channels1 = notification_channels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a notification channel. Fields not specified in the field mask remain unchanged.");
            notification_channels1 = notification_channels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("send_verification_code").about("Causes a verification code to be delivered to the channel. The code can then be supplied in VerifyNotificationChannel to verify the channel.");
            notification_channels1 = notification_channels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("verify").about("Verifies a NotificationChannel by proving receipt of the code delivered to the channel as a result of calling SendNotificationChannelVerificationCode.");
            notification_channels1 = notification_channels1.subcommand(mcmd);
        }
        let mut time_series1 = SubCommand::with_name("time_series")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, list and query");
        {
            let mcmd = SubCommand::with_name("create").about("Creates or adds data to one or more time series. The response is empty if all time series in the request were written. If any time series could not be written, a corresponding failure message is included in the error response.");
            time_series1 = time_series1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists time series that match a filter. This method does not require a Workspace.",
            );
            time_series1 = time_series1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("query").about("Queries time series using Monitoring Query Language. This method does not require a Workspace.");
            time_series1 = time_series1.subcommand(mcmd);
        }
        let mut uptime_check_configs1 = SubCommand::with_name("uptime_check_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a new Uptime check configuration.");
            uptime_check_configs1 = uptime_check_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an Uptime check configuration. Note that this method will fail if the Uptime check configuration is referenced by an alert policy or other dependent configs that would be rendered invalid by the deletion.");
            uptime_check_configs1 = uptime_check_configs1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets a single Uptime check configuration.");
            uptime_check_configs1 = uptime_check_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the existing valid Uptime check configurations for the project (leaving out any invalid configurations).");
            uptime_check_configs1 = uptime_check_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an Uptime check configuration. You can either replace the entire configuration with a new one or replace only certain fields in the current configuration by specifying the fields to be updated via updateMask. Returns the updated configuration.");
            uptime_check_configs1 = uptime_check_configs1.subcommand(mcmd);
        }
        let mut service_level_objectives1 = SubCommand::with_name("service_level_objectives")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Create a ServiceLevelObjective for the given Service.");
            service_level_objectives1 = service_level_objectives1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Delete the given ServiceLevelObjective.");
            service_level_objectives1 = service_level_objectives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get a ServiceLevelObjective by name.");
            service_level_objectives1 = service_level_objectives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List the ServiceLevelObjectives for the given Service.");
            service_level_objectives1 = service_level_objectives1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Update the given ServiceLevelObjective.");
            service_level_objectives1 = service_level_objectives1.subcommand(mcmd);
        }
        let mut members2 = SubCommand::with_name("members")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the monitored resources that are members of a group.");
            members2 = members2.subcommand(mcmd);
        }
        groups1 = groups1.subcommand(members2);
        services0 = services0.subcommand(service_level_objectives1);
        projects0 = projects0.subcommand(uptime_check_configs1);
        projects0 = projects0.subcommand(time_series1);
        projects0 = projects0.subcommand(notification_channels1);
        projects0 = projects0.subcommand(notification_channel_descriptors1);
        projects0 = projects0.subcommand(monitored_resource_descriptors1);
        projects0 = projects0.subcommand(metric_descriptors1);
        projects0 = projects0.subcommand(groups1);
        projects0 = projects0.subcommand(collectd_time_series1);
        projects0 = projects0.subcommand(alert_policies1);
        organizations0 = organizations0.subcommand(time_series1);
        folders0 = folders0.subcommand(time_series1);
        app = app.subcommand(uptime_check_ips0);
        app = app.subcommand(services0);
        app = app.subcommand(projects0);
        app = app.subcommand(organizations0);
        app = app.subcommand(folders0);

        Self { app }
    }
}
use google_monitoring3 as api;

fn main() {
    // TODO: set homedir afterwards, once the address is unmovable, or use Pin for the very first time
    // to allow a self-referential structure :D!
    let _home_dir = dirs::config_dir()
        .expect("configuration directory can be obtained")
        .join("google-service-cli");
    let outer = Outer::default_boxed();
    let app = outer.inner.app;
    let _matches = app.get_matches();
}

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
        let mut app = App::new("securitycenter1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210306")
            .about("Security Command Center API provides access to temporal views of assets and findings within an organization.")
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
            .about("sub-resources: assets and sources");
        let mut organizations0 = SubCommand::with_name("organizations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_organization_settings and update_organization_settings");
        {
            let mcmd = SubCommand::with_name("get_organization_settings")
                .about("Gets the settings for an organization.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_organization_settings")
                .about("Updates an organization\'s settings.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: assets and sources");
        let mut assets1 = SubCommand::with_name("assets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: group, list and update_security_marks");
        {
            let mcmd = SubCommand::with_name("group").about(
                "Filters an organization\'s assets and groups them by their specified properties.",
            );
            assets1 = assets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists an organization\'s assets.");
            assets1 = assets1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update_security_marks").about("Updates security marks.");
            assets1 = assets1.subcommand(mcmd);
        }
        let mut sources1 = SubCommand::with_name("sources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all sources belonging to an organization.");
            sources1 = sources1.subcommand(mcmd);
        }
        let mut assets1 = SubCommand::with_name("assets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: group, list, run_discovery and update_security_marks");
        {
            let mcmd = SubCommand::with_name("group").about(
                "Filters an organization\'s assets and groups them by their specified properties.",
            );
            assets1 = assets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists an organization\'s assets.");
            assets1 = assets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("run_discovery").about("Runs asset discovery. The discovery is tracked with a long-running operation. This API can only be called with limited frequency for an organization. If it is called too frequently the caller will receive a TOO_MANY_REQUESTS error.");
            assets1 = assets1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update_security_marks").about("Updates security marks.");
            assets1 = assets1.subcommand(mcmd);
        }
        let mut notification_configs1 = SubCommand::with_name("notification_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a notification config.");
            notification_configs1 = notification_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a notification config.");
            notification_configs1 = notification_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a notification config.");
            notification_configs1 = notification_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists notification configs.");
            notification_configs1 = notification_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(" Updates a notification config. The following update fields are allowed: description, pubsub_topic, streaming_config.filter");
            notification_configs1 = notification_configs1.subcommand(mcmd);
        }
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut sources1 = SubCommand::with_name("sources")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a source.");
            sources1 = sources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a source.");
            sources1 = sources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy")
                .about("Gets the access control policy on the specified Source.");
            sources1 = sources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all sources belonging to an organization.");
            sources1 = sources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a source.");
            sources1 = sources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy")
                .about("Sets the access control policy on the specified Source.");
            sources1 = sources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns the permissions that a caller has on the specified source.");
            sources1 = sources1.subcommand(mcmd);
        }
        let mut assets1 = SubCommand::with_name("assets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: group, list and update_security_marks");
        {
            let mcmd = SubCommand::with_name("group").about(
                "Filters an organization\'s assets and groups them by their specified properties.",
            );
            assets1 = assets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists an organization\'s assets.");
            assets1 = assets1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update_security_marks").about("Updates security marks.");
            assets1 = assets1.subcommand(mcmd);
        }
        let mut sources1 = SubCommand::with_name("sources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all sources belonging to an organization.");
            sources1 = sources1.subcommand(mcmd);
        }
        let mut findings2 = SubCommand::with_name("findings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: group, list, patch, set_state and update_security_marks");
        {
            let mcmd = SubCommand::with_name("group").about("Filters an organization or source\'s findings and groups them by their specified properties. To group across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings, /v1/folders/{folder_id}/sources/-/findings, /v1/projects/{project_id}/sources/-/findings");
            findings2 = findings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists an organization or source\'s findings. To list across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings");
            findings2 = findings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Creates or updates a finding. The corresponding source must exist for a finding creation to succeed.");
            findings2 = findings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_state").about("Updates the state of a finding.");
            findings2 = findings2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update_security_marks").about("Updates security marks.");
            findings2 = findings2.subcommand(mcmd);
        }
        let mut findings2 = SubCommand::with_name("findings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, group, list, patch, set_state and update_security_marks");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a finding. The corresponding source must exist for finding creation to succeed.");
            findings2 = findings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("group").about("Filters an organization or source\'s findings and groups them by their specified properties. To group across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings, /v1/folders/{folder_id}/sources/-/findings, /v1/projects/{project_id}/sources/-/findings");
            findings2 = findings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists an organization or source\'s findings. To list across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings");
            findings2 = findings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Creates or updates a finding. The corresponding source must exist for a finding creation to succeed.");
            findings2 = findings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_state").about("Updates the state of a finding.");
            findings2 = findings2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update_security_marks").about("Updates security marks.");
            findings2 = findings2.subcommand(mcmd);
        }
        let mut findings2 = SubCommand::with_name("findings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: group, list, patch, set_state and update_security_marks");
        {
            let mcmd = SubCommand::with_name("group").about("Filters an organization or source\'s findings and groups them by their specified properties. To group across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings, /v1/folders/{folder_id}/sources/-/findings, /v1/projects/{project_id}/sources/-/findings");
            findings2 = findings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists an organization or source\'s findings. To list across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings");
            findings2 = findings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Creates or updates a finding. The corresponding source must exist for a finding creation to succeed.");
            findings2 = findings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_state").about("Updates the state of a finding.");
            findings2 = findings2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update_security_marks").about("Updates security marks.");
            findings2 = findings2.subcommand(mcmd);
        }
        sources1 = sources1.subcommand(findings2);
        sources1 = sources1.subcommand(findings2);
        sources1 = sources1.subcommand(findings2);
        projects0 = projects0.subcommand(sources1);
        projects0 = projects0.subcommand(assets1);
        organizations0 = organizations0.subcommand(sources1);
        organizations0 = organizations0.subcommand(operations1);
        organizations0 = organizations0.subcommand(notification_configs1);
        organizations0 = organizations0.subcommand(assets1);
        folders0 = folders0.subcommand(sources1);
        folders0 = folders0.subcommand(assets1);
        app = app.subcommand(projects0);
        app = app.subcommand(organizations0);
        app = app.subcommand(folders0);

        Self { app }
    }
}
use google_securitycenter1 as api;

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

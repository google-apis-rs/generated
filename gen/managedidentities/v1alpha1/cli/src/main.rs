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
        let mut app = App::new("managedidentities1_alpha1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210305")
            .about("The Managed Service for Microsoft Active Directory API is used for managing a highly available, hardened service running Microsoft Active Directory (AD).")
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
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut global2 = SubCommand::with_name("global")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: domains, operations and peerings");
        let mut domains3 = SubCommand::with_name("domains")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: attach_trust, create, delete, detach_trust, get, get_iam_policy, list, patch, reconfigure_trust, reset_admin_password, set_iam_policy, test_iam_permissions and validate_trust");
        {
            let mcmd = SubCommand::with_name("attach_trust")
                .about("Adds AD trust in a given domain. Operation");
            domains3 = domains3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a Microsoft AD Domain in a given project. Operation");
            domains3 = domains3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes identified domain. Operation");
            domains3 = domains3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("detach_trust").about("Removes identified trust. Operation");
            domains3 = domains3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details of a single Domain.");
            domains3 = domains3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            domains3 = domains3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Domains in a given project.");
            domains3 = domains3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates the metadata and configuration of a specified domain. Operation");
            domains3 = domains3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reconfigure_trust")
                .about("Updates the dns conditional forwarder. Operation");
            domains3 = domains3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset_admin_password").about("Resets managed identities admin password identified by managed_identities_admin_name");
            domains3 = domains3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            domains3 = domains3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            domains3 = domains3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("validate_trust").about("Validate the trust state Operation");
            domains3 = domains3.subcommand(mcmd);
        }
        let mut operations3 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations3 = operations3.subcommand(mcmd);
        }
        let mut peerings3 = SubCommand::with_name("peerings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_iam_policy, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            peerings3 = peerings3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            peerings3 = peerings3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            peerings3 = peerings3.subcommand(mcmd);
        }
        let mut sql_integrations4 = SubCommand::with_name("sql_integrations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets details of a single sqlIntegration.");
            sql_integrations4 = sql_integrations4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists SQLIntegrations in a given domain.");
            sql_integrations4 = sql_integrations4.subcommand(mcmd);
        }
        domains3 = domains3.subcommand(sql_integrations4);
        global2 = global2.subcommand(peerings3);
        global2 = global2.subcommand(operations3);
        global2 = global2.subcommand(domains3);
        locations1 = locations1.subcommand(global2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_managedidentities1_alpha1 as api;

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

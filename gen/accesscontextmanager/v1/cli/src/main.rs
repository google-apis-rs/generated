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
        let mut app = App::new("accesscontextmanager1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210314")
            .about("An API for setting attribute based access control to requests to GCP services.")
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
        let mut access_policies0 = SubCommand::with_name("access_policies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Create an `AccessPolicy`. Fails if this organization already has a `AccessPolicy`. The longrunning Operation will have a successful status once the `AccessPolicy` has propagated to long-lasting storage. Syntactic and basic semantic errors will be returned in `metadata` as a BadRequest proto.");
            access_policies0 = access_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete an AccessPolicy by resource name. The longrunning Operation will have a successful status once the AccessPolicy has been removed from long-lasting storage.");
            access_policies0 = access_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get an AccessPolicy by name.");
            access_policies0 = access_policies0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("List all AccessPolicies under a container.");
            access_policies0 = access_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update an AccessPolicy. The longrunning Operation from this RPC will have a successful status once the changes to the AccessPolicy have propagated to long-lasting storage. Syntactic and basic semantic errors will be returned in `metadata` as a BadRequest proto.");
            access_policies0 = access_policies0.subcommand(mcmd);
        }
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut organizations0 = SubCommand::with_name("organizations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: gcp_user_access_bindings");
        let mut access_levels1 = SubCommand::with_name("access_levels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and replace_all");
        {
            let mcmd = SubCommand::with_name("create").about("Create an Access Level. The longrunning operation from this RPC will have a successful status once the Access Level has propagated to long-lasting storage. Access Levels containing errors will result in an error response for the first error encountered.");
            access_levels1 = access_levels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete an Access Level by resource name. The longrunning operation from this RPC will have a successful status once the Access Level has been removed from long-lasting storage.");
            access_levels1 = access_levels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get an Access Level by resource name.");
            access_levels1 = access_levels1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("List all Access Levels for an access policy.");
            access_levels1 = access_levels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update an Access Level. The longrunning operation from this RPC will have a successful status once the changes to the Access Level have propagated to long-lasting storage. Access Levels containing errors will result in an error response for the first error encountered.");
            access_levels1 = access_levels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_all").about("Replace all existing Access Levels in an Access Policy with the Access Levels provided. This is done atomically. The longrunning operation from this RPC will have a successful status once all replacements have propagated to long-lasting storage. Replacements containing errors will result in an error response for the first error encountered. Replacement will be cancelled on error, existing Access Levels will not be affected. Operation.response field will contain ReplaceAccessLevelsResponse. Removing Access Levels contained in existing Service Perimeters will result in error.");
            access_levels1 = access_levels1.subcommand(mcmd);
        }
        let mut service_perimeters1 = SubCommand::with_name("service_perimeters")
            .setting(AppSettings::ColoredHelp)
            .about("methods: commit, create, delete, get, list, patch and replace_all");
        {
            let mcmd = SubCommand::with_name("commit").about("Commit the dry-run spec for all the Service Perimeters in an Access Policy. A commit operation on a Service Perimeter involves copying its `spec` field to that Service Perimeter\'s `status` field. Only Service Perimeters with `use_explicit_dry_run_spec` field set to true are affected by a commit operation. The longrunning operation from this RPC will have a successful status once the dry-run specs for all the Service Perimeters have been committed. If a commit fails, it will cause the longrunning operation to return an error response and the entire commit operation will be cancelled. When successful, Operation.response field will contain CommitServicePerimetersResponse. The `dry_run` and the `spec` fields will be cleared after a successful commit operation.");
            service_perimeters1 = service_perimeters1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Create a Service Perimeter. The longrunning operation from this RPC will have a successful status once the Service Perimeter has propagated to long-lasting storage. Service Perimeters containing errors will result in an error response for the first error encountered.");
            service_perimeters1 = service_perimeters1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a Service Perimeter by resource name. The longrunning operation from this RPC will have a successful status once the Service Perimeter has been removed from long-lasting storage.");
            service_perimeters1 = service_perimeters1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Get a Service Perimeter by resource name.");
            service_perimeters1 = service_perimeters1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all Service Perimeters for an access policy.");
            service_perimeters1 = service_perimeters1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update a Service Perimeter. The longrunning operation from this RPC will have a successful status once the changes to the Service Perimeter have propagated to long-lasting storage. Service Perimeter containing errors will result in an error response for the first error encountered.");
            service_perimeters1 = service_perimeters1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_all").about("Replace all existing Service Perimeters in an Access Policy with the Service Perimeters provided. This is done atomically. The longrunning operation from this RPC will have a successful status once all replacements have propagated to long-lasting storage. Replacements containing errors will result in an error response for the first error encountered. Replacement will be cancelled on error, existing Service Perimeters will not be affected. Operation.response field will contain ReplaceServicePerimetersResponse.");
            service_perimeters1 = service_perimeters1.subcommand(mcmd);
        }
        let mut gcp_user_access_bindings1 = SubCommand::with_name("gcp_user_access_bindings")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a GcpUserAccessBinding. If the client specifies a name, the server will ignore it. Fails if a resource already exists with the same group_key. Completion of this long-running operation does not necessarily signify that the new binding is deployed onto all affected users, which may take more time.");
            gcp_user_access_bindings1 = gcp_user_access_bindings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a GcpUserAccessBinding. Completion of this long-running operation does not necessarily signify that the binding deletion is deployed onto all affected users, which may take more time.");
            gcp_user_access_bindings1 = gcp_user_access_bindings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the GcpUserAccessBinding with the given name.");
            gcp_user_access_bindings1 = gcp_user_access_bindings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all GcpUserAccessBindings for a Google Cloud organization.");
            gcp_user_access_bindings1 = gcp_user_access_bindings1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a GcpUserAccessBinding. Completion of this long-running operation does not necessarily signify that the changed binding is deployed onto all affected users, which may take more time.");
            gcp_user_access_bindings1 = gcp_user_access_bindings1.subcommand(mcmd);
        }
        organizations0 = organizations0.subcommand(gcp_user_access_bindings1);
        access_policies0 = access_policies0.subcommand(service_perimeters1);
        access_policies0 = access_policies0.subcommand(access_levels1);
        app = app.subcommand(organizations0);
        app = app.subcommand(operations0);
        app = app.subcommand(access_policies0);

        Self { app }
    }
}
use google_accesscontextmanager1 as api;

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

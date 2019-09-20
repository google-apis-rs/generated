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
            .version("0.1.0-20190913")
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
            let mcmd = SubCommand::with_name("create").about("Create an `AccessPolicy`. Fails if this organization already has a\n`AccessPolicy`. The longrunning Operation will have a successful status\nonce the `AccessPolicy` has propagated to long-lasting storage.\nSyntactic and basic semantic errors will be returned in `metadata` as a\nBadRequest proto.");
            access_policies0 = access_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete an AccessPolicy by resource\nname. The longrunning Operation will have a successful status once the\nAccessPolicy\nhas been removed from long-lasting storage.");
            access_policies0 = access_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get an AccessPolicy by name.");
            access_policies0 = access_policies0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("List all AccessPolicies under a\ncontainer.");
            access_policies0 = access_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update an AccessPolicy. The\nlongrunning Operation from this RPC will have a successful status once the\nchanges to the AccessPolicy have propagated\nto long-lasting storage. Syntactic and basic semantic errors will be\nreturned in `metadata` as a BadRequest proto.");
            access_policies0 = access_policies0.subcommand(mcmd);
        }
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is\nno longer interested in the operation result. It does not cancel the\noperation. If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the\nserver doesn\'t support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut access_levels1 = SubCommand::with_name("access_levels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Create an Access Level. The longrunning\noperation from this RPC will have a successful status once the Access\nLevel has\npropagated to long-lasting storage. Access Levels containing\nerrors will result in an error response for the first error encountered.");
            access_levels1 = access_levels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete an Access Level by resource\nname. The longrunning operation from this RPC will have a successful status\nonce the Access Level has been removed\nfrom long-lasting storage.");
            access_levels1 = access_levels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get an Access Level by resource\nname.");
            access_levels1 = access_levels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all Access Levels for an access\npolicy.");
            access_levels1 = access_levels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update an Access Level. The longrunning\noperation from this RPC will have a successful status once the changes to\nthe Access Level have propagated\nto long-lasting storage. Access Levels containing\nerrors will result in an error response for the first error encountered.");
            access_levels1 = access_levels1.subcommand(mcmd);
        }
        let mut service_perimeters1 = SubCommand::with_name("service_perimeters")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Create an Service Perimeter. The\nlongrunning operation from this RPC will have a successful status once the\nService Perimeter has\npropagated to long-lasting storage. Service Perimeters containing\nerrors will result in an error response for the first error encountered.");
            service_perimeters1 = service_perimeters1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete an Service Perimeter by resource\nname. The longrunning operation from this RPC will have a successful status\nonce the Service Perimeter has been\nremoved from long-lasting storage.");
            service_perimeters1 = service_perimeters1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Get an Service Perimeter by resource\nname.");
            service_perimeters1 = service_perimeters1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all Service Perimeters for an\naccess policy.");
            service_perimeters1 = service_perimeters1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update an Service Perimeter. The\nlongrunning operation from this RPC will have a successful status once the\nchanges to the Service Perimeter have\npropagated to long-lasting storage. Service Perimeter containing\nerrors will result in an error response for the first error encountered.");
            service_perimeters1 = service_perimeters1.subcommand(mcmd);
        }
        access_policies0 = access_policies0.subcommand(service_perimeters1);
        access_policies0 = access_policies0.subcommand(access_levels1);
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

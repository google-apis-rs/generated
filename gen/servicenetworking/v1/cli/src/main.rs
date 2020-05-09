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
        let mut app = App::new("servicenetworking1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200508")
            .about("Provides automatic management of network configurations necessary for certain services.")
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
        let mut services0 = SubCommand::with_name("services")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_subnetwork, disable_vpc_service_controls, enable_vpc_service_controls, search_range and validate");
        {
            let mcmd = SubCommand::with_name("add_subnetwork").about("For service producers, provisions a new subnet in a peered service\'s shared\nVPC network in the requested region and with the requested size that\'s\nexpressed as a CIDR range (number of leading bits of ipV4 network mask).\nThe method checks against the assigned allocated ranges to find a\nnon-conflicting IP address range. The method will reuse a subnet if\nsubsequent calls contain the same subnet name, region, and prefix length.\nThis method will make producer\'s tenant project to be a shared VPC service\nproject as needed.");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("disable_vpc_service_controls")
                .about("Disables VPC service controls for a connection.");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("enable_vpc_service_controls")
                .about("Enables VPC service controls for a connection.");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_range").about("Service producers can use this method to find a currently unused range\nwithin consumer allocated ranges. This returned range is not reserved,\nand not guaranteed to remain unused. It will validate previously provided\nallocated ranges, find non-conflicting sub-range of requested size\n(expressed in number of leading bits of ipv4 network mask, as in CIDR range\nnotation).");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("validate").about("Service producers use this method to validate if the consumer provided\nnetwork, project and requested range are valid. This allows them to use\na fail-fast mechanism for consumer requests, and not have to wait for\nAddSubnetwork operation completion to determine if user request is invalid.");
            services0 = services0.subcommand(mcmd);
        }
        let mut connections1 = SubCommand::with_name("connections")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a private connection that establishes a VPC Network Peering\nconnection to a VPC network in the service producer\'s organization.\nThe administrator of the service consumer\'s VPC network invokes this\nmethod. The administrator must assign one or more allocated IP ranges for\nprovisioning subnetworks in the service producer\'s VPC network. This\nconnection is used for all supported services in the service producer\'s\norganization, so it only needs to be invoked once.");
            connections1 = connections1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List the private connections that are configured in a service consumer\'s\nVPC network.");
            connections1 = connections1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates the allocated ranges that are assigned to a connection.");
            connections1 = connections1.subcommand(mcmd);
        }
        let mut roles1 = SubCommand::with_name("roles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: add");
        {
            let mcmd = SubCommand::with_name("add").about("Service producers can use this method to add roles in the shared VPC host\nproject. Each role is bound to the provided member. Each role must be\nselected from within a whitelisted set of roles. Each role is applied at\nonly the granularity specified in the whitelist.");
            roles1 = roles1.subcommand(mcmd);
        }
        services0 = services0.subcommand(roles1);
        services0 = services0.subcommand(connections1);
        app = app.subcommand(services0);
        app = app.subcommand(operations0);

        Self { app }
    }
}
use google_servicenetworking1 as api;

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

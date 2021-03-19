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
            .version("0.1.0-20210316")
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
        let mut services0 = SubCommand::with_name("services")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_subnetwork, disable_vpc_service_controls, enable_vpc_service_controls, search_range and validate");
        {
            let mcmd = SubCommand::with_name("add_subnetwork").about("For service producers, provisions a new subnet in a peered service\'s shared VPC network in the requested region and with the requested size that\'s expressed as a CIDR range (number of leading bits of ipV4 network mask). The method checks against the assigned allocated ranges to find a non-conflicting IP address range. The method will reuse a subnet if subsequent calls contain the same subnet name, region, and prefix length. This method will make producer\'s tenant project to be a shared VPC service project as needed.");
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
            let mcmd = SubCommand::with_name("search_range").about("Service producers can use this method to find a currently unused range within consumer allocated ranges. This returned range is not reserved, and not guaranteed to remain unused. It will validate previously provided allocated ranges, find non-conflicting sub-range of requested size (expressed in number of leading bits of ipv4 network mask, as in CIDR range notation).");
            services0 = services0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("validate").about("Service producers use this method to validate if the consumer provided network, project and requested range are valid. This allows them to use a fail-fast mechanism for consumer requests, and not have to wait for AddSubnetwork operation completion to determine if user request is invalid.");
            services0 = services0.subcommand(mcmd);
        }
        let mut connections1 = SubCommand::with_name("connections")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a private connection that establishes a VPC Network Peering connection to a VPC network in the service producer\'s organization. The administrator of the service consumer\'s VPC network invokes this method. The administrator must assign one or more allocated IP ranges for provisioning subnetworks in the service producer\'s VPC network. This connection is used for all supported services in the service producer\'s organization, so it only needs to be invoked once.");
            connections1 = connections1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List the private connections that are configured in a service consumer\'s VPC network.");
            connections1 = connections1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates the allocated ranges that are assigned to a connection.");
            connections1 = connections1.subcommand(mcmd);
        }
        let mut dns_record_sets1 = SubCommand::with_name("dns_record_sets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: add, remove and update");
        {
            let mcmd = SubCommand::with_name("add").about("Service producers can use this method to add DNS record sets to private DNS zones in the shared producer host project.");
            dns_record_sets1 = dns_record_sets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove").about("Service producers can use this method to remove DNS record sets from private DNS zones in the shared producer host project.");
            dns_record_sets1 = dns_record_sets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Service producers can use this method to update DNS record sets from private DNS zones in the shared producer host project.");
            dns_record_sets1 = dns_record_sets1.subcommand(mcmd);
        }
        let mut dns_zones1 = SubCommand::with_name("dns_zones")
            .setting(AppSettings::ColoredHelp)
            .about("methods: add and remove");
        {
            let mcmd = SubCommand::with_name("add").about("Service producers can use this method to add private DNS zones in the shared producer host project and matching peering zones in the consumer project.");
            dns_zones1 = dns_zones1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove").about("Service producers can use this method to remove private DNS zones in the shared producer host project and matching peering zones in the consumer project.");
            dns_zones1 = dns_zones1.subcommand(mcmd);
        }
        let mut projects1 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: global");
        let mut roles1 = SubCommand::with_name("roles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: add");
        {
            let mcmd = SubCommand::with_name("add").about("Service producers can use this method to add roles in the shared VPC host project. Each role is bound to the provided member. Each role must be selected from within an allowlisted set of roles. Each role is applied at only the granularity specified in the allowlist.");
            roles1 = roles1.subcommand(mcmd);
        }
        let mut global2 = SubCommand::with_name("global")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: networks");
        let mut networks3 = SubCommand::with_name("networks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and update_consumer_config");
        {
            let mcmd = SubCommand::with_name("get").about("Service producers use this method to get the configuration of their connection including the import/export of custom routes and subnetwork routes with public IP.");
            networks3 = networks3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_consumer_config").about("Service producers use this method to update the configuration of their connection including the import/export of custom routes and subnetwork routes with public IP.");
            networks3 = networks3.subcommand(mcmd);
        }
        let mut peered_dns_domains4 = SubCommand::with_name("peered_dns_domains")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a peered DNS domain which sends requests for records in given namespace originating in the service producer VPC network to the consumer VPC network to be resolved.");
            peered_dns_domains4 = peered_dns_domains4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a peered DNS domain.");
            peered_dns_domains4 = peered_dns_domains4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists peered DNS domains for a connection.");
            peered_dns_domains4 = peered_dns_domains4.subcommand(mcmd);
        }
        networks3 = networks3.subcommand(peered_dns_domains4);
        global2 = global2.subcommand(networks3);
        projects1 = projects1.subcommand(global2);
        services0 = services0.subcommand(roles1);
        services0 = services0.subcommand(projects1);
        services0 = services0.subcommand(dns_zones1);
        services0 = services0.subcommand(dns_record_sets1);
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

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
        let mut app = App::new("dns1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210303")
            .about("")
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
        let mut changes0 = SubCommand::with_name("changes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get and list");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Atomically updates the ResourceRecordSet collection.");
            changes0 = changes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Fetches the representation of an existing Change.");
            changes0 = changes0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Enumerates Changes to a ResourceRecordSet collection.");
            changes0 = changes0.subcommand(mcmd);
        }
        let mut dns_keys0 = SubCommand::with_name("dns_keys")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Fetch the representation of an existing DnsKey.");
            dns_keys0 = dns_keys0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Enumerate DnsKeys to a ResourceRecordSet collection.");
            dns_keys0 = dns_keys0.subcommand(mcmd);
        }
        let mut managed_zone_operations0 = SubCommand::with_name("managed_zone_operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Fetches the representation of an existing Operation.");
            managed_zone_operations0 = managed_zone_operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Enumerates Operations for the given ManagedZone.");
            managed_zone_operations0 = managed_zone_operations0.subcommand(mcmd);
        }
        let mut managed_zones0 = SubCommand::with_name("managed_zones")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("create").about("Create a new ManagedZone.");
            managed_zones0 = managed_zones0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Delete a previously created ManagedZone.");
            managed_zones0 = managed_zones0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Fetch the representation of an existing ManagedZone.");
            managed_zones0 = managed_zones0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Enumerate ManagedZones that have been created but not yet deleted.");
            managed_zones0 = managed_zones0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Apply a partial update to an existing ManagedZone.");
            managed_zones0 = managed_zones0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Update an existing ManagedZone.");
            managed_zones0 = managed_zones0.subcommand(mcmd);
        }
        let mut policies0 = SubCommand::with_name("policies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new Policy.");
            policies0 = policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a previously created Policy. Fails if the policy is still being referenced by a network.");
            policies0 = policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Fetches the representation of an existing Policy.");
            policies0 = policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Enumerates all Policies associated with a project.");
            policies0 = policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Apply a partial update to an existing Policy.");
            policies0 = policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Update an existing Policy.");
            policies0 = policies0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Fetch the representation of an existing Project.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut resource_record_sets0 = SubCommand::with_name("resource_record_sets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Enumerates ResourceRecordSets that you have created but not yet deleted.");
            resource_record_sets0 = resource_record_sets0.subcommand(mcmd);
        }
        let mut managed_zones1 = SubCommand::with_name("managed_zones")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: rrsets");
        let mut rrsets2 = SubCommand::with_name("rrsets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Create a new ResourceRecordSet.");
            rrsets2 = rrsets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete a previously created ResourceRecordSet.");
            rrsets2 = rrsets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Fetch the representation of an existing ResourceRecordSet.");
            rrsets2 = rrsets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Apply a partial update to an existing ResourceRecordSet.");
            rrsets2 = rrsets2.subcommand(mcmd);
        }
        managed_zones1 = managed_zones1.subcommand(rrsets2);
        projects0 = projects0.subcommand(managed_zones1);
        app = app.subcommand(resource_record_sets0);
        app = app.subcommand(projects0);
        app = app.subcommand(policies0);
        app = app.subcommand(managed_zones0);
        app = app.subcommand(managed_zone_operations0);
        app = app.subcommand(dns_keys0);
        app = app.subcommand(changes0);

        Self { app }
    }
}
use google_dns1 as api;

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

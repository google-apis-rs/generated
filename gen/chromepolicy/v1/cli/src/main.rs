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
        let mut app = App::new("chromepolicy1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210317")
            .about("The Chrome Policy API is a suite of services that allows Chrome administrators to control the policies applied to their managed Chrome OS devices and Chrome browsers.")
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
        let mut customers0 = SubCommand::with_name("customers")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: policies and policy_schemas");
        let mut policies1 = SubCommand::with_name("policies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: resolve");
        {
            let mcmd = SubCommand::with_name("resolve").about(
                "Gets the resolved policy values for a list of policies that match a search query.",
            );
            policies1 = policies1.subcommand(mcmd);
        }
        let mut policy_schemas1 = SubCommand::with_name("policy_schemas")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get a specific policy schema for a customer by its resource name.");
            policy_schemas1 = policy_schemas1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Gets a list of policy schemas that match a specified filter value for a given customer.");
            policy_schemas1 = policy_schemas1.subcommand(mcmd);
        }
        let mut orgunits2 = SubCommand::with_name("orgunits")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_inherit and batch_modify");
        {
            let mcmd = SubCommand::with_name("batch_inherit").about("Modify multiple policy values that are applied to a specific org unit so that they now inherit the value from a parent (if applicable). All targets must have the same target format. That is to say that they must point to the same target resource and must have the same keys specified in `additionalTargetKeyNames`. On failure the request will return the error details as part of the google.rpc.Status.");
            orgunits2 = orgunits2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_modify").about("Modify multiple policy values that are applied to a specific org unit. All targets must have the same target format. That is to say that they must point to the same target resource and must have the same keys specified in `additionalTargetKeyNames`. On failure the request will return the error details as part of the google.rpc.Status.");
            orgunits2 = orgunits2.subcommand(mcmd);
        }
        policies1 = policies1.subcommand(orgunits2);
        customers0 = customers0.subcommand(policy_schemas1);
        customers0 = customers0.subcommand(policies1);
        app = app.subcommand(customers0);

        Self { app }
    }
}
use google_chromepolicy1 as api;

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

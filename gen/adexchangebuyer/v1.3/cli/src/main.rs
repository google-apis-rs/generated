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
        let mut app = App::new("adexchangebuyer1d3")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210317")
            .about("Accesses your bidding-account information, submits creatives for validation, finds available direct deals, and retrieves performance reports.")
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
        let mut accounts0 = SubCommand::with_name("accounts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one account by ID.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves the authenticated user\'s list of accounts.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing account. This method supports patch semantics.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        let mut billing_info0 = SubCommand::with_name("billing_info")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns the billing information for one account specified by account ID.");
            billing_info0 = billing_info0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of billing information for all accounts of the authenticated user.");
            billing_info0 = billing_info0.subcommand(mcmd);
        }
        let mut budget0 = SubCommand::with_name("budget")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Returns the budget information for the adgroup specified by the accountId and billingId.");
            budget0 = budget0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the budget amount for the budget of the adgroup specified by the accountId and billingId, with the budget amount in the request. This method supports patch semantics.");
            budget0 = budget0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the budget amount for the budget of the adgroup specified by the accountId and billingId, with the budget amount in the request.");
            budget0 = budget0.subcommand(mcmd);
        }
        let mut creatives0 = SubCommand::with_name("creatives")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, insert and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the status for a single creative. A creative will be available 30-40 minutes after submission.");
            creatives0 = creatives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Submit a new creative.");
            creatives0 = creatives0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of the authenticated user\'s active creatives. A creative will be available 30-40 minutes after submission.");
            creatives0 = creatives0.subcommand(mcmd);
        }
        let mut direct_deals0 = SubCommand::with_name("direct_deals")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets one direct deal by ID.");
            direct_deals0 = direct_deals0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves the authenticated user\'s list of direct deals.");
            direct_deals0 = direct_deals0.subcommand(mcmd);
        }
        let mut performance_report0 = SubCommand::with_name("performance_report")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves the authenticated user\'s list of performance metrics.");
            performance_report0 = performance_report0.subcommand(mcmd);
        }
        let mut pretargeting_config0 = SubCommand::with_name("pretargeting_config")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes an existing pretargeting config.");
            pretargeting_config0 = pretargeting_config0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets a specific pretargeting configuration");
            pretargeting_config0 = pretargeting_config0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Inserts a new pretargeting configuration.");
            pretargeting_config0 = pretargeting_config0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves a list of the authenticated user\'s pretargeting configurations.",
            );
            pretargeting_config0 = pretargeting_config0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates an existing pretargeting config. This method supports patch semantics.",
            );
            pretargeting_config0 = pretargeting_config0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates an existing pretargeting config.");
            pretargeting_config0 = pretargeting_config0.subcommand(mcmd);
        }
        app = app.subcommand(pretargeting_config0);
        app = app.subcommand(performance_report0);
        app = app.subcommand(direct_deals0);
        app = app.subcommand(creatives0);
        app = app.subcommand(budget0);
        app = app.subcommand(billing_info0);
        app = app.subcommand(accounts0);

        Self { app }
    }
}
use google_adexchangebuyer1d3 as api;

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

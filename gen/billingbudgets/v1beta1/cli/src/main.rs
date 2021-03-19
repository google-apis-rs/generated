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
        let mut app = App::new("billingbudgets1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210314")
            .about("The Cloud Billing Budget API stores Cloud Billing budgets, which define a budget plan and the rules to execute as spend is tracked against that plan.")
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
        let mut billing_accounts0 = SubCommand::with_name("billing_accounts")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: budgets");
        let mut budgets1 = SubCommand::with_name("budgets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new budget. See Quotas and limits for more information on the limits of the number of budgets you can create.");
            budgets1 = budgets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a budget. Returns successfully if already deleted.");
            budgets1 = budgets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a budget. WARNING: There are some fields exposed on the Google Cloud Console that aren\'t available on this API. When reading from the API, you will not see these fields in the return value, though they may have been set in the Cloud Console.");
            budgets1 = budgets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of budgets for a billing account. WARNING: There are some fields exposed on the Google Cloud Console that aren\'t available on this API. When reading from the API, you will not see these fields in the return value, though they may have been set in the Cloud Console.");
            budgets1 = budgets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a budget and returns the updated budget. WARNING: There are some fields exposed on the Google Cloud Console that aren\'t available on this API. Budget fields that are not exposed in this API will not be changed by this method.");
            budgets1 = budgets1.subcommand(mcmd);
        }
        billing_accounts0 = billing_accounts0.subcommand(budgets1);
        app = app.subcommand(billing_accounts0);

        Self { app }
    }
}
use google_billingbudgets1_beta1 as api;

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

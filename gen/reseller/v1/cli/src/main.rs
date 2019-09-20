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
        let mut app = App::new("reseller1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190228")
            .about("Creates and manages your customers and their subscriptions.")
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
            .about("methods: get, insert, patch and update");
        {
            let mcmd = SubCommand::with_name("get").about("Get a customer account.");
            customers0 = customers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Order a new customer\'s account.");
            customers0 = customers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Update a customer account\'s settings. This method supports patch semantics.",
            );
            customers0 = customers0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Update a customer account\'s settings.");
            customers0 = customers0.subcommand(mcmd);
        }
        let mut resellernotify0 = SubCommand::with_name("resellernotify")
            .setting(AppSettings::ColoredHelp)
            .about("methods: getwatchdetails, register and unregister");
        {
            let mcmd = SubCommand::with_name("getwatchdetails")
                .about("Returns all the details of the watch corresponding to the reseller.");
            resellernotify0 = resellernotify0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("register")
                .about("Registers a Reseller for receiving notifications.");
            resellernotify0 = resellernotify0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unregister")
                .about("Unregisters a Reseller for receiving notifications.");
            resellernotify0 = resellernotify0.subcommand(mcmd);
        }
        let mut subscriptions0 = SubCommand::with_name("subscriptions")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: activate, change_plan, change_renewal_settings, change_seats, delete, get, insert, list, start_paid_service and suspend");
        {
            let mcmd = SubCommand::with_name("activate")
                .about("Activates a subscription previously suspended by the reseller");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("change_plan").about("Update a subscription plan. Use this method to update a plan for a 30-day trial or a flexible plan subscription to an annual commitment plan with monthly or yearly payments.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("change_renewal_settings").about("Update a user license\'s renewal settings. This is applicable for accounts with annual commitment plans only.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("change_seats")
                .about("Update a subscription\'s user license settings.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Cancel or transfer a subscription to direct.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get a specific subscription.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Create or transfer a subscription.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List of subscriptions managed by the reseller. The list can be all subscriptions, all of a customer\'s subscriptions, or all of a customer\'s transferable subscriptions.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start_paid_service").about(
                "Immediately move a 30-day free trial subscription to a paid service subscription.",
            );
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("suspend").about("Suspends an active subscription.");
            subscriptions0 = subscriptions0.subcommand(mcmd);
        }
        app = app.subcommand(subscriptions0);
        app = app.subcommand(resellernotify0);
        app = app.subcommand(customers0);

        Self { app }
    }
}
use google_reseller1 as api;

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

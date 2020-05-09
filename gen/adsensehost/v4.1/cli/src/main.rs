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
        let mut app = App::new("adsensehost4d1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200506")
            .about("Generates performance reports, generates ad codes, and provides publisher management capabilities for AdSense Hosts.")
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
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get information about the selected associated AdSense account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "List hosted accounts associated with this AdSense account by ad client id.",
            );
            accounts0 = accounts0.subcommand(mcmd);
        }
        let mut adclients0 = SubCommand::with_name("adclients")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get information about one of the ad clients in the Host AdSense account.");
            adclients0 = adclients0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all host ad clients in this AdSense account.");
            adclients0 = adclients0.subcommand(mcmd);
        }
        let mut associationsessions0 = SubCommand::with_name("associationsessions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: start and verify");
        {
            let mcmd = SubCommand::with_name("start").about(
                "Create an association session for initiating an association with an AdSense user.",
            );
            associationsessions0 = associationsessions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("verify").about("Verify an association session after the association callback returns from AdSense signup.");
            associationsessions0 = associationsessions0.subcommand(mcmd);
        }
        let mut customchannels0 = SubCommand::with_name("customchannels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete a specific custom channel from the host AdSense account.");
            customchannels0 = customchannels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get a specific custom channel from the host AdSense account.");
            customchannels0 = customchannels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Add a new custom channel to the host AdSense account.");
            customchannels0 = customchannels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all host custom channels in this AdSense account.");
            customchannels0 = customchannels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update a custom channel in the host AdSense account. This method supports patch semantics.");
            customchannels0 = customchannels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Update a custom channel in the host AdSense account.");
            customchannels0 = customchannels0.subcommand(mcmd);
        }
        let mut reports0 = SubCommand::with_name("reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generate");
        {
            let mcmd = SubCommand::with_name("generate").about("Generate an AdSense report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify \"alt=csv\" as a query parameter.");
            reports0 = reports0.subcommand(mcmd);
        }
        let mut urlchannels0 = SubCommand::with_name("urlchannels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert and list");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete a URL channel from the host AdSense account.");
            urlchannels0 = urlchannels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Add a new URL channel to the host AdSense account.");
            urlchannels0 = urlchannels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all host URL channels in the host AdSense account.");
            urlchannels0 = urlchannels0.subcommand(mcmd);
        }
        let mut adclients1 = SubCommand::with_name("adclients")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Get information about one of the ad clients in the specified publisher\'s AdSense account.");
            adclients1 = adclients1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all hosted ad clients in the specified hosted account.");
            adclients1 = adclients1.subcommand(mcmd);
        }
        let mut adunits1 = SubCommand::with_name("adunits")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, get_ad_code, insert, list, patch and update");
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Delete the specified ad unit from the specified publisher AdSense account.",
            );
            adunits1 = adunits1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get the specified host ad unit in this AdSense account.");
            adunits1 = adunits1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_ad_code").about("Get ad code for the specified ad unit, attaching the specified host custom channels.");
            adunits1 = adunits1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Insert the supplied ad unit into the specified publisher AdSense account.");
            adunits1 = adunits1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all ad units in the specified publisher\'s AdSense account.");
            adunits1 = adunits1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update the supplied ad unit in the specified publisher AdSense account. This method supports patch semantics.");
            adunits1 = adunits1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Update the supplied ad unit in the specified publisher AdSense account.");
            adunits1 = adunits1.subcommand(mcmd);
        }
        let mut reports1 = SubCommand::with_name("reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generate");
        {
            let mcmd = SubCommand::with_name("generate").about("Generate an AdSense report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify \"alt=csv\" as a query parameter.");
            reports1 = reports1.subcommand(mcmd);
        }
        accounts0 = accounts0.subcommand(reports1);
        accounts0 = accounts0.subcommand(adunits1);
        accounts0 = accounts0.subcommand(adclients1);
        app = app.subcommand(urlchannels0);
        app = app.subcommand(reports0);
        app = app.subcommand(customchannels0);
        app = app.subcommand(associationsessions0);
        app = app.subcommand(adclients0);
        app = app.subcommand(accounts0);

        Self { app }
    }
}
use google_adsensehost4d1 as api;

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

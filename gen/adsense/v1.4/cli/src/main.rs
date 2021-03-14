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
        let mut app = App::new("adsense1d4")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20201002")
            .about("Accesses AdSense publishers\' inventory and generates performance reports.")
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
                .about("Get information about the selected AdSense account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all accounts available to this AdSense account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        let mut adclients0 = SubCommand::with_name("adclients")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd =
                SubCommand::with_name("list").about("List all ad clients in this AdSense account.");
            adclients0 = adclients0.subcommand(mcmd);
        }
        let mut adunits0 = SubCommand::with_name("adunits")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, get_ad_code and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the specified ad unit in the specified ad client.");
            adunits0 = adunits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_ad_code")
                .about("Get ad code for the specified ad unit.");
            adunits0 = adunits0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all ad units in the specified ad client for this AdSense account.");
            adunits0 = adunits0.subcommand(mcmd);
        }
        let mut alerts0 = SubCommand::with_name("alerts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete and list");
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Dismiss (delete) the specified alert from the publisher\'s AdSense account.",
            );
            alerts0 = alerts0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("List the alerts for this AdSense account.");
            alerts0 = alerts0.subcommand(mcmd);
        }
        let mut customchannels0 = SubCommand::with_name("customchannels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get the specified custom channel from the specified ad client.");
            customchannels0 = customchannels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "List all custom channels in the specified ad client for this AdSense account.",
            );
            customchannels0 = customchannels0.subcommand(mcmd);
        }
        let mut metadata0 = SubCommand::with_name("metadata")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: dimensions and metrics");
        let mut payments0 = SubCommand::with_name("payments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd =
                SubCommand::with_name("list").about("List the payments for this AdSense account.");
            payments0 = payments0.subcommand(mcmd);
        }
        let mut reports0 = SubCommand::with_name("reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generate");
        {
            let mcmd = SubCommand::with_name("generate").about("Generate an AdSense report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify \"alt=csv\" as a query parameter.");
            reports0 = reports0.subcommand(mcmd);
        }
        let mut savedadstyles0 = SubCommand::with_name("savedadstyles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get a specific saved ad style from the user\'s account.");
            savedadstyles0 = savedadstyles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all saved ad styles in the user\'s account.");
            savedadstyles0 = savedadstyles0.subcommand(mcmd);
        }
        let mut urlchannels0 = SubCommand::with_name("urlchannels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about(
                "List all URL channels in the specified ad client for this AdSense account.",
            );
            urlchannels0 = urlchannels0.subcommand(mcmd);
        }
        let mut adclients1 = SubCommand::with_name("adclients")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_ad_code and list");
        {
            let mcmd = SubCommand::with_name("get_ad_code")
                .about("Get Auto ad code for a given ad client.");
            adclients1 = adclients1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all ad clients in the specified account.");
            adclients1 = adclients1.subcommand(mcmd);
        }
        let mut adunits1 = SubCommand::with_name("adunits")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, get_ad_code and list");
        {
            let mcmd = SubCommand::with_name("get").about(
                "Gets the specified ad unit in the specified ad client for the specified account.",
            );
            adunits1 = adunits1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_ad_code")
                .about("Get ad code for the specified ad unit.");
            adunits1 = adunits1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all ad units in the specified ad client for the specified account.");
            adunits1 = adunits1.subcommand(mcmd);
        }
        let mut alerts1 = SubCommand::with_name("alerts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Dismiss (delete) the specified alert from the specified publisher AdSense account.");
            alerts1 = alerts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List the alerts for the specified AdSense account.");
            alerts1 = alerts1.subcommand(mcmd);
        }
        let mut customchannels1 = SubCommand::with_name("customchannels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Get the specified custom channel from the specified ad client for the specified account.");
            customchannels1 = customchannels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "List all custom channels in the specified ad client for the specified account.",
            );
            customchannels1 = customchannels1.subcommand(mcmd);
        }
        let mut payments1 = SubCommand::with_name("payments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("List the payments for the specified AdSense account.");
            payments1 = payments1.subcommand(mcmd);
        }
        let mut reports1 = SubCommand::with_name("reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generate");
        {
            let mcmd = SubCommand::with_name("generate").about("Generate an AdSense report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify \"alt=csv\" as a query parameter.");
            reports1 = reports1.subcommand(mcmd);
        }
        let mut savedadstyles1 = SubCommand::with_name("savedadstyles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("List a specific saved ad style for the specified account.");
            savedadstyles1 = savedadstyles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all saved ad styles in the specified account.");
            savedadstyles1 = savedadstyles1.subcommand(mcmd);
        }
        let mut urlchannels1 = SubCommand::with_name("urlchannels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about(
                "List all URL channels in the specified ad client for the specified account.",
            );
            urlchannels1 = urlchannels1.subcommand(mcmd);
        }
        let mut customchannels1 = SubCommand::with_name("customchannels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all custom channels which the specified ad unit belongs to.");
            customchannels1 = customchannels1.subcommand(mcmd);
        }
        let mut adunits1 = SubCommand::with_name("adunits")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all ad units in the specified custom channel.");
            adunits1 = adunits1.subcommand(mcmd);
        }
        let mut dimensions1 = SubCommand::with_name("dimensions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("List the metadata for the dimensions available to this AdSense account.");
            dimensions1 = dimensions1.subcommand(mcmd);
        }
        let mut metrics1 = SubCommand::with_name("metrics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("List the metadata for the metrics available to this AdSense account.");
            metrics1 = metrics1.subcommand(mcmd);
        }
        let mut saved1 = SubCommand::with_name("saved")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generate and list");
        {
            let mcmd = SubCommand::with_name("generate").about("Generate an AdSense report based on the saved report ID sent in the query parameters.");
            saved1 = saved1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all saved reports in this AdSense account.");
            saved1 = saved1.subcommand(mcmd);
        }
        let mut customchannels2 = SubCommand::with_name("customchannels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all custom channels which the specified ad unit belongs to.");
            customchannels2 = customchannels2.subcommand(mcmd);
        }
        let mut adunits2 = SubCommand::with_name("adunits")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all ad units in the specified custom channel.");
            adunits2 = adunits2.subcommand(mcmd);
        }
        let mut saved2 = SubCommand::with_name("saved")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generate and list");
        {
            let mcmd = SubCommand::with_name("generate").about("Generate an AdSense report based on the saved report ID sent in the query parameters.");
            saved2 = saved2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all saved reports in the specified AdSense account.");
            saved2 = saved2.subcommand(mcmd);
        }
        reports1 = reports1.subcommand(saved2);
        customchannels1 = customchannels1.subcommand(adunits2);
        adunits1 = adunits1.subcommand(customchannels2);
        reports0 = reports0.subcommand(saved1);
        metadata0 = metadata0.subcommand(metrics1);
        metadata0 = metadata0.subcommand(dimensions1);
        customchannels0 = customchannels0.subcommand(adunits1);
        adunits0 = adunits0.subcommand(customchannels1);
        accounts0 = accounts0.subcommand(urlchannels1);
        accounts0 = accounts0.subcommand(savedadstyles1);
        accounts0 = accounts0.subcommand(reports1);
        accounts0 = accounts0.subcommand(payments1);
        accounts0 = accounts0.subcommand(customchannels1);
        accounts0 = accounts0.subcommand(alerts1);
        accounts0 = accounts0.subcommand(adunits1);
        accounts0 = accounts0.subcommand(adclients1);
        app = app.subcommand(urlchannels0);
        app = app.subcommand(savedadstyles0);
        app = app.subcommand(reports0);
        app = app.subcommand(payments0);
        app = app.subcommand(metadata0);
        app = app.subcommand(customchannels0);
        app = app.subcommand(alerts0);
        app = app.subcommand(adunits0);
        app = app.subcommand(adclients0);
        app = app.subcommand(accounts0);

        Self { app }
    }
}
use google_adsense1d4 as api;

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

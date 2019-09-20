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
        let mut app = App::new("firebasehosting1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190916")
            .about("The Firebase Hosting REST API enables programmatic and customizable deployments to your Firebase-hosted sites. Use this REST API to deploy new or updated hosting configurations and content files.")
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
        let mut sites0 = SubCommand::with_name("sites")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_config and update_config");
        {
            let mcmd = SubCommand::with_name("get_config")
                .about("Gets the Hosting metadata for a specific site.");
            sites0 = sites0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_config")
                .about("Sets the Hosting metadata for a specific site.");
            sites0 = sites0.subcommand(mcmd);
        }
        let mut domains1 = SubCommand::with_name("domains")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a domain mapping on the specified site.");
            domains1 = domains1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the existing domain mapping on the specified site.");
            domains1 = domains1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets a domain mapping on the specified site.");
            domains1 = domains1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the domains for the specified site.");
            domains1 = domains1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the specified domain mapping, creating the mapping as if it does\nnot exist.");
            domains1 = domains1.subcommand(mcmd);
        }
        let mut releases1 = SubCommand::with_name("releases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new release which makes the content of the specified version\nactively display on the site.");
            releases1 = releases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the releases that have been created on the specified site.");
            releases1 = releases1.subcommand(mcmd);
        }
        let mut versions1 = SubCommand::with_name("versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, patch and populate_files");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new version for a site.");
            versions1 = versions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified version.");
            versions1 = versions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified metadata for a version. Note that this method will\nfail with `FAILED_PRECONDITION` in the event of an invalid state\ntransition. The only valid transition for a version is currently from a\n`CREATED` status to a `FINALIZED` status.\nUse [`DeleteVersion`](../sites.versions/delete) to set the status of a\nversion to `DELETED`.");
            versions1 = versions1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("populate_files").about("Adds content files to a version.");
            versions1 = versions1.subcommand(mcmd);
        }
        let mut files2 = SubCommand::with_name("files")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the remaining files to be uploaded for the specified version.");
            files2 = files2.subcommand(mcmd);
        }
        versions1 = versions1.subcommand(files2);
        sites0 = sites0.subcommand(versions1);
        sites0 = sites0.subcommand(releases1);
        sites0 = sites0.subcommand(domains1);
        app = app.subcommand(sites0);

        Self { app }
    }
}
use google_firebasehosting1_beta1 as api;

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

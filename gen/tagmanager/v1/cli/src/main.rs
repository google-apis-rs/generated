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
        let mut app = App::new("tagmanager1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190516")
            .about("Accesses Tag Manager accounts and containers.")
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
            .about("methods: get, list and update");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a GTM Account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all GTM Accounts that a user has access to.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a GTM Account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        let mut containers1 = SubCommand::with_name("containers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Container.");
            containers1 = containers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Container.");
            containers1 = containers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Container.");
            containers1 = containers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all Containers that belongs to a GTM Account.");
            containers1 = containers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a Container.");
            containers1 = containers1.subcommand(mcmd);
        }
        let mut permissions1 = SubCommand::with_name("permissions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a user\'s Account & Container Permissions.");
            permissions1 = permissions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Removes a user from the account, revoking access to it and all of its containers.",
            );
            permissions1 = permissions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a user\'s Account & Container Permissions.");
            permissions1 = permissions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List all users that have access to the account along with Account and Container Permissions granted to each of them.");
            permissions1 = permissions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Updates a user\'s Account & Container Permissions.");
            permissions1 = permissions1.subcommand(mcmd);
        }
        let mut environments2 = SubCommand::with_name("environments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a GTM Environment.");
            environments2 = environments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a GTM Environment.");
            environments2 = environments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a GTM Environment.");
            environments2 = environments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all GTM Environments of a GTM Container.");
            environments2 = environments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a GTM Environment.");
            environments2 = environments2.subcommand(mcmd);
        }
        let mut folders2 = SubCommand::with_name("folders")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a GTM Folder.");
            folders2 = folders2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a GTM Folder.");
            folders2 = folders2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a GTM Folder.");
            folders2 = folders2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all GTM Folders of a Container.");
            folders2 = folders2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a GTM Folder.");
            folders2 = folders2.subcommand(mcmd);
        }
        let mut move_folders2 = SubCommand::with_name("move_folders")
            .setting(AppSettings::ColoredHelp)
            .about("methods: update");
        {
            let mcmd = SubCommand::with_name("update").about("Moves entities to a GTM Folder.");
            move_folders2 = move_folders2.subcommand(mcmd);
        }
        let mut reauthorize_environments2 = SubCommand::with_name("reauthorize_environments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: update");
        {
            let mcmd = SubCommand::with_name("update")
                .about("Re-generates the authorization code for a GTM Environment.");
            reauthorize_environments2 = reauthorize_environments2.subcommand(mcmd);
        }
        let mut tags2 = SubCommand::with_name("tags")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a GTM Tag.");
            tags2 = tags2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a GTM Tag.");
            tags2 = tags2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a GTM Tag.");
            tags2 = tags2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all GTM Tags of a Container.");
            tags2 = tags2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a GTM Tag.");
            tags2 = tags2.subcommand(mcmd);
        }
        let mut triggers2 = SubCommand::with_name("triggers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a GTM Trigger.");
            triggers2 = triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a GTM Trigger.");
            triggers2 = triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a GTM Trigger.");
            triggers2 = triggers2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all GTM Triggers of a Container.");
            triggers2 = triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a GTM Trigger.");
            triggers2 = triggers2.subcommand(mcmd);
        }
        let mut variables2 = SubCommand::with_name("variables")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a GTM Variable.");
            variables2 = variables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a GTM Variable.");
            variables2 = variables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a GTM Variable.");
            variables2 = variables2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all GTM Variables of a Container.");
            variables2 = variables2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a GTM Variable.");
            variables2 = variables2.subcommand(mcmd);
        }
        let mut versions2 = SubCommand::with_name("versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, publish, restore, undelete and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Container Version.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Container Version.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Container Version.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all Container Versions of a GTM Container.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("publish").about("Publishes a Container Version.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("restore").about("Restores a Container Version. This will overwrite the container\'s current configuration (including its variables, triggers and tags). The operation will not have any effect on the version that is being served (i.e. the published version).");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete").about("Undeletes a Container Version.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a Container Version.");
            versions2 = versions2.subcommand(mcmd);
        }
        let mut entities3 = SubCommand::with_name("entities")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all entities in a GTM Folder.");
            entities3 = entities3.subcommand(mcmd);
        }
        folders2 = folders2.subcommand(entities3);
        containers1 = containers1.subcommand(versions2);
        containers1 = containers1.subcommand(variables2);
        containers1 = containers1.subcommand(triggers2);
        containers1 = containers1.subcommand(tags2);
        containers1 = containers1.subcommand(reauthorize_environments2);
        containers1 = containers1.subcommand(move_folders2);
        containers1 = containers1.subcommand(folders2);
        containers1 = containers1.subcommand(environments2);
        accounts0 = accounts0.subcommand(permissions1);
        accounts0 = accounts0.subcommand(containers1);
        app = app.subcommand(accounts0);

        Self { app }
    }
}
use google_tagmanager1 as api;

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

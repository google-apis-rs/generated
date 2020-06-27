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
        let mut app = App::new("vault1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200623")
            .about("Archiving and eDiscovery for G Suite.")
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
        let mut matters0 = SubCommand::with_name("matters")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_permissions, close, create, delete, get, list, remove_permissions, reopen, undelete and update");
        {
            let mcmd = SubCommand::with_name("add_permissions")
                .about("Adds an account as a matter collaborator.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("close")
                .about("Closes the specified matter. Returns matter with updated state.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new matter with the given name and description. The initial state\nis open, and the owner is the method caller. Returns the created matter\nwith default view.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified matter. Returns matter with updated state.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified matter.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists matters the user has access to.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_permissions")
                .about("Removes an account as a matter collaborator.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reopen")
                .about("Reopens the specified matter. Returns matter with updated state.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete")
                .about("Undeletes the specified matter. Returns matter with updated state.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the specified matter.\nThis updates only the name and description of the matter, identified by\nmatter ID. Changes to any other fields are ignored.\nReturns the default view of the matter.");
            matters0 = matters0.subcommand(mcmd);
        }
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is\nno longer interested in the operation result. It does not cancel the\noperation. If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut exports1 = SubCommand::with_name("exports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an Export.");
            exports1 = exports1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an Export.");
            exports1 = exports1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an Export.");
            exports1 = exports1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Exports.");
            exports1 = exports1.subcommand(mcmd);
        }
        let mut holds1 = SubCommand::with_name("holds")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_held_accounts, create, delete, get, list, remove_held_accounts and update");
        {
            let mcmd = SubCommand::with_name("add_held_accounts").about("Adds HeldAccounts to a hold. Returns a list of accounts that have been\nsuccessfully added. Accounts can only be added to an existing account-based\nhold.");
            holds1 = holds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a hold in the given matter.");
            holds1 = holds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Removes a hold by ID. This will release any HeldAccounts on this Hold.");
            holds1 = holds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a hold by ID.");
            holds1 = holds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists holds within a matter. An empty page token in ListHoldsResponse\ndenotes no more holds to list.");
            holds1 = holds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_held_accounts").about("Removes HeldAccounts from a hold. Returns a list of statuses in the same\norder as the request. If this request leaves the hold with no held\naccounts, the hold will not apply to any accounts.");
            holds1 = holds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the OU and/or query parameters of a hold. You cannot add accounts\nto a hold that covers an OU, nor can you add OUs to a hold that covers\nindividual accounts. Accounts listed in the hold will be ignored.");
            holds1 = holds1.subcommand(mcmd);
        }
        let mut saved_queries1 = SubCommand::with_name("saved_queries")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a saved query.");
            saved_queries1 = saved_queries1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a saved query by Id.");
            saved_queries1 = saved_queries1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a saved query by Id.");
            saved_queries1 = saved_queries1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists saved queries within a matter. An empty page token in\nListSavedQueriesResponse denotes no more saved queries to list.");
            saved_queries1 = saved_queries1.subcommand(mcmd);
        }
        let mut accounts2 = SubCommand::with_name("accounts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete and list");
        {
            let mcmd = SubCommand::with_name("create").about("Adds a HeldAccount to a hold. Accounts can only be added to a hold that\nhas no held_org_unit set. Attempting to add an account to an OU-based\nhold will result in an error.");
            accounts2 = accounts2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Removes a HeldAccount from a hold. If this request leaves the hold with\nno held accounts, the hold will not apply to any accounts.");
            accounts2 = accounts2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists HeldAccounts for a hold. This will only list individually specified\nheld accounts. If the hold is on an OU, then use\n<a href=\"https://developers.google.com/admin-sdk/\">Admin SDK</a>\nto enumerate its members.");
            accounts2 = accounts2.subcommand(mcmd);
        }
        holds1 = holds1.subcommand(accounts2);
        matters0 = matters0.subcommand(saved_queries1);
        matters0 = matters0.subcommand(holds1);
        matters0 = matters0.subcommand(exports1);
        app = app.subcommand(operations0);
        app = app.subcommand(matters0);

        Self { app }
    }
}
use google_vault1 as api;

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

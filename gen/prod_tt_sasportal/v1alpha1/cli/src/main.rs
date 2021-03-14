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
        let mut app = App::new("prod_tt_sasportal1_alpha1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210312")
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
        let mut customers0 = SubCommand::with_name("customers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and patch");
        {
            let mcmd = SubCommand::with_name("get").about("Returns a requested customer.");
            customers0 = customers0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Returns a list of requested customers.");
            customers0 = customers0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing customer.");
            customers0 = customers0.subcommand(mcmd);
        }
        let mut deployments0 = SubCommand::with_name("deployments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Returns a requested deployment.");
            deployments0 = deployments0.subcommand(mcmd);
        }
        let mut installer0 = SubCommand::with_name("installer")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generate_secret and validate");
        {
            let mcmd = SubCommand::with_name("generate_secret")
                .about("Generates a secret to be used with the ValidateInstaller.");
            installer0 = installer0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("validate")
                .about("Validates the identity of a Certified Professional Installer (CPI).");
            installer0 = installer0.subcommand(mcmd);
        }
        let mut nodes0 = SubCommand::with_name("nodes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Returns a requested node.");
            nodes0 = nodes0.subcommand(mcmd);
        }
        let mut policies0 = SubCommand::with_name("policies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, set and test");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            policies0 = policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set").about("Sets the access control policy on the specified resource. Replaces any existing policy.");
            policies0 = policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test")
                .about("Returns permissions that a caller has on the specified resource.");
            policies0 = policies0.subcommand(mcmd);
        }
        let mut deployments1 = SubCommand::with_name("deployments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, r#move and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new deployment.");
            deployments1 = deployments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a deployment.");
            deployments1 = deployments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a requested deployment.");
            deployments1 = deployments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists deployments.");
            deployments1 = deployments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("r#move")
                .about("Moves a deployment under another node or customer.");
            deployments1 = deployments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing deployment.");
            deployments1 = deployments1.subcommand(mcmd);
        }
        let mut devices1 = SubCommand::with_name("devices")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, create_signed, delete, get, list, r#move, patch, sign_device and update_signed");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a device under a node or customer.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create_signed")
                .about("Creates a signed device under a node or customer.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a device.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details about a device.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists devices under a node or customer.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("r#move")
                .about("Moves a device under another node or customer.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a device.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("sign_device").about("Signs a device.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_signed").about("Updates a signed device.");
            devices1 = devices1.subcommand(mcmd);
        }
        let mut nodes1 = SubCommand::with_name("nodes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, r#move and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new node.");
            nodes1 = nodes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a node.");
            nodes1 = nodes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a requested node.");
            nodes1 = nodes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists nodes.");
            nodes1 = nodes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("r#move")
                .about("Moves a node under another node or customer.");
            nodes1 = nodes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing node.");
            nodes1 = nodes1.subcommand(mcmd);
        }
        let mut devices1 = SubCommand::with_name("devices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, r#move, patch, sign_device and update_signed");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a device.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details about a device.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("r#move")
                .about("Moves a device under another node or customer.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a device.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("sign_device").about("Signs a device.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_signed").about("Updates a signed device.");
            devices1 = devices1.subcommand(mcmd);
        }
        let mut deployments1 = SubCommand::with_name("deployments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list, r#move and patch");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a deployment.");
            deployments1 = deployments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a requested deployment.");
            deployments1 = deployments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists deployments.");
            deployments1 = deployments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("r#move")
                .about("Moves a deployment under another node or customer.");
            deployments1 = deployments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing deployment.");
            deployments1 = deployments1.subcommand(mcmd);
        }
        let mut devices1 = SubCommand::with_name("devices")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, create_signed, delete, get, list, r#move, patch, sign_device and update_signed");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a device under a node or customer.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create_signed")
                .about("Creates a signed device under a node or customer.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a device.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details about a device.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists devices under a node or customer.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("r#move")
                .about("Moves a device under another node or customer.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a device.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("sign_device").about("Signs a device.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_signed").about("Updates a signed device.");
            devices1 = devices1.subcommand(mcmd);
        }
        let mut nodes1 = SubCommand::with_name("nodes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, r#move and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new node.");
            nodes1 = nodes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a node.");
            nodes1 = nodes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a requested node.");
            nodes1 = nodes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists nodes.");
            nodes1 = nodes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("r#move")
                .about("Moves a node under another node or customer.");
            nodes1 = nodes1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing node.");
            nodes1 = nodes1.subcommand(mcmd);
        }
        let mut devices2 = SubCommand::with_name("devices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, create_signed and list");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a device under a node or customer.");
            devices2 = devices2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create_signed")
                .about("Creates a signed device under a node or customer.");
            devices2 = devices2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists devices under a node or customer.");
            devices2 = devices2.subcommand(mcmd);
        }
        let mut deployments2 = SubCommand::with_name("deployments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new deployment.");
            deployments2 = deployments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists deployments.");
            deployments2 = deployments2.subcommand(mcmd);
        }
        let mut devices2 = SubCommand::with_name("devices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, create_signed and list");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a device under a node or customer.");
            devices2 = devices2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create_signed")
                .about("Creates a signed device under a node or customer.");
            devices2 = devices2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists devices under a node or customer.");
            devices2 = devices2.subcommand(mcmd);
        }
        let mut nodes2 = SubCommand::with_name("nodes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new node.");
            nodes2 = nodes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists nodes.");
            nodes2 = nodes2.subcommand(mcmd);
        }
        let mut devices2 = SubCommand::with_name("devices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, create_signed and list");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a device under a node or customer.");
            devices2 = devices2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create_signed")
                .about("Creates a signed device under a node or customer.");
            devices2 = devices2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists devices under a node or customer.");
            devices2 = devices2.subcommand(mcmd);
        }
        let mut deployments2 = SubCommand::with_name("deployments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new deployment.");
            deployments2 = deployments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists deployments.");
            deployments2 = deployments2.subcommand(mcmd);
        }
        let mut devices2 = SubCommand::with_name("devices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, create_signed and list");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a device under a node or customer.");
            devices2 = devices2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create_signed")
                .about("Creates a signed device under a node or customer.");
            devices2 = devices2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists devices under a node or customer.");
            devices2 = devices2.subcommand(mcmd);
        }
        let mut nodes2 = SubCommand::with_name("nodes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new node.");
            nodes2 = nodes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists nodes.");
            nodes2 = nodes2.subcommand(mcmd);
        }
        nodes1 = nodes1.subcommand(nodes2);
        nodes1 = nodes1.subcommand(devices2);
        nodes1 = nodes1.subcommand(deployments2);
        deployments1 = deployments1.subcommand(devices2);
        nodes1 = nodes1.subcommand(nodes2);
        nodes1 = nodes1.subcommand(devices2);
        nodes1 = nodes1.subcommand(deployments2);
        deployments1 = deployments1.subcommand(devices2);
        nodes0 = nodes0.subcommand(nodes1);
        nodes0 = nodes0.subcommand(devices1);
        nodes0 = nodes0.subcommand(deployments1);
        deployments0 = deployments0.subcommand(devices1);
        customers0 = customers0.subcommand(nodes1);
        customers0 = customers0.subcommand(devices1);
        customers0 = customers0.subcommand(deployments1);
        app = app.subcommand(policies0);
        app = app.subcommand(nodes0);
        app = app.subcommand(installer0);
        app = app.subcommand(deployments0);
        app = app.subcommand(customers0);

        Self { app }
    }
}
use google_prod_tt_sasportal1_alpha1 as api;

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

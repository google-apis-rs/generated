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
        let mut app = App::new("secretmanager1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200417")
            .about("Stores sensitive data such as API keys, passwords, and certificates. Provides convenience while improving security.\n")
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
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations and secrets");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut secrets1 = SubCommand::with_name("secrets")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_version, create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("add_version").about("Creates a new SecretVersion containing secret data and attaches\nit to an existing Secret.");
            secrets1 = secrets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new Secret containing no SecretVersions.");
            secrets1 = secrets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Secret.");
            secrets1 = secrets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets metadata for a given Secret.");
            secrets1 = secrets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a secret.\nReturns empty policy if the secret exists and does not have a policy set.");
            secrets1 = secrets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Secrets.");
            secrets1 = secrets1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates metadata of an existing Secret.");
            secrets1 = secrets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified secret. Replaces any\nexisting policy.\n\nPermissions on SecretVersions are enforced according\nto the policy set on the associated Secret.");
            secrets1 = secrets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has for the specified secret.\nIf the secret does not exist, this call returns an empty set of\npermissions, not a NOT_FOUND error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning.");
            secrets1 = secrets1.subcommand(mcmd);
        }
        let mut versions2 = SubCommand::with_name("versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: access, destroy, disable, enable, get and list");
        {
            let mcmd = SubCommand::with_name("access").about("Accesses a SecretVersion. This call returns the secret data.\n\n`projects/*/secrets/*/versions/latest` is an alias to the `latest`\nSecretVersion.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("destroy").about("Destroys a SecretVersion.\n\nSets the state of the SecretVersion to\nDESTROYED and irrevocably destroys the\nsecret data.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("disable").about(
                "Disables a SecretVersion.\n\nSets the state of the SecretVersion to\nDISABLED.",
            );
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("enable").about(
                "Enables a SecretVersion.\n\nSets the state of the SecretVersion to\nENABLED.",
            );
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets metadata for a SecretVersion.\n\n`projects/*/secrets/*/versions/latest` is an alias to the `latest`\nSecretVersion.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists SecretVersions. This call does not return secret\ndata.");
            versions2 = versions2.subcommand(mcmd);
        }
        secrets1 = secrets1.subcommand(versions2);
        projects0 = projects0.subcommand(secrets1);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_secretmanager1_beta1 as api;

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

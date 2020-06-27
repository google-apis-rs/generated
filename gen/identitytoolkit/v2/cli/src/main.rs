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
        let mut app = App::new("identitytoolkit2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200615")
            .about("The Google Identity Toolkit API lets you use open standards to verify a\n    user\'s identity.")
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
            .about("sub-resources: mfa_enrollment and mfa_sign_in");
        let mut default_supported_idps0 = SubCommand::with_name("default_supported_idps")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all default supported Idps.");
            default_supported_idps0 = default_supported_idps0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: default_supported_idp_configs, inbound_saml_configs, oauth_idp_configs and tenants");
        let mut mfa_enrollment1 = SubCommand::with_name("mfa_enrollment")
            .setting(AppSettings::ColoredHelp)
            .about("methods: finalize, start and withdraw");
        {
            let mcmd = SubCommand::with_name("finalize")
                .about("Finishes enrolling a second factor for the user.");
            mfa_enrollment1 = mfa_enrollment1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start").about("Step one of the MFA enrollment process. In SMS case, this sends an\nSMS verification code to the user.");
            mfa_enrollment1 = mfa_enrollment1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("withdraw").about(
                "Revokes one second factor from the enrolled second factors for an account.",
            );
            mfa_enrollment1 = mfa_enrollment1.subcommand(mcmd);
        }
        let mut mfa_sign_in1 = SubCommand::with_name("mfa_sign_in")
            .setting(AppSettings::ColoredHelp)
            .about("methods: finalize and start");
        {
            let mcmd = SubCommand::with_name("finalize")
                .about("Verifies the MFA challenge and performs sign-in");
            mfa_sign_in1 = mfa_sign_in1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start").about("Sends the MFA challenge");
            mfa_sign_in1 = mfa_sign_in1.subcommand(mcmd);
        }
        let mut default_supported_idp_configs1 =
            SubCommand::with_name("default_supported_idp_configs")
                .setting(AppSettings::ColoredHelp)
                .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about(
                "Create a default supported Idp configuration for an Identity Toolkit\nproject.",
            );
            default_supported_idp_configs1 = default_supported_idp_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Delete a default supported Idp configuration for an Identity Toolkit\nproject.",
            );
            default_supported_idp_configs1 = default_supported_idp_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Retrieve a default supported Idp configuration for an Identity Toolkit\nproject.",
            );
            default_supported_idp_configs1 = default_supported_idp_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "List all default supported Idp configurations for an Identity Toolkit\nproject.",
            );
            default_supported_idp_configs1 = default_supported_idp_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Update a default supported Idp configuration for an Identity Toolkit\nproject.",
            );
            default_supported_idp_configs1 = default_supported_idp_configs1.subcommand(mcmd);
        }
        let mut inbound_saml_configs1 = SubCommand::with_name("inbound_saml_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Create an inbound SAML configuration for an Identity Toolkit project.");
            inbound_saml_configs1 = inbound_saml_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete an inbound SAML configuration for an Identity Toolkit project.");
            inbound_saml_configs1 = inbound_saml_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieve an inbound SAML configuration for an Identity Toolkit project.");
            inbound_saml_configs1 = inbound_saml_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all inbound SAML configurations for an Identity Toolkit project.");
            inbound_saml_configs1 = inbound_saml_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Update an inbound SAML configuration for an Identity Toolkit project.");
            inbound_saml_configs1 = inbound_saml_configs1.subcommand(mcmd);
        }
        let mut oauth_idp_configs1 = SubCommand::with_name("oauth_idp_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Create an Oidc Idp configuration for an Identity Toolkit project.");
            oauth_idp_configs1 = oauth_idp_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete an Oidc Idp configuration for an Identity Toolkit project.");
            oauth_idp_configs1 = oauth_idp_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieve an Oidc Idp configuration for an Identity Toolkit project.");
            oauth_idp_configs1 = oauth_idp_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all Oidc Idp configurations for an Identity Toolkit project.");
            oauth_idp_configs1 = oauth_idp_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Update an Oidc Idp configuration for an Identity Toolkit project.");
            oauth_idp_configs1 = oauth_idp_configs1.subcommand(mcmd);
        }
        let mut tenants1 = SubCommand::with_name("tenants")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Create a tenant. Requires write permission on the Agent project.");
            tenants1 = tenants1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete a tenant. Requires write permission on the Agent project.");
            tenants1 = tenants1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get a tenant. Requires read permission on the Tenant resource.");
            tenants1 = tenants1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. An error is returned if the\nresource does not exist. An empty policy is returned if the resource exists\nbut does not have a policy set on it. Caller must have the right Google IAM\npermission on the resource.");
            tenants1 = tenants1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List tenants under the given agent project. Requires read permission on the\nAgent project.");
            tenants1 = tenants1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Update a tenant. Requires write permission on the Tenant resource.");
            tenants1 = tenants1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy for a resource. If the policy exists, it is\nreplaced. Caller must have the right Google IAM permission on the resource.");
            tenants1 = tenants1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the caller\'s permissions on a resource. An error is returned if the\nresource does not exist. A caller is not required to\nhave Google IAM permission to make this request.");
            tenants1 = tenants1.subcommand(mcmd);
        }
        let mut default_supported_idp_configs2 =
            SubCommand::with_name("default_supported_idp_configs")
                .setting(AppSettings::ColoredHelp)
                .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about(
                "Create a default supported Idp configuration for an Identity Toolkit\nproject.",
            );
            default_supported_idp_configs2 = default_supported_idp_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Delete a default supported Idp configuration for an Identity Toolkit\nproject.",
            );
            default_supported_idp_configs2 = default_supported_idp_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Retrieve a default supported Idp configuration for an Identity Toolkit\nproject.",
            );
            default_supported_idp_configs2 = default_supported_idp_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "List all default supported Idp configurations for an Identity Toolkit\nproject.",
            );
            default_supported_idp_configs2 = default_supported_idp_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Update a default supported Idp configuration for an Identity Toolkit\nproject.",
            );
            default_supported_idp_configs2 = default_supported_idp_configs2.subcommand(mcmd);
        }
        let mut inbound_saml_configs2 = SubCommand::with_name("inbound_saml_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Create an inbound SAML configuration for an Identity Toolkit project.");
            inbound_saml_configs2 = inbound_saml_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete an inbound SAML configuration for an Identity Toolkit project.");
            inbound_saml_configs2 = inbound_saml_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieve an inbound SAML configuration for an Identity Toolkit project.");
            inbound_saml_configs2 = inbound_saml_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all inbound SAML configurations for an Identity Toolkit project.");
            inbound_saml_configs2 = inbound_saml_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Update an inbound SAML configuration for an Identity Toolkit project.");
            inbound_saml_configs2 = inbound_saml_configs2.subcommand(mcmd);
        }
        let mut oauth_idp_configs2 = SubCommand::with_name("oauth_idp_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Create an Oidc Idp configuration for an Identity Toolkit project.");
            oauth_idp_configs2 = oauth_idp_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete an Oidc Idp configuration for an Identity Toolkit project.");
            oauth_idp_configs2 = oauth_idp_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieve an Oidc Idp configuration for an Identity Toolkit project.");
            oauth_idp_configs2 = oauth_idp_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all Oidc Idp configurations for an Identity Toolkit project.");
            oauth_idp_configs2 = oauth_idp_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Update an Oidc Idp configuration for an Identity Toolkit project.");
            oauth_idp_configs2 = oauth_idp_configs2.subcommand(mcmd);
        }
        tenants1 = tenants1.subcommand(oauth_idp_configs2);
        tenants1 = tenants1.subcommand(inbound_saml_configs2);
        tenants1 = tenants1.subcommand(default_supported_idp_configs2);
        projects0 = projects0.subcommand(tenants1);
        projects0 = projects0.subcommand(oauth_idp_configs1);
        projects0 = projects0.subcommand(inbound_saml_configs1);
        projects0 = projects0.subcommand(default_supported_idp_configs1);
        accounts0 = accounts0.subcommand(mfa_sign_in1);
        accounts0 = accounts0.subcommand(mfa_enrollment1);
        app = app.subcommand(projects0);
        app = app.subcommand(default_supported_idps0);
        app = app.subcommand(accounts0);

        Self { app }
    }
}
use google_identitytoolkit2 as api;

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

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
        let mut app = App::new("identitytoolkit3")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20180723")
            .about("Help the third party sites to implement federated login.")
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
        let mut relyingparty0 = SubCommand::with_name("relyingparty")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create_auth_uri, delete_account, download_account, email_link_signin, get_account_info, get_oob_confirmation_code, get_project_config, get_public_keys, get_recaptcha_param, reset_password, send_verification_code, set_account_info, set_project_config, sign_out_user, signup_new_user, upload_account, verify_assertion, verify_custom_token, verify_password and verify_phone_number");
        {
            let mcmd = SubCommand::with_name("create_auth_uri")
                .about("Creates the URI used by the IdP to authenticate the user.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_account").about("Delete user account.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("download_account").about("Batch download user accounts.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("email_link_signin").about("Reset password for a user.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_account_info").about("Returns the account info.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_oob_confirmation_code")
                .about("Get a code for user action confirmation.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get_project_config").about("Get project configuration.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get_public_keys").about("Get token signing public key.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get_recaptcha_param").about("Get recaptcha secure param.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset_password").about("Reset password for a user.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("send_verification_code")
                .about("Send SMS verification code.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("set_account_info").about("Set account info for a user.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("set_project_config").about("Set project configuration.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("sign_out_user").about("Sign out user.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("signup_new_user").about("Signup new user.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upload_account")
                .about("Batch upload existing user accounts.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("verify_assertion")
                .about("Verifies the assertion returned by the IdP.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("verify_custom_token")
                .about("Verifies the developer asserted ID token.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("verify_password")
                .about("Verifies the user entered password.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("verify_phone_number").about("Verifies ownership of a phone number and creates/updates the user account accordingly.");
            relyingparty0 = relyingparty0.subcommand(mcmd);
        }
        app = app.subcommand(relyingparty0);

        Self { app }
    }
}
use google_identitytoolkit3 as api;

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

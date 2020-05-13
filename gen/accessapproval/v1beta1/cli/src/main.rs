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
        let mut app = App::new("accessapproval1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200508")
            .about("An API for controlling access to data by Google personnel.")
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
        let mut folders0 = SubCommand::with_name("folders")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete_access_approval_settings, get_access_approval_settings and update_access_approval_settings");
        {
            let mcmd = SubCommand::with_name("delete_access_approval_settings").about("Deletes the settings associated with a project, folder, or organization.\nThis will have the effect of disabling Access Approval for the project,\nfolder, or organization, but only if all ancestors also have Access\nApproval disabled. If Access Approval is enabled at a higher level of the\nhierarchy, then Access Approval will still be enabled at this level as\nthe settings are inherited.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_access_approval_settings")
                .about("Gets the settings associated with a project, folder, or organization.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_access_approval_settings").about("Updates the settings associated with a project, folder, or organization.\nSettings to update are determined by the value of field_mask.");
            folders0 = folders0.subcommand(mcmd);
        }
        let mut organizations0 = SubCommand::with_name("organizations")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete_access_approval_settings, get_access_approval_settings and update_access_approval_settings");
        {
            let mcmd = SubCommand::with_name("delete_access_approval_settings").about("Deletes the settings associated with a project, folder, or organization.\nThis will have the effect of disabling Access Approval for the project,\nfolder, or organization, but only if all ancestors also have Access\nApproval disabled. If Access Approval is enabled at a higher level of the\nhierarchy, then Access Approval will still be enabled at this level as\nthe settings are inherited.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_access_approval_settings")
                .about("Gets the settings associated with a project, folder, or organization.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_access_approval_settings").about("Updates the settings associated with a project, folder, or organization.\nSettings to update are determined by the value of field_mask.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete_access_approval_settings, get_access_approval_settings and update_access_approval_settings");
        {
            let mcmd = SubCommand::with_name("delete_access_approval_settings").about("Deletes the settings associated with a project, folder, or organization.\nThis will have the effect of disabling Access Approval for the project,\nfolder, or organization, but only if all ancestors also have Access\nApproval disabled. If Access Approval is enabled at a higher level of the\nhierarchy, then Access Approval will still be enabled at this level as\nthe settings are inherited.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_access_approval_settings")
                .about("Gets the settings associated with a project, folder, or organization.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_access_approval_settings").about("Updates the settings associated with a project, folder, or organization.\nSettings to update are determined by the value of field_mask.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut approval_requests1 = SubCommand::with_name("approval_requests")
            .setting(AppSettings::ColoredHelp)
            .about("methods: approve, dismiss, get and list");
        {
            let mcmd = SubCommand::with_name("approve").about("Approves a request and returns the updated ApprovalRequest.\n\nReturns NOT_FOUND if the request does not exist. Returns\nFAILED_PRECONDITION if the request exists but is not in a pending state.");
            approval_requests1 = approval_requests1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("dismiss").about("Dismisses a request. Returns the updated ApprovalRequest.\n\nNOTE: This does not deny access to the resource if another request has been\nmade and approved. It is equivalent in effect to ignoring the request\naltogether.\n\nReturns NOT_FOUND if the request does not exist.\n\nReturns FAILED_PRECONDITION if the request exists but is not in a pending\nstate.");
            approval_requests1 = approval_requests1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Gets an approval request. Returns NOT_FOUND if the request does not exist.",
            );
            approval_requests1 = approval_requests1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists approval requests associated with a project, folder, or organization.\nApproval requests can be filtered by state (pending, active, dismissed).\nThe order is reverse chronological.");
            approval_requests1 = approval_requests1.subcommand(mcmd);
        }
        let mut approval_requests1 = SubCommand::with_name("approval_requests")
            .setting(AppSettings::ColoredHelp)
            .about("methods: approve, dismiss, get and list");
        {
            let mcmd = SubCommand::with_name("approve").about("Approves a request and returns the updated ApprovalRequest.\n\nReturns NOT_FOUND if the request does not exist. Returns\nFAILED_PRECONDITION if the request exists but is not in a pending state.");
            approval_requests1 = approval_requests1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("dismiss").about("Dismisses a request. Returns the updated ApprovalRequest.\n\nNOTE: This does not deny access to the resource if another request has been\nmade and approved. It is equivalent in effect to ignoring the request\naltogether.\n\nReturns NOT_FOUND if the request does not exist.\n\nReturns FAILED_PRECONDITION if the request exists but is not in a pending\nstate.");
            approval_requests1 = approval_requests1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Gets an approval request. Returns NOT_FOUND if the request does not exist.",
            );
            approval_requests1 = approval_requests1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists approval requests associated with a project, folder, or organization.\nApproval requests can be filtered by state (pending, active, dismissed).\nThe order is reverse chronological.");
            approval_requests1 = approval_requests1.subcommand(mcmd);
        }
        let mut approval_requests1 = SubCommand::with_name("approval_requests")
            .setting(AppSettings::ColoredHelp)
            .about("methods: approve, dismiss, get and list");
        {
            let mcmd = SubCommand::with_name("approve").about("Approves a request and returns the updated ApprovalRequest.\n\nReturns NOT_FOUND if the request does not exist. Returns\nFAILED_PRECONDITION if the request exists but is not in a pending state.");
            approval_requests1 = approval_requests1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("dismiss").about("Dismisses a request. Returns the updated ApprovalRequest.\n\nNOTE: This does not deny access to the resource if another request has been\nmade and approved. It is equivalent in effect to ignoring the request\naltogether.\n\nReturns NOT_FOUND if the request does not exist.\n\nReturns FAILED_PRECONDITION if the request exists but is not in a pending\nstate.");
            approval_requests1 = approval_requests1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Gets an approval request. Returns NOT_FOUND if the request does not exist.",
            );
            approval_requests1 = approval_requests1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists approval requests associated with a project, folder, or organization.\nApproval requests can be filtered by state (pending, active, dismissed).\nThe order is reverse chronological.");
            approval_requests1 = approval_requests1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(approval_requests1);
        organizations0 = organizations0.subcommand(approval_requests1);
        folders0 = folders0.subcommand(approval_requests1);
        app = app.subcommand(projects0);
        app = app.subcommand(organizations0);
        app = app.subcommand(folders0);

        Self { app }
    }
}
use google_accessapproval1_beta1 as api;

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

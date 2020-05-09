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
        let mut app = App::new("cloudbilling1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200503")
            .about("Allows developers to manage billing for their Google Cloud Platform projects\n    programmatically.")
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
        let mut billing_accounts0 = SubCommand::with_name("billing_accounts")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a billing account.\nThis method can only be used to create\n[billing subaccounts](https://cloud.google.com/billing/docs/concepts)\nby GCP resellers.\nWhen creating a subaccount, the current authenticated user must have the\n`billing.accounts.update` IAM permission on the master account, which is\ntypically given to billing account\n[administrators](https://cloud.google.com/billing/docs/how-to/billing-access).\nThis method will return an error if the master account has not been\nprovisioned as a reseller account.");
            billing_accounts0 = billing_accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a billing account. The current authenticated user\nmust be a [viewer of the billing\naccount](https://cloud.google.com/billing/docs/how-to/billing-access).");
            billing_accounts0 = billing_accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a billing account.\nThe caller must have the `billing.accounts.getIamPolicy` permission on the\naccount, which is often given to billing account\n[viewers](https://cloud.google.com/billing/docs/how-to/billing-access).");
            billing_accounts0 = billing_accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the billing accounts that the current authenticated user has\npermission to\n[view](https://cloud.google.com/billing/docs/how-to/billing-access).");
            billing_accounts0 = billing_accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a billing account\'s fields.\nCurrently the only field that can be edited is `display_name`.\nThe current authenticated user must have the `billing.accounts.update`\nIAM permission, which is typically given to the\n[administrator](https://cloud.google.com/billing/docs/how-to/billing-access)\nof the billing account.");
            billing_accounts0 = billing_accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy for a billing account. Replaces any existing\npolicy.\nThe caller must have the `billing.accounts.setIamPolicy` permission on the\naccount, which is often given to billing account\n[administrators](https://cloud.google.com/billing/docs/how-to/billing-access).");
            billing_accounts0 = billing_accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Tests the access control policy for a billing account. This method takes\nthe resource and a set of permissions as input and returns the subset of\nthe input permissions that the caller is allowed for that resource.");
            billing_accounts0 = billing_accounts0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_billing_info and update_billing_info");
        {
            let mcmd = SubCommand::with_name("get_billing_info").about("Gets the billing information for a project. The current authenticated user\nmust have [permission to view the\nproject](https://cloud.google.com/docs/permissions-overview#h.bgs0oxofvnoo\n).");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_billing_info").about("Sets or updates the billing account associated with a project. You specify\nthe new billing account by setting the `billing_account_name` in the\n`ProjectBillingInfo` resource to the resource name of a billing account.\nAssociating a project with an open billing account enables billing on the\nproject and allows charges for resource usage. If the project already had a\nbilling account, this method changes the billing account used for resource\nusage charges.\n\n*Note:* Incurred charges that have not yet been reported in the transaction\nhistory of the GCP Console might be billed to the new billing\naccount, even if the charge occurred before the new billing account was\nassigned to the project.\n\nThe current authenticated user must have ownership privileges for both the\n[project](https://cloud.google.com/docs/permissions-overview#h.bgs0oxofvnoo\n) and the [billing\naccount](https://cloud.google.com/billing/docs/how-to/billing-access).\n\nYou can disable billing on the project by setting the\n`billing_account_name` field to empty. This action disassociates the\ncurrent billing account from the project. Any billable activity of your\nin-use services will stop, and your application could stop functioning as\nexpected. Any unbilled charges to date will be billed to the previously\nassociated account. The current authenticated user must be either an owner\nof the project or an owner of the billing account for the project.\n\nNote that associating a project with a *closed* billing account will have\nmuch the same effect as disabling billing on the project: any paid\nresources used by the project will be shut down. Thus, unless you wish to\ndisable billing, you should always call this method with the name of an\n*open* billing account.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut services0 = SubCommand::with_name("services")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists all public cloud services.");
            services0 = services0.subcommand(mcmd);
        }
        let mut projects1 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the projects associated with a billing account. The current\nauthenticated user must have the `billing.resourceAssociations.list` IAM\npermission, which is often given to billing account\n[viewers](https://cloud.google.com/billing/docs/how-to/billing-access).");
            projects1 = projects1.subcommand(mcmd);
        }
        let mut skus1 = SubCommand::with_name("skus")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all publicly available SKUs for a given cloud service.");
            skus1 = skus1.subcommand(mcmd);
        }
        services0 = services0.subcommand(skus1);
        billing_accounts0 = billing_accounts0.subcommand(projects1);
        app = app.subcommand(services0);
        app = app.subcommand(projects0);
        app = app.subcommand(billing_accounts0);

        Self { app }
    }
}
use google_cloudbilling1 as api;

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

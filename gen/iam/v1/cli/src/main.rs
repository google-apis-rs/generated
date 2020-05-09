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
        let mut app = App::new("iam1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200319")
            .about("Manages identity and access control for Google Cloud Platform resources, including the creation of service accounts, which you can use to authenticate to Google and make API calls.")
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
        let mut iam_policies0 = SubCommand::with_name("iam_policies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: lint_policy and query_auditable_services");
        {
            let mcmd = SubCommand::with_name("lint_policy").about("Lints a Cloud IAM policy object or its sub fields. Currently supports\ngoogle.iam.v1.Binding.condition.\n\nEach lint operation consists of multiple lint validation units.\nEach unit inspects the input object in regard to a particular linting\naspect and issues a google.iam.admin.v1.LintResult disclosing the\nresult.\n\nThe set of applicable validation units is determined by the Cloud IAM\nserver and is not configurable.\n\nRegardless of any lint issues or their severities, successful calls to\n`lintPolicy` return an HTTP 200 OK status code.");
            iam_policies0 = iam_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("query_auditable_services").about("Returns a list of services that support service level audit logging\nconfiguration for the given resource.");
            iam_policies0 = iam_policies0.subcommand(mcmd);
        }
        let mut organizations0 = SubCommand::with_name("organizations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: roles");
        let mut permissions0 = SubCommand::with_name("permissions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: query_testable_permissions");
        {
            let mcmd = SubCommand::with_name("query_testable_permissions").about("Lists the permissions testable on a resource.\nA permission is testable if it can be tested for an identity on a resource.");
            permissions0 = permissions0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: roles and service_accounts");
        let mut roles0 = SubCommand::with_name("roles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and query_grantable_roles");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Role definition.");
            roles0 = roles0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the Roles defined on a resource.");
            roles0 = roles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("query_grantable_roles").about("Queries roles that can be granted on a particular resource.\nA role is grantable if it can be used as the role in a binding for a policy\nfor that resource.");
            roles0 = roles0.subcommand(mcmd);
        }
        let mut roles1 = SubCommand::with_name("roles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and undelete");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new Role.");
            roles1 = roles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Soft deletes a role. The role is suspended and cannot be used to create new\nIAM Policy Bindings.\nThe Role will not be included in `ListRoles()` unless `show_deleted` is set\nin the `ListRolesRequest`. The Role contains the deleted boolean set.\nExisting Bindings remains, but are inactive. The Role can be undeleted\nwithin 7 days. After 7 days the Role is deleted and all Bindings associated\nwith the role are removed.");
            roles1 = roles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Role definition.");
            roles1 = roles1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the Roles defined on a resource.");
            roles1 = roles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a Role definition.");
            roles1 = roles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete")
                .about("Undelete a Role, bringing it back in its previous state.");
            roles1 = roles1.subcommand(mcmd);
        }
        let mut roles1 = SubCommand::with_name("roles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and undelete");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new Role.");
            roles1 = roles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Soft deletes a role. The role is suspended and cannot be used to create new\nIAM Policy Bindings.\nThe Role will not be included in `ListRoles()` unless `show_deleted` is set\nin the `ListRolesRequest`. The Role contains the deleted boolean set.\nExisting Bindings remains, but are inactive. The Role can be undeleted\nwithin 7 days. After 7 days the Role is deleted and all Bindings associated\nwith the role are removed.");
            roles1 = roles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Role definition.");
            roles1 = roles1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the Roles defined on a resource.");
            roles1 = roles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a Role definition.");
            roles1 = roles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete")
                .about("Undelete a Role, bringing it back in its previous state.");
            roles1 = roles1.subcommand(mcmd);
        }
        let mut service_accounts1 = SubCommand::with_name("service_accounts")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, disable, enable, get, get_iam_policy, list, patch, set_iam_policy, sign_blob, sign_jwt, test_iam_permissions, undelete and update");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a ServiceAccount\nand returns it.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a ServiceAccount.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("disable").about("DisableServiceAccount is currently in the alpha launch stage.\n\nDisables a ServiceAccount,\nwhich immediately prevents the service account from authenticating and\ngaining access to APIs.\n\nDisabled service accounts can be safely restored by using\nEnableServiceAccount at any point. Deleted service accounts cannot be\nrestored using this method.\n\nDisabling a service account that is bound to VMs, Apps, Functions, or\nother jobs will cause those jobs to lose access to resources if they are\nusing the disabled service account.\n\nTo improve reliability of your services and avoid unexpected outages, it\nis recommended to first disable a service account rather than delete it.\nAfter disabling the service account, wait at least 24 hours to verify there\nare no unintended consequences, and then delete the service account.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("enable").about("EnableServiceAccount is currently in the alpha launch stage.\n\n Restores a disabled ServiceAccount\n that has been manually disabled by using DisableServiceAccount. Service\n accounts that have been disabled by other means or for other reasons,\n such as abuse, cannot be restored using this method.\n\n EnableServiceAccount will have no effect on a service account that is\n not disabled.  Enabling an already enabled service account will have no\n effect.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a ServiceAccount.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Returns the Cloud IAM access control policy for a\nServiceAccount.\n\nNote: Service accounts are both\n[resources and\nidentities](/iam/docs/service-accounts#service_account_permissions). This\nmethod treats the service account as a resource. It returns the Cloud IAM\npolicy that reflects what members have access to the service account.\n\nThis method does not return what resources the service account has access\nto. To see if a service account has access to a resource, call the\n`getIamPolicy` method on the target resource. For example, to view grants\nfor a project, call the\n[projects.getIamPolicy](/resource-manager/reference/rest/v1/projects/getIamPolicy)\nmethod.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists ServiceAccounts for a project.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches a ServiceAccount.\n\nCurrently, only the following fields are updatable:\n`display_name` and `description`.\n\nOnly fields specified in the request are guaranteed to be returned in\nthe response. Other fields in the response may be empty.\n\nNote: The field mask is required.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the Cloud IAM access control policy for a\nServiceAccount.\n\nNote: Service accounts are both\n[resources and\nidentities](/iam/docs/service-accounts#service_account_permissions). This\nmethod treats the service account as a resource. Use it to grant members\naccess to the service account, such as when they need to impersonate it.\n\nThis method does not grant the service account access to other resources,\nsuch as projects. To grant a service account access to resources, include\nthe service account in the Cloud IAM policy for the desired resource, then\ncall the appropriate `setIamPolicy` method on the target resource. For\nexample, to grant a service account access to a project, call the\n[projects.setIamPolicy](/resource-manager/reference/rest/v1/projects/setIamPolicy)\nmethod.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("sign_blob").about("**Note**: This method is in the process of being deprecated. Call the\n[`signBlob()`](/iam/credentials/reference/rest/v1/projects.serviceAccounts/signBlob)\nmethod of the Cloud IAM Service Account Credentials API instead.\n\nSigns a blob using a service account\'s system-managed private key.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("sign_jwt").about("**Note**: This method is in the process of being deprecated. Call the\n[`signJwt()`](/iam/credentials/reference/rest/v1/projects.serviceAccounts/signJwt)\nmethod of the Cloud IAM Service Account Credentials API instead.\n\nSigns a JWT using a service account\'s system-managed private key.\n\nIf no expiry time (`exp`) is provided in the `SignJwtRequest`, IAM sets an\nan expiry time of one hour by default. If you request an expiry time of\nmore than one hour, the request will fail.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Tests the specified permissions against the IAM access control policy\nfor a ServiceAccount.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete").about("Restores a deleted ServiceAccount.\nThis is to be used as an action of last resort.  A service account may\nnot always be restorable.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Note: This method is in the process of being deprecated. Use\nPatchServiceAccount instead.\n\nUpdates a ServiceAccount.\n\nCurrently, only the following fields are updatable:\n`display_name` and `description`.");
            service_accounts1 = service_accounts1.subcommand(mcmd);
        }
        let mut keys2 = SubCommand::with_name("keys")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and upload");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a ServiceAccountKey\nand returns it.");
            keys2 = keys2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a ServiceAccountKey.");
            keys2 = keys2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the ServiceAccountKey\nby key id.");
            keys2 = keys2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists ServiceAccountKeys.");
            keys2 = keys2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upload").about("Upload public key for a given service account.\nThis rpc will create a\nServiceAccountKey that has the\nprovided public key and returns it.");
            keys2 = keys2.subcommand(mcmd);
        }
        service_accounts1 = service_accounts1.subcommand(keys2);
        projects0 = projects0.subcommand(service_accounts1);
        projects0 = projects0.subcommand(roles1);
        organizations0 = organizations0.subcommand(roles1);
        app = app.subcommand(roles0);
        app = app.subcommand(projects0);
        app = app.subcommand(permissions0);
        app = app.subcommand(organizations0);
        app = app.subcommand(iam_policies0);

        Self { app }
    }
}
use google_iam1 as api;

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

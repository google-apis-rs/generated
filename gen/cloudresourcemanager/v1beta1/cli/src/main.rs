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
        let mut app = App::new("cloudresourcemanager1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200504")
            .about("Creates, reads, and updates metadata for Google Cloud Platform resource containers.")
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
        let mut organizations0 = SubCommand::with_name("organizations")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: get, get_iam_policy, list, set_iam_policy, test_iam_permissions and update");
        {
            let mcmd = SubCommand::with_name("get").about(
                "Fetches an Organization resource identified by the specified resource name.",
            );
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for an Organization resource. May be empty\nif no such policy or resource exists. The `resource` field should be the\norganization\'s resource name, e.g. \"organizations/123\".");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Organization resources that are visible to the user and satisfy\nthe specified filter. This method returns Organizations in an unspecified\norder. New Organizations do not necessarily appear at the end of the list.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on an Organization resource. Replaces any\nexisting policy. The `resource` field should be the organization\'s resource\nname, e.g. \"organizations/123\".");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified Organization.\nThe `resource` field should be the organization\'s resource name,\ne.g. \"organizations/123\".");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about(
                "Updates an Organization resource identified by the specified resource name.",
            );
            organizations0 = organizations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_ancestry, get_iam_policy, list, set_iam_policy, test_iam_permissions, undelete and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Project resource.\n\nInitially, the Project resource is owned by its creator exclusively.\nThe creator can later grant permission to others to read or update the\nProject.\n\nSeveral APIs are activated automatically for the Project, including\nGoogle Cloud Storage. The parent is identified by a specified\nResourceId, which must include both an ID and a type, such as\nproject, folder, or organization.\n\nThis method does not associate the new project with a billing account.\nYou can set or update the billing account associated with a project using\nthe [`projects.updateBillingInfo`]\n(/billing/reference/rest/v1/projects/updateBillingInfo) method.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Marks the Project identified by the specified\n`project_id` (for example, `my-project-123`) for deletion.\nThis method will only affect the Project if it has a lifecycle state of\nACTIVE.\n\nThis method changes the Project\'s lifecycle state from\nACTIVE\nto DELETE_REQUESTED.\nThe deletion starts at an unspecified time, at which point the project is\nno longer accessible.\n\nUntil the deletion completes, you can check the lifecycle state\nchecked by retrieving the Project with GetProject,\nand the Project remains visible to ListProjects.\nHowever, you cannot update the project.\n\nAfter the deletion completes, the Project is not retrievable by\nthe  GetProject\nand ListProjects\nmethods.\n\nThe caller must have modify permissions for this Project.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the Project identified by the specified\n`project_id` (for example, `my-project-123`).\n\nThe caller must have read permissions for this Project.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_ancestry").about("Gets a list of ancestors in the resource hierarchy for the Project\nidentified by the specified `project_id` (for example, `my-project-123`).\n\nThe caller must have read permissions for this Project.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Returns the IAM access control policy for the specified Project.\nPermission is denied if the policy or the resource does not exist.\n\nFor additional information about resource structure and identification,\nsee [Resource Names](/apis/design/resource_names).");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Projects that the caller has the `resourcemanager.projects.get`\npermission on and satisfy the specified filter.\n\nThis method returns Projects in an unspecified order.\nThis method is eventually consistent with project mutations; this means\nthat a newly created project may not appear in the results or recent\nupdates to an existing project may not be reflected in the results. To\nretrieve the latest state of a project, use the\nGetProject method.\n\nNOTE: If the request filter contains a `parent.type` and `parent.id` and\nthe caller has the `resourcemanager.projects.list` permission on the\nparent, the results will be drawn from an alternate index which provides\nmore consistent results. In future versions of this API, this List method\nwill be split into List and Search to properly capture the behavorial\ndifference.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the IAM access control policy for the specified Project. Overwrites\nany existing policy.\n\nThe following constraints apply when using `setIamPolicy()`:\n\n+ Project does not support `allUsers` and `allAuthenticatedUsers` as\n`members` in a `Binding` of a `Policy`.\n\n+ The owner role can be granted to a `user`, `serviceAccount`, or a group\nthat is part of an organization. For example,\ngroup@myownpersonaldomain.com could be added as an owner to a project in\nthe myownpersonaldomain.com organization, but not the examplepetstore.com\norganization.\n\n+ Service accounts can be made owners of a project directly\nwithout any restrictions. However, to be added as an owner, a user must be\ninvited via Cloud Platform console and must accept the invitation.\n\n+ A user cannot be granted the owner role using `setIamPolicy()`. The user\nmust be granted the owner role using the Cloud Platform Console and must\nexplicitly accept the invitation.\n\n+ Invitations to grant the owner role cannot be sent using\n`setIamPolicy()`; they must be sent only using the Cloud Platform Console.\n\n+ Membership changes that leave the project without any owners that have\naccepted the Terms of Service (ToS) will be rejected.\n\n+ If the project is not part of an organization, there must be at least\none owner who has accepted the Terms of Service (ToS) agreement in the\npolicy. Calling `setIamPolicy()` to remove the last ToS-accepted owner\nfrom the policy will fail. This restriction also applies to legacy\nprojects that no longer have owners who have accepted the ToS. Edits to\nIAM policies will be rejected until the lack of a ToS-accepting owner is\nrectified.\n\n+ This method will replace the existing policy, and cannot be used to\nappend additional IAM settings.\n\nNote: Removing service accounts from policies or changing their roles\ncan render services completely inoperable. It is important to understand\nhow the service account is being used before removing or updating its\nroles.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified Project.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete").about("Restores the Project identified by the specified\n`project_id` (for example, `my-project-123`).\nYou can only use this method for a Project that has a lifecycle state of\nDELETE_REQUESTED.\nAfter deletion starts, the Project cannot be restored.\n\nThe caller must have modify permissions for this Project.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the attributes of the Project identified by the specified\n`project_id` (for example, `my-project-123`).\n\nThe caller must have modify permissions for this Project.");
            projects0 = projects0.subcommand(mcmd);
        }
        app = app.subcommand(projects0);
        app = app.subcommand(organizations0);

        Self { app }
    }
}
use google_cloudresourcemanager1_beta1 as api;

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

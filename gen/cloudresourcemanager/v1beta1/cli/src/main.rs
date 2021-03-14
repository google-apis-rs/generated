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
            .version("0.1.0-20210309")
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
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for an Organization resource. May be empty if no such policy or resource exists. The `resource` field should be the organization\'s resource name, e.g. \"organizations/123\".");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Organization resources that are visible to the user and satisfy the specified filter. This method returns Organizations in an unspecified order. New Organizations do not necessarily appear at the end of the list.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on an Organization resource. Replaces any existing policy. The `resource` field should be the organization\'s resource name, e.g. \"organizations/123\".");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified Organization. The `resource` field should be the organization\'s resource name, e.g. \"organizations/123\".");
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
            let mcmd = SubCommand::with_name("create").about("Creates a Project resource. Initially, the Project resource is owned by its creator exclusively. The creator can later grant permission to others to read or update the Project. Several APIs are activated automatically for the Project, including Google Cloud Storage. The parent is identified by a specified ResourceId, which must include both an ID and a type, such as project, folder, or organization. This method does not associate the new project with a billing account. You can set or update the billing account associated with a project using the [`projects.updateBillingInfo`] (/billing/reference/rest/v1/projects/updateBillingInfo) method.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Marks the Project identified by the specified `project_id` (for example, `my-project-123`) for deletion. This method will only affect the Project if it has a lifecycle state of ACTIVE. This method changes the Project\'s lifecycle state from ACTIVE to DELETE_REQUESTED. The deletion starts at an unspecified time, at which point the project is no longer accessible. Until the deletion completes, you can check the lifecycle state checked by retrieving the Project with GetProject, and the Project remains visible to ListProjects. However, you cannot update the project. After the deletion completes, the Project is not retrievable by the GetProject and ListProjects methods. The caller must have delete permissions for this Project.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the Project identified by the specified `project_id` (for example, `my-project-123`). The caller must have read permissions for this Project.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_ancestry").about("Gets a list of ancestors in the resource hierarchy for the Project identified by the specified `project_id` (for example, `my-project-123`). The caller must have read permissions for this Project.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Returns the IAM access control policy for the specified Project. Permission is denied if the policy or the resource does not exist. For additional information about resource structure and identification, see [Resource Names](/apis/design/resource_names).");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Projects that the caller has the `resourcemanager.projects.get` permission on and satisfy the specified filter. This method returns Projects in an unspecified order. This method is eventually consistent with project mutations; this means that a newly created project may not appear in the results or recent updates to an existing project may not be reflected in the results. To retrieve the latest state of a project, use the GetProject method. NOTE: If the request filter contains a `parent.type` and `parent.id` and the caller has the `resourcemanager.projects.list` permission on the parent, the results will be drawn from an alternate index which provides more consistent results. In future versions of this API, this List method will be split into List and Search to properly capture the behavioral difference.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the IAM access control policy for the specified Project. CAUTION: This method will replace the existing policy, and cannot be used to append additional IAM settings. NOTE: Removing service accounts from policies or changing their roles can render services completely inoperable. It is important to understand how the service account is being used before removing or updating its roles. The following constraints apply when using `setIamPolicy()`: + Project does not support `allUsers` and `allAuthenticatedUsers` as `members` in a `Binding` of a `Policy`. + The owner role can be granted to a `user`, `serviceAccount`, or a group that is part of an organization. For example, group@myownpersonaldomain.com could be added as an owner to a project in the myownpersonaldomain.com organization, but not the examplepetstore.com organization. + Service accounts can be made owners of a project directly without any restrictions. However, to be added as an owner, a user must be invited via Cloud Platform console and must accept the invitation. + A user cannot be granted the owner role using `setIamPolicy()`. The user must be granted the owner role using the Cloud Platform Console and must explicitly accept the invitation. + Invitations to grant the owner role cannot be sent using `setIamPolicy()`; they must be sent only using the Cloud Platform Console. + Membership changes that leave the project without any owners that have accepted the Terms of Service (ToS) will be rejected. + If the project is not part of an organization, there must be at least one owner who has accepted the Terms of Service (ToS) agreement in the policy. Calling `setIamPolicy()` to remove the last ToS-accepted owner from the policy will fail. This restriction also applies to legacy projects that no longer have owners who have accepted the ToS. Edits to IAM policies will be rejected until the lack of a ToS-accepting owner is rectified. Authorization requires the Google IAM permission `resourcemanager.projects.setIamPolicy` on the project");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns permissions that a caller has on the specified Project.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete").about("Restores the Project identified by the specified `project_id` (for example, `my-project-123`). You can only use this method for a Project that has a lifecycle state of DELETE_REQUESTED. After deletion starts, the Project cannot be restored. The caller must have undelete permissions for this Project.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the attributes of the Project identified by the specified `project_id` (for example, `my-project-123`). The caller must have modify permissions for this Project.");
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

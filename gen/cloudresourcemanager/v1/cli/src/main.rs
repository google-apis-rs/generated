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
        let mut app = App::new("cloudresourcemanager1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190830")
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
        let mut folders0 = SubCommand::with_name("folders")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: clear_org_policy, get_effective_org_policy, get_org_policy, list_available_org_policy_constraints, list_org_policies and set_org_policy");
        {
            let mcmd = SubCommand::with_name("clear_org_policy")
                .about("Clears a `Policy` from a resource.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_effective_org_policy").about("Gets the effective `Policy` on a resource. This is the result of merging\n`Policies` in the resource hierarchy. The returned `Policy` will not have\nan `etag`set because it is a computed `Policy` across multiple resources.\nSubtrees of Resource Manager resource hierarchy with \'under:\' prefix will\nnot be expanded.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_org_policy").about("Gets a `Policy` on a resource.\n\nIf no `Policy` is set on the resource, a `Policy` is returned with default\nvalues including `POLICY_TYPE_NOT_SET` for the `policy_type oneof`. The\n`etag` value can be used with `SetOrgPolicy()` to create or update a\n`Policy` during read-modify-write.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_available_org_policy_constraints")
                .about("Lists `Constraints` that could be applied on the specified resource.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_org_policies")
                .about("Lists all the `Policies` set for a particular resource.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_org_policy").about("Updates the specified `Policy` on the resource. Creates a new `Policy` for\nthat `Constraint` on the resource if one does not exist.\n\nNot supplying an `etag` on the request `Policy` results in an unconditional\nwrite of the `Policy`.");
            folders0 = folders0.subcommand(mcmd);
        }
        let mut liens0 = SubCommand::with_name("liens")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Create a Lien which applies to the resource denoted by the `parent` field.\n\nCallers of this method will require permission on the `parent` resource.\nFor example, applying to `projects/1234` requires permission\n`resourcemanager.projects.updateLiens`.\n\nNOTE: Some resources may limit the number of Liens which may be applied.");
            liens0 = liens0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a Lien by `name`.\n\nCallers of this method will require permission on the `parent` resource.\nFor example, a Lien with a `parent` of `projects/1234` requires permission\n`resourcemanager.projects.updateLiens`.");
            liens0 = liens0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve a Lien by `name`.\n\nCallers of this method will require permission on the `parent` resource.\nFor example, a Lien with a `parent` of `projects/1234` requires permission\nrequires permission `resourcemanager.projects.get` or\n`resourcemanager.projects.updateLiens`.");
            liens0 = liens0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List all Liens applied to the `parent` resource.\n\nCallers of this method will require permission on the `parent` resource.\nFor example, a Lien with a `parent` of `projects/1234` requires permission\n`resourcemanager.projects.get`.");
            liens0 = liens0.subcommand(mcmd);
        }
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut organizations0 = SubCommand::with_name("organizations")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: clear_org_policy, get, get_effective_org_policy, get_iam_policy, get_org_policy, list_available_org_policy_constraints, list_org_policies, search, set_iam_policy, set_org_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("clear_org_policy")
                .about("Clears a `Policy` from a resource.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Fetches an Organization resource identified by the specified resource name.",
            );
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_effective_org_policy").about("Gets the effective `Policy` on a resource. This is the result of merging\n`Policies` in the resource hierarchy. The returned `Policy` will not have\nan `etag`set because it is a computed `Policy` across multiple resources.\nSubtrees of Resource Manager resource hierarchy with \'under:\' prefix will\nnot be expanded.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for an Organization resource. May be empty\nif no such policy or resource exists. The `resource` field should be the\norganization\'s resource name, e.g. \"organizations/123\".\n\nAuthorization requires the Google IAM permission\n`resourcemanager.organizations.getIamPolicy` on the specified organization");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_org_policy").about("Gets a `Policy` on a resource.\n\nIf no `Policy` is set on the resource, a `Policy` is returned with default\nvalues including `POLICY_TYPE_NOT_SET` for the `policy_type oneof`. The\n`etag` value can be used with `SetOrgPolicy()` to create or update a\n`Policy` during read-modify-write.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_available_org_policy_constraints")
                .about("Lists `Constraints` that could be applied on the specified resource.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_org_policies")
                .about("Lists all the `Policies` set for a particular resource.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search").about("Searches Organization resources that are visible to the user and satisfy\nthe specified filter. This method returns Organizations in an unspecified\norder. New Organizations do not necessarily appear at the end of the\nresults.\n\nSearch will only return organizations on which the user has the permission\n`resourcemanager.organizations.get`");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on an Organization resource. Replaces any\nexisting policy. The `resource` field should be the organization\'s resource\nname, e.g. \"organizations/123\".\n\nAuthorization requires the Google IAM permission\n`resourcemanager.organizations.setIamPolicy` on the specified organization");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_org_policy").about("Updates the specified `Policy` on the resource. Creates a new `Policy` for\nthat `Constraint` on the resource if one does not exist.\n\nNot supplying an `etag` on the request `Policy` results in an unconditional\nwrite of the `Policy`.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified Organization.\nThe `resource` field should be the organization\'s resource name,\ne.g. \"organizations/123\".\n\nThere are no permissions required for making this API call.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: clear_org_policy, create, delete, get, get_ancestry, get_effective_org_policy, get_iam_policy, get_org_policy, list, list_available_org_policy_constraints, list_org_policies, set_iam_policy, set_org_policy, test_iam_permissions, undelete and update");
        {
            let mcmd = SubCommand::with_name("clear_org_policy")
                .about("Clears a `Policy` from a resource.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Request that a new Project be created. The result is an Operation which\ncan be used to track the creation process. This process usually takes a few\nseconds, but can sometimes take much longer. The tracking Operation is\nautomatically deleted after a few hours, so there is no need to call\nDeleteOperation.\n\nAuthorization requires the Google IAM permission\n`resourcemanager.projects.create` on the specified parent for the new\nproject. The parent is identified by a specified ResourceId,\nwhich must include both an ID and a type, such as organization.\n\nThis method does not associate the new project with a billing account.\nYou can set or update the billing account associated with a project using\nthe [`projects.updateBillingInfo`]\n(/billing/reference/rest/v1/projects/updateBillingInfo) method.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Marks the Project identified by the specified\n`project_id` (for example, `my-project-123`) for deletion.\nThis method will only affect the Project if it has a lifecycle state of\nACTIVE.\n\nThis method changes the Project\'s lifecycle state from\nACTIVE\nto DELETE_REQUESTED.\nThe deletion starts at an unspecified time,\nat which point the Project is no longer accessible.\n\nUntil the deletion completes, you can check the lifecycle state\nchecked by retrieving the Project with GetProject,\nand the Project remains visible to ListProjects.\nHowever, you cannot update the project.\n\nAfter the deletion completes, the Project is not retrievable by\nthe  GetProject and\nListProjects methods.\n\nThe caller must have modify permissions for this Project.");
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
            let mcmd = SubCommand::with_name("get_effective_org_policy").about("Gets the effective `Policy` on a resource. This is the result of merging\n`Policies` in the resource hierarchy. The returned `Policy` will not have\nan `etag`set because it is a computed `Policy` across multiple resources.\nSubtrees of Resource Manager resource hierarchy with \'under:\' prefix will\nnot be expanded.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Returns the IAM access control policy for the specified Project.\nPermission is denied if the policy or the resource does not exist.\n\nAuthorization requires the Google IAM permission\n`resourcemanager.projects.getIamPolicy` on the project.\n\nFor additional information about resource structure and identification,\nsee [Resource Names](/apis/design/resource_names).");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_org_policy").about("Gets a `Policy` on a resource.\n\nIf no `Policy` is set on the resource, a `Policy` is returned with default\nvalues including `POLICY_TYPE_NOT_SET` for the `policy_type oneof`. The\n`etag` value can be used with `SetOrgPolicy()` to create or update a\n`Policy` during read-modify-write.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Projects that the caller has the `resourcemanager.projects.get`\npermission on and satisfy the specified filter.\n\nThis method returns Projects in an unspecified order.\nThis method is eventually consistent with project mutations; this means\nthat a newly created project may not appear in the results or recent\nupdates to an existing project may not be reflected in the results. To\nretrieve the latest state of a project, use the\nGetProject method.\n\nNOTE: If the request filter contains a `parent.type` and `parent.id` and\nthe caller has the `resourcemanager.projects.list` permission on the\nparent, the results will be drawn from an alternate index which provides\nmore consistent results. In future versions of this API, this List method\nwill be split into List and Search to properly capture the behavorial\ndifference.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_available_org_policy_constraints")
                .about("Lists `Constraints` that could be applied on the specified resource.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_org_policies")
                .about("Lists all the `Policies` set for a particular resource.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the IAM access control policy for the specified Project. Overwrites\nany existing policy.\n\nThe following constraints apply when using `setIamPolicy()`:\n\n+ Project does not support `allUsers` and `allAuthenticatedUsers` as\n`members` in a `Binding` of a `Policy`.\n\n+ The owner role can be granted to a `user`, `serviceAccount`, or a group\nthat is part of an organization. For example,\ngroup@myownpersonaldomain.com could be added as an owner to a project in\nthe myownpersonaldomain.com organization, but not the examplepetstore.com\norganization.\n\n+ Service accounts can be made owners of a project directly\nwithout any restrictions. However, to be added as an owner, a user must be\ninvited via Cloud Platform console and must accept the invitation.\n\n+ A user cannot be granted the owner role using `setIamPolicy()`. The user\nmust be granted the owner role using the Cloud Platform Console and must\nexplicitly accept the invitation.\n\n+ You can only grant ownership of a project to a member by using the\nGCP Console. Inviting a member will deliver an invitation email that\nthey must accept. An invitation email is not generated if you are\ngranting a role other than owner, or if both the member you are inviting\nand the project are part of your organization.\n\n+ Membership changes that leave the project without any owners that have\naccepted the Terms of Service (ToS) will be rejected.\n\n+ If the project is not part of an organization, there must be at least\none owner who has accepted the Terms of Service (ToS) agreement in the\npolicy. Calling `setIamPolicy()` to remove the last ToS-accepted owner\nfrom the policy will fail. This restriction also applies to legacy\nprojects that no longer have owners who have accepted the ToS. Edits to\nIAM policies will be rejected until the lack of a ToS-accepting owner is\nrectified.\n\n+ This method will replace the existing policy, and cannot be used to\nappend additional IAM settings.\n\nNote: Removing service accounts from policies or changing their roles\ncan render services completely inoperable. It is important to understand\nhow the service account is being used before removing or updating its\nroles.\n\nAuthorization requires the Google IAM permission\n`resourcemanager.projects.setIamPolicy` on the project");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_org_policy").about("Updates the specified `Policy` on the resource. Creates a new `Policy` for\nthat `Constraint` on the resource if one does not exist.\n\nNot supplying an `etag` on the request `Policy` results in an unconditional\nwrite of the `Policy`.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified Project.\n\nThere are no permissions required for making this API call.");
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
        app = app.subcommand(operations0);
        app = app.subcommand(liens0);
        app = app.subcommand(folders0);

        Self { app }
    }
}
use google_cloudresourcemanager1 as api;

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

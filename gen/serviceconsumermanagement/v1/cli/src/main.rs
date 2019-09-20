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
        let mut app = App::new("serviceconsumermanagement1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190918")
            .about("Manages the service consumers of a Service Infrastructure service.")
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
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is\nno longer interested in the operation result. It does not cancel the\noperation. If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the\nserver doesn\'t support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut services0 = SubCommand::with_name("services")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search")
                .about("Search tenancy units for a managed service.");
            services0 = services0.subcommand(mcmd);
        }
        let mut tenancy_units1 = SubCommand::with_name("tenancy_units")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_project, apply_project_config, attach_project, create, delete, delete_project, list, remove_project and undelete_project");
        {
            let mcmd = SubCommand::with_name("add_project").about("Add a new tenant project to the tenancy unit.\nThere can be a maximum of 512 tenant projects in a tenancy unit.\nIf there are previously failed `AddTenantProject` calls, you might need to\ncall `RemoveTenantProject` first to resolve them before you can make\nanother call to `AddTenantProject` with the same tag.\nOperation<response: Empty>.");
            tenancy_units1 = tenancy_units1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("apply_project_config").about("Apply a configuration to an existing tenant project.\nThis project must exist in an active state and have the original owner\naccount. The caller must have permission to add a project to the given\ntenancy unit. The configuration is applied, but any existing settings on\nthe project aren\'t modified.\nSpecified policy bindings are applied. Existing bindings aren\'t modified.\nSpecified services are activated. No service is deactivated.\nIf specified, new billing configuration is applied.\nOmit a billing configuration to keep the existing one.\nA service account in the project is created if previously non existed.\nSpecified labels will be appended to tenant project, note that the value of\nexisting label key will be updated if the same label key is requested.\nThe specified folder is ignored, as moving a tenant project to a different\nfolder isn\'t supported.\nThe operation fails if any of the steps fail, but no rollback of already\napplied configuration changes is attempted.\nOperation<response: Empty>.");
            tenancy_units1 = tenancy_units1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("attach_project").about("Attach an existing project to the tenancy unit as a new tenant\nresource. The project could either be the tenant project reserved by\ncalling `AddTenantProject` under a tenancy unit of a service producer\'s\nproject of a managed service, or from a separate project.\nThe caller is checked against a set of permissions as if calling\n`AddTenantProject` on the same service consumer.\nTo trigger the attachment, the targeted tenant project must be in a\nfolder. Make sure the ServiceConsumerManagement service account is\nthe owner of that project. These two requirements are already met\nif the project is reserved by calling `AddTenantProject`.\nOperation<response: Empty>.");
            tenancy_units1 = tenancy_units1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a tenancy unit with no tenant resources.\nIf tenancy unit already exists, it will be returned,\nhowever, in this case, returned TenancyUnit does not have tenant_resources\nfield set and ListTenancyUnit has to be used to get a complete\nTenancyUnit with all fields populated.");
            tenancy_units1 = tenancy_units1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a tenancy unit. Before you delete the tenancy unit, there should be\nno tenant resources in it that aren\'t in a DELETED state.\nOperation<response: Empty>.");
            tenancy_units1 = tenancy_units1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_project").about("Deletes the specified project resource identified by a tenant resource tag.\nThe mothod removes a project lien with a \'TenantManager\' origin if that was\nadded. It will then attempt to delete the project. If that operation fails,\nthis method also fails.\nAfter the project has been deleted, the tenant resource state is set to\nDELETED.  To permanently remove resource metadata, call the\n`RemoveTenantProject` method.\nNew resources with the same tag can\'t be added if there are existing\nresources in a DELETED state.\nOperation<response: Empty>.");
            tenancy_units1 = tenancy_units1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Find the tenancy unit for a managed service and service consumer.\nThis method shouldn\'t be used in a service producer\'s runtime path, for\nexample to find the tenant project number when creating VMs. Service\nproducers must persist the tenant project\'s information after the project\nis created.");
            tenancy_units1 = tenancy_units1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_project").about("Removes the specified project resource identified by a tenant resource tag.\nThe method removes the project lien with \'TenantManager\' origin if that\nwas added. It then attempts to delete the project. If that operation\nfails, this method also fails.\nCalls to remove already removed or non-existent tenant project succeed.\nAfter the project has been deleted, or if was already in a DELETED state,\nresource metadata is permanently removed from the tenancy unit.\nOperation<response: Empty>.");
            tenancy_units1 = tenancy_units1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete_project").about("Attempts to undelete a previously deleted tenant project. The project must\nbe in a DELETED state.\nThere are no guarantees that an undeleted project will be in\na fully restored and functional state. Call the `ApplyTenantProjectConfig`\nmethod to update its configuration and then validate all managed service\nresources.\nOperation<response: Empty>.");
            tenancy_units1 = tenancy_units1.subcommand(mcmd);
        }
        services0 = services0.subcommand(tenancy_units1);
        app = app.subcommand(services0);
        app = app.subcommand(operations0);

        Self { app }
    }
}
use google_serviceconsumermanagement1 as api;

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

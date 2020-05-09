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
        let mut app = App::new("bigqueryreservation1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200410")
            .about("A service to modify your BigQuery flat-rate reservations.")
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
            .about("methods: delete and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is\nno longer interested in the operation result. It does not cancel the\noperation. If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the\nserver doesn\'t support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_bi_reservation, search_assignments and update_bi_reservation");
        {
            let mcmd =
                SubCommand::with_name("get_bi_reservation").about("Retrieves a BI reservation.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_assignments").about("Looks up assignments for a specified resource for a particular region.\nIf the request is about a project:\n  1) Assignments created on the project will be returned if they exist.\n  2) Otherwise assignments created on the closest ancestor will be\n  returned. 3) Assignments for different JobTypes will all be returned.\nSame logic applies if the request is about a folder.\nIf the request is about an organization, then assignments created on the\norganization will be returned (organization doesn\'t have ancestors).\nComparing to ListAssignments, there are some behavior\ndifferences:\n  1) permission on the assignee will be verified in this API.\n  2) Hierarchy lookup (project->folder->organization) happens in this API.\n  3) Parent here is projects/*/locations/*, instead of\n  projects/*/locations/*reservations/*.\nNote \"-\" cannot be used for projects\nnor locations.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_bi_reservation").about("Updates a BI reservation.\nOnly fields specified in the field_mask are updated.\nSingleton BI reservation always exists with default size 0.\nIn order to reserve BI capacity it needs to be updated to an amount\ngreater than 0. In order to release BI capacity reservation size\nmust be set to 0.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut capacity_commitments2 = SubCommand::with_name("capacity_commitments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, merge, patch and split");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new capacity commitment resource.");
            capacity_commitments2 = capacity_commitments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a capacity commitment. Attempting to delete capacity commitment\nbefore its commitment_end_time will fail with the error code\n`google.rpc.Code.FAILED_PRECONDITION`.");
            capacity_commitments2 = capacity_commitments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns information about the capacity commitment.");
            capacity_commitments2 = capacity_commitments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the capacity commitments for the admin project.");
            capacity_commitments2 = capacity_commitments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("merge").about("Merges capacity commitments of the same plan into one. Resulting capacity\ncommitment has the longer commitment_end_time out of the two. Attempting to\nmerge capacity commitments of different plan will fail with the error code\n`google.rpc.Code.FAILED_PRECONDITION`.");
            capacity_commitments2 = capacity_commitments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing capacity commitment.\n\nOnly plan and renewal_plan fields can be updated.\nPlan can only be changed to a plan of a longer commitment period.\nAttempting to change to a plan with shorter commitment period will fail\nwith the error code `google.rpc.Code.FAILED_PRECONDITION`.");
            capacity_commitments2 = capacity_commitments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("split").about("Splits capacity commitment to two commitments of the same plan and\ncommitment_end_time. A common use case to do that is to perform a downgrade\ne.g., in order to downgrade from 10000 slots to 8000, one might split 10000\ncapacity commitment to 2000 and 8000, change the plan of the first one to\nflex and then delete it.");
            capacity_commitments2 = capacity_commitments2.subcommand(mcmd);
        }
        let mut reservations2 = SubCommand::with_name("reservations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new reservation resource.");
            reservations2 = reservations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a reservation.\nReturns `google.rpc.Code.FAILED_PRECONDITION` when reservation has\nassignments.");
            reservations2 = reservations2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Returns information about the reservation.");
            reservations2 = reservations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the reservations for the project in the specified location.");
            reservations2 = reservations2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates an existing reservation resource.");
            reservations2 = reservations2.subcommand(mcmd);
        }
        let mut assignments3 = SubCommand::with_name("assignments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, list and r#move");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an object which allows the given project to submit jobs\nof a certain type using slots from the specified reservation. Currently a\nresource (project, folder, organization) can only have one assignment per\n{job_type, location}, and that reservation will be used for all jobs of the\nmatching type. Within the organization, different assignments can be\ncreated on projects, folders or organization level. During query execution,\nthe assignment is looked up at the project, folder and organization levels\nin that order. The first assignment found is applied to the query. When\ncreating assignments, it does not matter if other assignments exist at\nhigher levels. E.g: organizationA contains project1, project2. Assignments\nfor organizationA, project1 and project2 could all be created, mapping to\nthe same or different reservations.\nReturns `google.rpc.Code.PERMISSION_DENIED` if user does not have\n\'bigquery.admin\' permissions on the project using the reservation\nand the project that owns this reservation.\nReturns `google.rpc.Code.INVALID_ARGUMENT` when location of the assignment\ndoes not match location of the reservation.");
            assignments3 = assignments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a assignment. No expansion will happen.\nE.g:\norganizationA contains project1 and project2. Reservation res1 exists.\nCreateAssignment was invoked previously and following assignments were\ncreated explicitly:\n  <organizationA, res1>\n  <project1, res1>\nThen deletion of <organizationA, res1> won\'t affect <project1, res1>. After\ndeletion of <organizationA, res1>, queries from project1 will still use\nres1, while queries from project2 will use on-demand mode.");
            assignments3 = assignments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists assignments.\nOnly explicitly created assignments will be returned. E.g:\norganizationA contains project1 and project2. Reservation res1 exists.\nCreateAssignment was invoked previously and following assignments were\ncreated explicitly:\n  <organizationA, res1>\n  <project1, res1>\nThen this API will just return the above two assignments for reservation\nres1, and no expansion/merge will happen. Wildcard \"-\" can be used for\nreservations in the request. In that case all assignments belongs to the\nspecified project and location will be listed. Note\n\"-\" cannot be used for projects nor locations.");
            assignments3 = assignments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("r#move").about("Moves a assignment under a new reservation. Customers can do this by\ndeleting the existing assignment followed by creating another assignment\nunder the new reservation, but this method provides a transactional way to\ndo so, to make sure the assignee always has an associated reservation.\nWithout the method customers might see some queries run on-demand which\nmight be unexpected.");
            assignments3 = assignments3.subcommand(mcmd);
        }
        reservations2 = reservations2.subcommand(assignments3);
        locations1 = locations1.subcommand(reservations2);
        locations1 = locations1.subcommand(capacity_commitments2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);
        app = app.subcommand(operations0);

        Self { app }
    }
}
use google_bigqueryreservation1 as api;

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

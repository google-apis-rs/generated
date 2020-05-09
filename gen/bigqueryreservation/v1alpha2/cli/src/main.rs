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
        let mut app = App::new("bigqueryreservation1_alpha2")
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
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search_reservation_grants");
        {
            let mcmd = SubCommand::with_name("search_reservation_grants").about("Look up grants for a specified resource for a particular region.\nIf the request is about a project:\n  1) Grants created on the project will be returned if they exist.\n  2) Otherwise grants created on the closest ancestor will be returned.\n  3) Grants for different JobTypes will all be returned.\nSame logic applies if the request is about a folder.\nIf the request is about an organization, then grants created on the\norganization will be returned (organization doesn\'t have ancestors).\nComparing to ListReservationGrants, there are two behavior\ndifferences:\n  1) permission on the grantee will be verified in this API.\n  2) Hierarchy lookup (project->folder->organization) happens in this API.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel and get");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut reservation_grants2 = SubCommand::with_name("reservation_grants")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete and list");
        {
            let mcmd = SubCommand::with_name("create").about("Returns `google.rpc.Code.PERMISSION_DENIED` if user does not have\n\'bigquery.admin\' permissions on the project using the reservation\nand the project that owns this reservation.\nReturns `google.rpc.Code.INVALID_ARGUMENT` when location of the grant\ndoes not match location of the reservation.");
            reservation_grants2 = reservation_grants2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a reservation grant. No expansion will happen.\nE.g:\norganizationA contains project1 and project2. Reservation res1 exists.\nCreateReservationGrant was invoked previously and following grants were\ncreated explicitly:\n  <organizationA, res1>\n  <project1, res1>\nThen deletion of <organizationA, res1> won\'t affect <project1, res1>. After\ndeletion of <organizationA, res1>, queries from project1 will still use\nres1, while queries from project2 will use on-demand mode.");
            reservation_grants2 = reservation_grants2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists reservation grants.\nOnly explicitly created grants will be returned. E.g:\norganizationA contains project1 and project2. Reservation res1 exists.\nCreateReservationGrant was invoked previously and following grants were\ncreated explicitly:\n  <organizationA, res1>\n  <project1, res1>\nThen this API will just return the above two grants for reservation res1,\nand no expansion/merge will happen.");
            reservation_grants2 = reservation_grants2.subcommand(mcmd);
        }
        let mut reservations2 = SubCommand::with_name("reservations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, create_reservation, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new reservation resource. Multiple reservations are created if\nthe ancestor reservations do not exist.");
            reservations2 = reservations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create_reservation").about("Creates a new reservation resource. Multiple reservations are created if\nthe ancestor reservations do not exist.");
            reservations2 = reservations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a reservation.\nReturns `google.rpc.Code.FAILED_PRECONDITION` in the following cases:\n  1. When reservation has child reservations. This check can be bypassed by\n     setting DeleteReservationRequest.force flag to true.\n  2. When top-level reservation with slot pools is being deleted.");
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
            let mcmd = SubCommand::with_name("patch").about("Updates an existing reservation resource. Applicable only for child\nreservations.");
            reservations2 = reservations2.subcommand(mcmd);
        }
        let mut slot_pools3 = SubCommand::with_name("slot_pools")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a slot pool. Attempting to delete slot pool before its\ncommitment_end_time will fail with the error code\n`google.rpc.Code.FAILED_PRECONDITION`.");
            slot_pools3 = slot_pools3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Returns information about the slot pool.");
            slot_pools3 = slot_pools3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the slot pools for the reservation.");
            slot_pools3 = slot_pools3.subcommand(mcmd);
        }
        reservations2 = reservations2.subcommand(slot_pools3);
        locations1 = locations1.subcommand(reservations2);
        locations1 = locations1.subcommand(reservation_grants2);
        locations1 = locations1.subcommand(operations2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_bigqueryreservation1_alpha2 as api;

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

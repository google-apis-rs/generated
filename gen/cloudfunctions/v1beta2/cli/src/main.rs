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
        let mut app = App::new("cloudfunctions1_beta2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200610")
            .about("Manages lightweight user-provided functions executed in response to events.")
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
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
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
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut functions2 = SubCommand::with_name("functions")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: call, create, delete, generate_download_url, generate_upload_url, get, list and update");
        {
            let mcmd = SubCommand::with_name("call").about("Synchronously invokes a deployed Cloud Function. To be used for testing\npurposes as very limited traffic is allowed. For more information on\nthe actual limits refer to [API Calls](\nhttps://cloud.google.com/functions/quotas#rate_limits).");
            functions2 = functions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new function. If a function with the given name already exists in\nthe specified project, the long running operation will return\n`ALREADY_EXISTS` error.");
            functions2 = functions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a function with the given name from the specified project. If the\ngiven function is used by some trigger, the trigger will be updated to\nremove this function.");
            functions2 = functions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_download_url").about("Returns a signed URL for downloading deployed function source code.\nThe URL is only valid for a limited period and should be used within\nminutes after generation.\nFor more information about the signed URL usage see:\nhttps://cloud.google.com/storage/docs/access-control/signed-urls");
            functions2 = functions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_upload_url").about("Returns a signed URL for uploading a function source code.\nFor more information about the signed URL usage see:\nhttps://cloud.google.com/storage/docs/access-control/signed-urls\nOnce the function source code upload is complete, the used signed\nURL should be provided in CreateFunction or UpdateFunction request\nas a reference to the function source code.\n\nWhen uploading source code to the generated signed URL, please follow\nthese restrictions:\n\n* Source file type should be a zip file.\n* Source file size should not exceed 100MB limit.\n* No credentials should be attached - the signed URLs provide access to the\n  target bucket using internal service identity; if credentials were\n  attached, the identity from the credentials would be used, but that\n  identity does not have permissions to upload files to the URL.\n\nWhen making a HTTP PUT request, these two headers need to be specified:\n\n* `content-type: application/zip`\n* `x-goog-content-length-range: 0,104857600`\n\nAnd this header SHOULD NOT be specified:\n\n* `Authorization: Bearer YOUR_TOKEN`");
            functions2 = functions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns a function with the given name from the requested project.");
            functions2 = functions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns a list of functions that belong to the requested project.");
            functions2 = functions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates existing function.");
            functions2 = functions2.subcommand(mcmd);
        }
        locations1 = locations1.subcommand(functions2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);
        app = app.subcommand(operations0);

        Self { app }
    }
}
use google_cloudfunctions1_beta2 as api;

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

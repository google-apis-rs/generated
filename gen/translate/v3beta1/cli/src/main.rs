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
        let mut app = App::new("translate3_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200619")
            .about("Integrates text translation into your website or application.")
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
            .about("methods: detect_language, get_supported_languages and translate_text");
        {
            let mcmd = SubCommand::with_name("detect_language")
                .about("Detects the language of text within a request.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_supported_languages")
                .about("Returns a list of supported languages for translation.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("translate_text")
                .about("Translates input text and returns translated text.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: batch_translate_text, detect_language, get, get_supported_languages, list and translate_text");
        {
            let mcmd = SubCommand::with_name("batch_translate_text").about("Translates a large volume of text in asynchronous batch mode.\nThis function provides real-time output as the inputs are being processed.\nIf caller cancels a request, the partial results (for an input file, it\'s\nall or nothing) may still be available on the specified output location.\n\nThis call returns immediately and you can\nuse google.longrunning.Operation.name to poll the status of the call.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("detect_language")
                .about("Detects the language of text within a request.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_supported_languages")
                .about("Returns a list of supported languages for translation.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("translate_text")
                .about("Translates input text and returns translated text.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut glossaries2 = SubCommand::with_name("glossaries")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a glossary and returns the long-running operation. Returns\nNOT_FOUND, if the project doesn\'t exist.");
            glossaries2 = glossaries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a glossary, or cancels glossary construction\nif the glossary isn\'t created yet.\nReturns NOT_FOUND, if the glossary doesn\'t exist.");
            glossaries2 = glossaries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a glossary. Returns NOT_FOUND, if the glossary doesn\'t\nexist.");
            glossaries2 = glossaries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists glossaries in a project. Returns NOT_FOUND, if the project doesn\'t\nexist.",
            );
            glossaries2 = glossaries2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get, list and wait");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is\nno longer interested in the operation result. It does not cancel the\noperation. If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the\nserver doesn\'t support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("wait").about("Waits for the specified long-running operation until it is done or reaches\nat most a specified timeout, returning the latest state.  If the operation\nis already done, the latest state is immediately returned.  If the timeout\nspecified is greater than the default HTTP/RPC timeout, the HTTP/RPC\ntimeout is used.  If the server does not support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.\nNote that this method is on a best-effort basis.  It may return the latest\nstate before the specified timeout (including immediately), meaning even an\nimmediate response is no guarantee that the operation is done.");
            operations2 = operations2.subcommand(mcmd);
        }
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(glossaries2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_translate3_beta1 as api;

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

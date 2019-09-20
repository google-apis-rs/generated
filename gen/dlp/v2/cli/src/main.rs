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
        let mut app = App::new("dlp2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20190913")
            .about("Provides methods for detection, risk analysis, and de-identification of privacy-sensitive fragments in text, images, and Google Cloud Platform storage repositories.")
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
        let mut info_types0 = SubCommand::with_name("info_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of the sensitive information types that the DLP API\nsupports. See https://cloud.google.com/dlp/docs/infotypes-reference to\nlearn more.");
            info_types0 = info_types0.subcommand(mcmd);
        }
        let mut locations0 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: info_types");
        {
            let mcmd = SubCommand::with_name("info_types").about("Returns a list of the sensitive information types that the DLP API\nsupports. See https://cloud.google.com/dlp/docs/infotypes-reference to\nlearn more.");
            locations0 = locations0.subcommand(mcmd);
        }
        let mut organizations0 = SubCommand::with_name("organizations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: deidentify_templates, inspect_templates and stored_info_types");
        let mut projects0 = SubCommand::with_name("projects")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: content, deidentify_templates, dlp_jobs, image, inspect_templates, job_triggers, locations and stored_info_types");
        let mut deidentify_templates1 = SubCommand::with_name("deidentify_templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a DeidentifyTemplate for re-using frequently used configuration\nfor de-identifying content, images, and storage.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a DeidentifyTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a DeidentifyTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists DeidentifyTemplates.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the DeidentifyTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        let mut inspect_templates1 = SubCommand::with_name("inspect_templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an InspectTemplate for re-using frequently used configuration\nfor inspecting content, images, and storage.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an InspectTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an InspectTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists InspectTemplates.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the InspectTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        let mut stored_info_types1 = SubCommand::with_name("stored_info_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a pre-built stored infoType to be used for inspection.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a stored infoType.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a stored infoType.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists stored infoTypes.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the stored infoType by creating a new version. The existing version\nwill continue to be used until the new version is ready.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        let mut content1 = SubCommand::with_name("content")
            .setting(AppSettings::ColoredHelp)
            .about("methods: deidentify, inspect and reidentify");
        {
            let mcmd = SubCommand::with_name("deidentify").about("De-identifies potentially sensitive info from a ContentItem.\nThis method has limits on input size and output size.\nSee https://cloud.google.com/dlp/docs/deidentify-sensitive-data to\nlearn more.\n\nWhen no InfoTypes or CustomInfoTypes are specified in this request, the\nsystem will automatically choose what detectors to run. By default this may\nbe all types, but may change over time as detectors are updated.");
            content1 = content1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("inspect").about("Finds potentially sensitive info in content.\nThis method has limits on input size, processing time, and output size.\n\nWhen no InfoTypes or CustomInfoTypes are specified in this request, the\nsystem will automatically choose what detectors to run. By default this may\nbe all types, but may change over time as detectors are updated.\n\nFor how to guides, see https://cloud.google.com/dlp/docs/inspecting-images\nand https://cloud.google.com/dlp/docs/inspecting-text,");
            content1 = content1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reidentify").about("Re-identifies content that has been de-identified.\nSee\nhttps://cloud.google.com/dlp/docs/pseudonymization#re-identification_in_free_text_code_example\nto learn more.");
            content1 = content1.subcommand(mcmd);
        }
        let mut deidentify_templates1 = SubCommand::with_name("deidentify_templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a DeidentifyTemplate for re-using frequently used configuration\nfor de-identifying content, images, and storage.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a DeidentifyTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a DeidentifyTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists DeidentifyTemplates.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the DeidentifyTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates-deid to learn\nmore.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        let mut dlp_jobs1 = SubCommand::with_name("dlp_jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running DlpJob. The server\nmakes a best effort to cancel the DlpJob, but success is not\nguaranteed.\nSee https://cloud.google.com/dlp/docs/inspecting-storage and\nhttps://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.");
            dlp_jobs1 = dlp_jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new job to inspect storage or calculate risk metrics.\nSee https://cloud.google.com/dlp/docs/inspecting-storage and\nhttps://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.\n\nWhen no InfoTypes or CustomInfoTypes are specified in inspect jobs, the\nsystem will automatically choose what detectors to run. By default this may\nbe all types, but may change over time as detectors are updated.");
            dlp_jobs1 = dlp_jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running DlpJob. This method indicates that the client is\nno longer interested in the DlpJob result. The job will be cancelled if\npossible.\nSee https://cloud.google.com/dlp/docs/inspecting-storage and\nhttps://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.");
            dlp_jobs1 = dlp_jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running DlpJob.\nSee https://cloud.google.com/dlp/docs/inspecting-storage and\nhttps://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.");
            dlp_jobs1 = dlp_jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists DlpJobs that match the specified filter in the request.\nSee https://cloud.google.com/dlp/docs/inspecting-storage and\nhttps://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.");
            dlp_jobs1 = dlp_jobs1.subcommand(mcmd);
        }
        let mut image1 = SubCommand::with_name("image")
            .setting(AppSettings::ColoredHelp)
            .about("methods: redact");
        {
            let mcmd = SubCommand::with_name("redact").about("Redacts potentially sensitive info from an image.\nThis method has limits on input size, processing time, and output size.\nSee https://cloud.google.com/dlp/docs/redacting-sensitive-data-images to\nlearn more.\n\nWhen no InfoTypes or CustomInfoTypes are specified in this request, the\nsystem will automatically choose what detectors to run. By default this may\nbe all types, but may change over time as detectors are updated.");
            image1 = image1.subcommand(mcmd);
        }
        let mut inspect_templates1 = SubCommand::with_name("inspect_templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an InspectTemplate for re-using frequently used configuration\nfor inspecting content, images, and storage.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an InspectTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an InspectTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists InspectTemplates.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the InspectTemplate.\nSee https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        let mut job_triggers1 = SubCommand::with_name("job_triggers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: activate, create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("activate").about("Activate a job trigger. Causes the immediate execute of a trigger\ninstead of waiting on the trigger event to occur.");
            job_triggers1 = job_triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a job trigger to run DLP actions such as scanning storage for\nsensitive information on a set schedule.\nSee https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers1 = job_triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a job trigger.\nSee https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers1 = job_triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a job trigger.\nSee https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers1 = job_triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists job triggers.\nSee https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers1 = job_triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a job trigger.\nSee https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers1 = job_triggers1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: content");
        let mut stored_info_types1 = SubCommand::with_name("stored_info_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a pre-built stored infoType to be used for inspection.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a stored infoType.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a stored infoType.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists stored infoTypes.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the stored infoType by creating a new version. The existing version\nwill continue to be used until the new version is ready.\nSee https://cloud.google.com/dlp/docs/creating-stored-infotypes to\nlearn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        let mut content2 = SubCommand::with_name("content")
            .setting(AppSettings::ColoredHelp)
            .about("methods: deidentify, inspect and reidentify");
        {
            let mcmd = SubCommand::with_name("deidentify").about("De-identifies potentially sensitive info from a ContentItem.\nThis method has limits on input size and output size.\nSee https://cloud.google.com/dlp/docs/deidentify-sensitive-data to\nlearn more.\n\nWhen no InfoTypes or CustomInfoTypes are specified in this request, the\nsystem will automatically choose what detectors to run. By default this may\nbe all types, but may change over time as detectors are updated.");
            content2 = content2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("inspect").about("Finds potentially sensitive info in content.\nThis method has limits on input size, processing time, and output size.\n\nWhen no InfoTypes or CustomInfoTypes are specified in this request, the\nsystem will automatically choose what detectors to run. By default this may\nbe all types, but may change over time as detectors are updated.\n\nFor how to guides, see https://cloud.google.com/dlp/docs/inspecting-images\nand https://cloud.google.com/dlp/docs/inspecting-text,");
            content2 = content2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reidentify").about("Re-identifies content that has been de-identified.\nSee\nhttps://cloud.google.com/dlp/docs/pseudonymization#re-identification_in_free_text_code_example\nto learn more.");
            content2 = content2.subcommand(mcmd);
        }
        locations1 = locations1.subcommand(content2);
        projects0 = projects0.subcommand(stored_info_types1);
        projects0 = projects0.subcommand(locations1);
        projects0 = projects0.subcommand(job_triggers1);
        projects0 = projects0.subcommand(inspect_templates1);
        projects0 = projects0.subcommand(image1);
        projects0 = projects0.subcommand(dlp_jobs1);
        projects0 = projects0.subcommand(deidentify_templates1);
        projects0 = projects0.subcommand(content1);
        organizations0 = organizations0.subcommand(stored_info_types1);
        organizations0 = organizations0.subcommand(inspect_templates1);
        organizations0 = organizations0.subcommand(deidentify_templates1);
        app = app.subcommand(projects0);
        app = app.subcommand(organizations0);
        app = app.subcommand(locations0);
        app = app.subcommand(info_types0);

        Self { app }
    }
}
use google_dlp2 as api;

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

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
        let mut app = App::new("firebase1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200625")
            .about("The Firebase Management API enables programmatic setup and management of Firebase projects, including a project\'s Firebase resources and Firebase apps.")
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
        let mut available_projects0 = SubCommand::with_name("available_projects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of [Google Cloud Platform (GCP) `Projects`]\n(https://cloud.google.com/resource-manager/reference/rest/v1/projects)\nthat are available to have Firebase resources added to them.\n<br>\n<br>A GCP `Project` will only be returned if:\n<ol>\n  <li><p>The caller has sufficient\n         [Google IAM](https://cloud.google.com/iam) permissions to call\n         AddFirebase.</p></li>\n  <li><p>The GCP `Project` is not already a FirebaseProject.</p></li>\n  <li><p>The GCP `Project` is not in an Organization which has policies\n         that prevent Firebase resources from being added.</p></li>\n</ol>");
            available_projects0 = available_projects0.subcommand(mcmd);
        }
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_firebase, add_google_analytics, get, get_admin_sdk_config, get_analytics_details, list, patch, remove_analytics and search_apps");
        {
            let mcmd = SubCommand::with_name("add_firebase").about("Adds Firebase resources to the specified existing\n[Google Cloud Platform (GCP) `Project`]\n(https://cloud.google.com/resource-manager/reference/rest/v1/projects).\n<br>\n<br>Since a FirebaseProject is actually also a GCP `Project`, a\n`FirebaseProject` uses underlying GCP identifiers (most importantly,\nthe `PROJECT_NUMBER`) as its own for easy interop with GCP APIs.\n<br>\n<br>The result of this call is an [`Operation`](../../v1beta1/operations).\nPoll the `Operation` to track the provisioning process by calling\nGetOperation until\n[`done`](../../v1beta1/operations#Operation.FIELDS.done) is `true`. When\n`done` is `true`, the `Operation` has either succeeded or failed. If the\n`Operation` succeeded, its\n[`response`](../../v1beta1/operations#Operation.FIELDS.response) is set to\na FirebaseProject; if the `Operation` failed, its\n[`error`](../../v1beta1/operations#Operation.FIELDS.error) is set to a\ngoogle.rpc.Status. The `Operation` is automatically deleted after\ncompletion, so there is no need to call\nDeleteOperation.\n<br>\n<br>This method does not modify any billing account information on the\nunderlying GCP `Project`.\n<br>\n<br>To call `AddFirebase`, a project member or service account must have\nthe following permissions (the IAM roles of Editor and Owner contain these\npermissions):\n`firebase.projects.update`, `resourcemanager.projects.get`,\n`serviceusage.services.enable`, and `serviceusage.services.get`.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("add_google_analytics").about("Links a FirebaseProject with an existing\n[Google Analytics account](http://www.google.com/analytics/).\n<br>\n<br>Using this call, you can either:\n<ul>\n<li>Specify an `analyticsAccountId` to provision a new Google Analytics\nproperty within the specified account and associate the new property with\nyour `FirebaseProject`.</li>\n<li>Specify an existing `analyticsPropertyId` to associate the property\nwith your `FirebaseProject`.</li>\n</ul>\n<br>\nNote that when you call `AddGoogleAnalytics`:\n<ol>\n<li>The first check determines if any existing data streams in the\nGoogle Analytics property correspond to any existing Firebase Apps in your\n`FirebaseProject` (based on the `packageName` or `bundleId` associated with\nthe data stream). Then, as applicable, the data streams and apps are\nlinked. Note that this auto-linking only applies to Android Apps and iOS\nApps.</li>\n<li>If no corresponding data streams are found for your Firebase Apps,\nnew data streams are provisioned in the Google Analytics property\nfor each of your Firebase Apps. Note that a new data stream is always\nprovisioned for a Web App even if it was previously associated with a\ndata stream in your Analytics property.</li>\n</ol>\nLearn more about the hierarchy and structure of Google Analytics\naccounts in the\n[Analytics\ndocumentation](https://support.google.com/analytics/answer/9303323).\n<br>\n<br>The result of this call is an [`Operation`](../../v1beta1/operations).\nPoll the `Operation` to track the provisioning process by calling\nGetOperation until\n[`done`](../../v1beta1/operations#Operation.FIELDS.done) is `true`. When\n`done` is `true`, the `Operation` has either succeeded or failed. If the\n`Operation` succeeded, its\n[`response`](../../v1beta1/operations#Operation.FIELDS.response) is set to\nan AnalyticsDetails; if the `Operation` failed, its\n[`error`](../../v1beta1/operations#Operation.FIELDS.error) is set to a\ngoogle.rpc.Status.\n<br>\n<br>To call `AddGoogleAnalytics`, a member must be an Owner for\nthe existing `FirebaseProject` and have the\n[`Edit` permission](https://support.google.com/analytics/answer/2884495)\nfor the Google Analytics account.\n<br>\n<br>If a `FirebaseProject` already has Google Analytics enabled, and you\ncall `AddGoogleAnalytics` using an `analyticsPropertyId` that\'s different\nfrom the currently associated property, then the call will fail. Analytics\nmay have already been enabled in the Firebase console or by specifying\n`timeZone` and `regionCode` in the call to\n[`AddFirebase`](../../v1beta1/projects/addFirebase).");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the FirebaseProject identified by the specified resource name.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_admin_sdk_config").about("Gets the configuration artifact used by servers to simplify initialization.\n<br>\n<br>Typically, this configuration is used with the Firebase Admin SDK\n[initializeApp](https://firebase.google.com/docs/admin/setup#initialize_the_sdk)\ncommand.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_analytics_details").about("Gets the Google Analytics details currently associated with a\nFirebaseProject.\n<br>\n<br>If the `FirebaseProject` is not yet linked to Google Analytics, then\nthe response to `GetAnalyticsDetails` is NOT_FOUND.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists each FirebaseProject accessible to the caller.\n<br>\n<br>The elements are returned in no particular order, but they will be a\nconsistent view of the Projects when additional requests are made with a\n`pageToken`.\n<br>\n<br>This method is eventually consistent with Project mutations, which\nmeans newly provisioned Projects and recent modifications to existing\nProjects might not be reflected in the set of Projects. The list will\ninclude only ACTIVE Projects.\n<br>\n<br>Use\nGetFirebaseProject\nfor consistent reads as well as for additional Project details.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the attributes of the FirebaseProject identified by the\nspecified resource name.\n<br>\n<br>All [query parameters](#query-parameters) are required.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_analytics").about("Unlinks the specified `FirebaseProject` from its Google Analytics account.\n<br>\n<br>This call removes the association of the specified `FirebaseProject`\nwith its current Google Analytics property. However, this call does not\ndelete the Google Analytics resources, such as the Google Analytics\nproperty or any data streams.\n<br>\n<br>These resources may be re-associated later to the `FirebaseProject` by\ncalling\n[`AddGoogleAnalytics`](../../v1beta1/projects/addGoogleAnalytics) and\nspecifying the same `analyticsPropertyId`. For Android Apps and iOS Apps,\nthis call re-links data streams with their corresponding apps. However,\nfor Web Apps, this call provisions a <em>new</em> data stream for each Web\nApp.\n<br>\n<br>To call `RemoveAnalytics`, a member must be an Owner for\nthe `FirebaseProject`.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_apps").about("A convenience method that lists all available Apps for the specified\nFirebaseProject.\n<br>\n<br>Typically, interaction with an App should be done using the\nplatform-specific service, but some tool use-cases require a summary of all\nknown Apps (such as for App selector interfaces).");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut android_apps1 = SubCommand::with_name("android_apps")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, get_config, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Requests that a new AndroidApp be created.\n<br>\n<br>The result of this call is an `Operation` which can be used to track\nthe provisioning process. The `Operation` is automatically deleted after\ncompletion, so there is no need to call `DeleteOperation`.");
            android_apps1 = android_apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the AndroidApp identified by the specified resource name.");
            android_apps1 = android_apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_config").about(
                "Gets the configuration artifact associated with the specified\nAndroidApp.",
            );
            android_apps1 = android_apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists each AndroidApp associated with the specified parent Project.\n<br>\n<br>The elements are returned in no particular order, but will be a\nconsistent view of the Apps when additional requests are made with a\n`pageToken`.");
            android_apps1 = android_apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the attributes of the AndroidApp identified by the specified\nresource name.");
            android_apps1 = android_apps1.subcommand(mcmd);
        }
        let mut available_locations1 = SubCommand::with_name("available_locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of valid Google Cloud Platform (GCP) resource locations for\nthe specified Project (including a FirebaseProject).\n<br>\n<br>One of these locations can be selected as the Project\'s [_default_ GCP\nresource location](https://firebase.google.com/docs/projects/locations),\nwhich is the geographical location where project resources, such as Cloud\nFirestore, will be provisioned by default. However, if the default GCP\nresource location has already been set for the Project, then this setting\ncannot be changed.\n<br>\n<br>This call checks for any possible\n[location\nrestrictions](https://cloud.google.com/resource-manager/docs/organization-policy/defining-locations)\nfor the specified Project and, thus, might return a subset of all possible\nGCP resource locations. To list all GCP resource locations (regardless of\nany restrictions), call the endpoint without specifying a `PROJECT_NUMBER`\n(that is, `/v1beta1/{parent=projects/-}/listAvailableLocations`). <br>\n<br>To call `ListAvailableLocations` with a specified project, a member\nmust be at minimum a Viewer of the project. Calls without a specified\nproject do not require any specific project permissions.");
            available_locations1 = available_locations1.subcommand(mcmd);
        }
        let mut default_location1 = SubCommand::with_name("default_location")
            .setting(AppSettings::ColoredHelp)
            .about("methods: finalize");
        {
            let mcmd = SubCommand::with_name("finalize").about("Sets the default Google Cloud Platform (GCP) resource location for the\nspecified FirebaseProject.\n<br>\n<br>This method creates an App Engine application with a\n[default Cloud Storage\nbucket](https://cloud.google.com/appengine/docs/standard/python/googlecloudstorageclient/setting-up-cloud-storage#activating_a_cloud_storage_bucket),\nlocated in the specified\n[`location_id`](#body.request_body.FIELDS.location_id).\nThis location must be one of the available\n[GCP resource\nlocations](https://firebase.google.com/docs/projects/locations). <br>\n<br>After the default GCP resource location is finalized, or if it was\nalready set, it cannot be changed. The default GCP resource location for\nthe specified FirebaseProject might already be set because either the\nGCP `Project` already has an App Engine application or\n`FinalizeDefaultLocation` was previously called with a specified\n`location_id`. Any new calls to `FinalizeDefaultLocation` with a\n<em>different</em> specified `location_id` will return a 409 error.\n<br>\n<br>The result of this call is an [`Operation`](../../v1beta1/operations),\nwhich can be used to track the provisioning process. The\n[`response`](../../v1beta1/operations#Operation.FIELDS.response) type of\nthe `Operation` is google.protobuf.Empty.\n<br>\n<br>The `Operation` can be polled by its `name` using\nGetOperation until `done` is\ntrue. When `done` is true, the `Operation` has either succeeded or failed.\nIf the `Operation` has succeeded, its\n[`response`](../../v1beta1/operations#Operation.FIELDS.response) will be\nset to a google.protobuf.Empty; if the `Operation` has failed, its\n`error` will be set to a google.rpc.Status. The `Operation` is\nautomatically deleted after completion, so there is no need to call\nDeleteOperation.\n<br>\n<br>All fields listed in the [request body](#request-body) are required.\n<br>\n<br>To call `FinalizeDefaultLocation`, a member must be an Owner\nof the project.");
            default_location1 = default_location1.subcommand(mcmd);
        }
        let mut ios_apps1 = SubCommand::with_name("ios_apps")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, get_config, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Requests that a new IosApp be created.\n<br>\n<br>The result of this call is an `Operation` which can be used to track\nthe provisioning process. The `Operation` is automatically deleted after\ncompletion, so there is no need to call `DeleteOperation`.");
            ios_apps1 = ios_apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the IosApp identified by the specified resource name.");
            ios_apps1 = ios_apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_config")
                .about("Gets the configuration artifact associated with the specified IosApp.");
            ios_apps1 = ios_apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists each IosApp associated with the specified parent Project.\n<br>\n<br>The elements are returned in no particular order, but will be a\nconsistent view of the Apps when additional requests are made with a\n`pageToken`.");
            ios_apps1 = ios_apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates the attributes of the IosApp identified by the specified\nresource name.",
            );
            ios_apps1 = ios_apps1.subcommand(mcmd);
        }
        let mut web_apps1 = SubCommand::with_name("web_apps")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, get_config, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Requests that a new WebApp be created.\n<br>\n<br>The result of this call is an `Operation` which can be used to track\nthe provisioning process. The `Operation` is automatically deleted after\ncompletion, so there is no need to call `DeleteOperation`.");
            web_apps1 = web_apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the WebApp identified by the specified resource name.");
            web_apps1 = web_apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_config")
                .about("Gets the configuration artifact associated with the specified WebApp.");
            web_apps1 = web_apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists each WebApp associated with the specified parent Project.\n<br>\n<br>The elements are returned in no particular order, but will be a\nconsistent view of the Apps when additional requests are made with a\n`pageToken`.");
            web_apps1 = web_apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Updates the attributes of the WebApp identified by the specified\nresource name.",
            );
            web_apps1 = web_apps1.subcommand(mcmd);
        }
        let mut sha2 = SubCommand::with_name("sha")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete and list");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Adds a SHA certificate to the specified AndroidApp.");
            sha2 = sha2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Removes a SHA certificate from the specified AndroidApp.");
            sha2 = sha2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Returns the list of SHA-1 and SHA-256 certificates for the specified\nAndroidApp.",
            );
            sha2 = sha2.subcommand(mcmd);
        }
        android_apps1 = android_apps1.subcommand(sha2);
        projects0 = projects0.subcommand(web_apps1);
        projects0 = projects0.subcommand(ios_apps1);
        projects0 = projects0.subcommand(default_location1);
        projects0 = projects0.subcommand(available_locations1);
        projects0 = projects0.subcommand(android_apps1);
        app = app.subcommand(projects0);
        app = app.subcommand(operations0);
        app = app.subcommand(available_projects0);

        Self { app }
    }
}
use google_firebase1_beta1 as api;

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

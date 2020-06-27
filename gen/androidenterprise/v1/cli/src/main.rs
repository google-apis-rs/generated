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
        let mut app = App::new("androidenterprise1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200620")
            .about("Manages the deployment of apps to Android Enterprise devices.")
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
        let mut devices0 = SubCommand::with_name("devices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: force_report_upload, get, get_state, list, set_state and update");
        {
            let mcmd = SubCommand::with_name("force_report_upload").about("Uploads a report containing any changes in app states on the device since\nthe last report was generated. You can call this method up to 3 times every\n24 hours for a given device.\nIf you exceed the quota, then the Google Play EMM API returns <code>HTTP\n429 Too Many Requests</code>.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the details of a device.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_state").about("Retrieves whether a device\'s access to Google services is enabled or\ndisabled.\nThe device state takes effect only if enforcing EMM policies on Android\ndevices is enabled in the Google Admin Console.\nOtherwise, the device state is ignored and all devices are allowed access\nto Google services.\nThis is only supported for Google-managed users.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves the IDs of all of a user\'s devices.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_state").about("Sets whether a device\'s access to Google services is enabled or disabled.\nThe device state takes effect only if enforcing EMM policies on Android\ndevices is enabled in the Google Admin Console.\nOtherwise, the device state is ignored and all devices are allowed access\nto Google services.\nThis is only supported for Google-managed users.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the device policy");
            devices0 = devices0.subcommand(mcmd);
        }
        let mut enterprises0 = SubCommand::with_name("enterprises")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: acknowledge_notification_set, complete_signup, create_web_token, enroll, generate_signup_url, get, get_service_account, get_store_layout, list, pull_notification_set, send_test_push_notification, set_account, set_store_layout and unenroll");
        {
            let mcmd = SubCommand::with_name("acknowledge_notification_set").about("Acknowledges notifications that were received from\nEnterprises.PullNotificationSet to prevent subsequent calls from returning\nthe same notifications.");
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("complete_signup").about("Completes the signup flow, by specifying the Completion token and\nEnterprise token.\nThis request must not be called multiple times for a given Enterprise\nToken.");
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create_web_token").about("Returns a unique token to access an embeddable UI. To generate a\nweb UI, pass the generated token into the managed Google Play javascript\nAPI. Each token may only be used to start one UI session. See the\njavascript API documentation for further information.");
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("enroll")
                .about("Enrolls an enterprise with the calling EMM.");
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("generate_signup_url").about("Generates a sign-up URL.");
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves the name and domain of an enterprise.");
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_service_account").about("Returns a service account and credentials. The service account\ncan be bound to the enterprise by calling setAccount. The service account\nis unique to this enterprise and EMM, and will be deleted if the enterprise\nis unbound. The credentials contain private key data and are not stored\nserver-side.\n<br> <br>\nThis method can only be called after calling\nEnterprises.Enroll or Enterprises.CompleteSignup, and before\nEnterprises.SetAccount; at other times it will return an error.\n<br> <br>\nSubsequent calls after the first will generate a new, unique set of\ncredentials, and invalidate the previously generated credentials.\n<br> <br>\nOnce the service account is bound to the enterprise, it can be managed\nusing the serviceAccountKeys resource.");
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_store_layout").about("Returns the store layout for the enterprise. If the store layout\nhas not been set, returns \"basic\" as the store layout type and no\nhomepage.");
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Looks up an enterprise by domain name.\nThis is only supported for enterprises created via the Google-initiated\ncreation flow.  Lookup of the id is not needed for enterprises created via\nthe EMM-initiated flow since the EMM learns the enterprise ID in the\ncallback specified in the Enterprises.generateSignupUrl call.");
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("pull_notification_set").about("Pulls and returns a notification set for the enterprises associated with\nthe service account authenticated for the request. The notification set may\nbe empty if no notification are pending.\n<br>\nA notification set returned needs to be acknowledged within 20 seconds\nby calling Enterprises.AcknowledgeNotificationSet, unless the\nnotification set is empty.\n<br>\nNotifications that are not acknowledged within the 20 seconds will\neventually be included again in the response to another PullNotificationSet\nrequest, and those that are never acknowledged will ultimately be deleted\naccording to the Google Cloud Platform Pub/Sub system policy.\n<br>\nMultiple requests might be performed concurrently to retrieve\nnotifications, in which case the pending notifications (if any) will be\nsplit among each caller, if any are pending.\n<br>\nIf no notifications are present, an empty notification list is returned.\nSubsequent requests may return more notifications once they become\navailable.");
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("send_test_push_notification").about("Sends a test notification to validate the EMM integration with\nthe Google Cloud Pub/Sub service for this enterprise.");
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_account").about(
                "Sets the account that will be used to authenticate to the API as the\nenterprise.",
            );
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_store_layout").about("Sets the store layout for the enterprise. By default, storeLayoutType\nis set to \"basic\" and the basic store layout is enabled. The basic\nlayout only contains apps approved by the admin, and that have\nbeen added to the available product set for a user (using the\n<a href=\"/android/work/play/emm-api/v1/users/setAvailableProductSet\">\nsetAvailableProductSet</a> call). Apps on the page are sorted in order of\ntheir product ID value. If you create a custom store layout (by setting\nstoreLayoutType = \"custom\" and setting a homepage), the basic store\nlayout is disabled.");
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unenroll")
                .about("Unenrolls an enterprise from the calling EMM.");
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        let mut entitlements0 = SubCommand::with_name("entitlements")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Removes an entitlement to an app for a user.");
            entitlements0 = entitlements0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves details of an entitlement.");
            entitlements0 = entitlements0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all entitlements for the specified user. Only the ID is set.");
            entitlements0 = entitlements0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Adds or updates an entitlement to an app for a user.");
            entitlements0 = entitlements0.subcommand(mcmd);
        }
        let mut grouplicenses0 = SubCommand::with_name("grouplicenses")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves details of an enterprise\'s group license for a product.");
            grouplicenses0 = grouplicenses0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Retrieves IDs of all products for which the enterprise has a group license.",
            );
            grouplicenses0 = grouplicenses0.subcommand(mcmd);
        }
        let mut grouplicenseusers0 = SubCommand::with_name("grouplicenseusers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the IDs of the users who have been granted entitlements\nunder the license.");
            grouplicenseusers0 = grouplicenseusers0.subcommand(mcmd);
        }
        let mut installs0 = SubCommand::with_name("installs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Requests to remove an app from a device. A call to <code>get</code> or\n<code>list</code> will still show the app as installed on the device until\nit is actually removed.");
            installs0 = installs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves details of an installation of an app on a device.");
            installs0 = installs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves the details of all apps installed on the specified device.");
            installs0 = installs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Requests to install the latest version of an app to a device. If the app\nis already installed, then it is updated to the latest version if\nnecessary.");
            installs0 = installs0.subcommand(mcmd);
        }
        let mut managedconfigurationsfordevice0 =
            SubCommand::with_name("managedconfigurationsfordevice")
                .setting(AppSettings::ColoredHelp)
                .about("methods: delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Removes a per-device managed configuration for an app for the specified\ndevice.",
            );
            managedconfigurationsfordevice0 = managedconfigurationsfordevice0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves details of a per-device managed configuration.");
            managedconfigurationsfordevice0 = managedconfigurationsfordevice0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all the per-device managed configurations for the specified device.\nOnly the ID is set.");
            managedconfigurationsfordevice0 = managedconfigurationsfordevice0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Adds or updates a per-device managed configuration for an app for the\nspecified device.");
            managedconfigurationsfordevice0 = managedconfigurationsfordevice0.subcommand(mcmd);
        }
        let mut managedconfigurationsforuser0 =
            SubCommand::with_name("managedconfigurationsforuser")
                .setting(AppSettings::ColoredHelp)
                .about("methods: delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Removes a per-user managed configuration for an app for the specified user.",
            );
            managedconfigurationsforuser0 = managedconfigurationsforuser0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves details of a per-user managed configuration for an app for the\nspecified user.");
            managedconfigurationsforuser0 = managedconfigurationsforuser0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all the per-user managed configurations for the specified user. Only\nthe ID is set.");
            managedconfigurationsforuser0 = managedconfigurationsforuser0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Adds or updates the managed configuration settings for an app for the\nspecified user.\nIf you support the <a\nhref=\"https://developers.google.com/android/work/play/emm-api/managed-configurations-iframe\">Managed\nconfigurations iframe</a>,\nyou can apply managed configurations to a user by specifying an\n<code>mcmId</code>\nand its associated configuration variables (if any) in the request.\nAlternatively,\nall EMMs can apply managed configurations by passing a list of managed\nproperties.");
            managedconfigurationsforuser0 = managedconfigurationsforuser0.subcommand(mcmd);
        }
        let mut managedconfigurationssettings0 =
            SubCommand::with_name("managedconfigurationssettings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the managed configurations settings for the specified app.");
            managedconfigurationssettings0 = managedconfigurationssettings0.subcommand(mcmd);
        }
        let mut permissions0 = SubCommand::with_name("permissions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves details of an Android app permission for display to an enterprise\nadmin.");
            permissions0 = permissions0.subcommand(mcmd);
        }
        let mut products0 = SubCommand::with_name("products")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: approve, generate_approval_url, get, get_app_restrictions_schema, get_permissions, list and unapprove");
        {
            let mcmd = SubCommand::with_name("approve").about("<p>Approves the specified product and the relevant app permissions, if any.\nThe maximum number of products that you can approve per enterprise customer\nis 1,000.</p>\n<p>To learn how to use managed Google Play to design and create a store\nlayout to display approved products to your users,\nsee <a href=\"/android/work/play/emm-api/store-layout\">Store Layout\nDesign</a>.</p>");
            products0 = products0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_approval_url").about("Generates a URL that can be rendered in an iframe to display the\npermissions (if any) of a product. An enterprise admin must view these\npermissions and accept them on behalf of their organization in order to\napprove that product. <br><br>\nAdmins should accept the displayed permissions by\ninteracting with a separate UI element in the EMM console, which in turn\nshould trigger the use of this URL as the\n<code>approvalUrlInfo.approvalUrl</code> property in a\n<code>Products.approve</code> call to approve the product.\nThis URL can only be used to display permissions for up to 1 day.");
            products0 = products0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves details of a product for display to an enterprise admin.");
            products0 = products0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_app_restrictions_schema").about("Retrieves the schema that defines the configurable properties for this\nproduct. All products have a schema, but this schema may be empty if no\nmanaged configurations have been defined. This schema can be used to\npopulate a UI that allows an admin to configure the product.\nTo apply a managed configuration based on the schema obtained using this\nAPI, see\n<a href=\"/android/work/play/emm-api/managed-configurations\">Managed\nConfigurations through Play</a>.");
            products0 = products0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_permissions")
                .about("Retrieves the Android app permissions required by this app.");
            products0 = products0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Finds approved products that match a query, or all approved products\nif there is no query.");
            products0 = products0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unapprove").about(
                "Unapproves the specified product (and the relevant app permissions, if any)",
            );
            products0 = products0.subcommand(mcmd);
        }
        let mut serviceaccountkeys0 = SubCommand::with_name("serviceaccountkeys")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, insert and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Removes and invalidates the specified credentials for the service account\nassociated with this enterprise. The calling service account must have been\nretrieved by calling Enterprises.GetServiceAccount and must have been set\nas the enterprise service account by calling Enterprises.SetAccount.");
            serviceaccountkeys0 = serviceaccountkeys0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Generates new credentials for the service account associated with this\nenterprise. The calling service account must have been retrieved by calling\nEnterprises.GetServiceAccount and must have been set as the enterprise\nservice account by calling Enterprises.SetAccount. <br><br>\nOnly the type of the key should be populated in the resource to be\ninserted.");
            serviceaccountkeys0 = serviceaccountkeys0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all active credentials for the service account associated with this\nenterprise. Only the ID and key type are returned. The calling service\naccount must have been retrieved by calling Enterprises.GetServiceAccount\nand must have been set as the enterprise service account by calling\nEnterprises.SetAccount.");
            serviceaccountkeys0 = serviceaccountkeys0.subcommand(mcmd);
        }
        let mut storelayoutclusters0 = SubCommand::with_name("storelayoutclusters")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a cluster.");
            storelayoutclusters0 = storelayoutclusters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves details of a cluster.");
            storelayoutclusters0 = storelayoutclusters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new cluster in a page.");
            storelayoutclusters0 = storelayoutclusters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves the details of all clusters on the specified page.");
            storelayoutclusters0 = storelayoutclusters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a cluster.");
            storelayoutclusters0 = storelayoutclusters0.subcommand(mcmd);
        }
        let mut storelayoutpages0 = SubCommand::with_name("storelayoutpages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a store page.");
            storelayoutpages0 = storelayoutpages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves details of a store page.");
            storelayoutpages0 = storelayoutpages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Inserts a new store page.");
            storelayoutpages0 = storelayoutpages0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves the details of all pages in the store.");
            storelayoutpages0 = storelayoutpages0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates the content of a store page.");
            storelayoutpages0 = storelayoutpages0.subcommand(mcmd);
        }
        let mut users0 = SubCommand::with_name("users")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete, generate_authentication_token, get, get_available_product_set, insert, list, revoke_device_access, set_available_product_set and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deleted an EMM-managed user.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_authentication_token").about("Generates an authentication token which the device policy client can use to\nprovision the given EMM-managed user account on a device.\nThe generated token is single-use and expires after a few minutes.\n\nYou can provision a maximum of 10 devices per user.\n\nThis call only works with EMM-managed accounts.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a user\'s details.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_available_product_set")
                .about("Retrieves the set of products a user is entitled to access.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert").about("Creates a new EMM-managed user.\n\nThe <a href=\"/android/work/play/emm-api/v1/users.html\">Users</a> resource\npassed in the body of the request should include an\n<code>accountIdentifier</code> and an <code>accountType</code>.\n<p>If a corresponding user already exists with the same account identifier,\nthe user will be updated with the resource. In this case only the\n<code>displayName</code> field can be changed.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Looks up a user by primary email address.\nThis is only supported for Google-managed users.  Lookup of the id is not\nneeded for EMM-managed users because the id is already returned in the\nresult of the Users.insert call.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("revoke_device_access").about("Revokes access to all devices currently provisioned to the user. The user\nwill no longer be able to use the managed Play store on any of their\nmanaged devices.\n\nThis call only works with EMM-managed accounts.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_available_product_set").about("Modifies the set of products that a user is entitled to access (referred to\nas <em>whitelisted</em> products). Only products that are\n<a href=\"/android/work/play/emm-api/v1/products/approve\">approved</a>\nor products that were previously approved (products with revoked approval)\ncan be whitelisted.");
            users0 = users0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the details of an EMM-managed user.\n\nCan be used with EMM-managed users only (not Google managed users).\nPass the new details in the\n<a href=\"/android/work/play/emm-api/v1/users.html\">Users</a>\nresource in the request body. Only the <code>displayName</code> field\ncan be changed. Other fields must either be unset or have the\ncurrently active value.");
            users0 = users0.subcommand(mcmd);
        }
        let mut webapps0 = SubCommand::with_name("webapps")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an existing web app.");
            webapps0 = webapps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an existing web app.");
            webapps0 = webapps0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("insert").about("Creates a new web app for the enterprise.");
            webapps0 = webapps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieves the details of all web apps for a given enterprise.");
            webapps0 = webapps0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing web app.");
            webapps0 = webapps0.subcommand(mcmd);
        }
        app = app.subcommand(webapps0);
        app = app.subcommand(users0);
        app = app.subcommand(storelayoutpages0);
        app = app.subcommand(storelayoutclusters0);
        app = app.subcommand(serviceaccountkeys0);
        app = app.subcommand(products0);
        app = app.subcommand(permissions0);
        app = app.subcommand(managedconfigurationssettings0);
        app = app.subcommand(managedconfigurationsforuser0);
        app = app.subcommand(managedconfigurationsfordevice0);
        app = app.subcommand(installs0);
        app = app.subcommand(grouplicenseusers0);
        app = app.subcommand(grouplicenses0);
        app = app.subcommand(entitlements0);
        app = app.subcommand(enterprises0);
        app = app.subcommand(devices0);

        Self { app }
    }
}
use google_androidenterprise1 as api;

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

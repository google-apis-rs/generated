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
        let mut app = App::new("apigee1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20200618")
            .about("The Apigee API lets you programmatically manage Apigee hybrid with a set of RESTful operations, including:<ul>  <li>Create, edit, and delete API proxies</li>  <li>Manage users</li>  <li>Deploy and undeploy proxy revisions</li>  <li>Configure environments</li></ul><p>For information on using the APIs described in this section, see <a href=\"docs.apigee.com/hybrid/latest/api-get-started\">Get started using the APIs</a>.</p><p><strong>Note:</strong> This product is available as a free trial for a time period of 60 days.")
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
        let mut hybrid0 = SubCommand::with_name("hybrid")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: issuers");
        let mut organizations0 = SubCommand::with_name("organizations")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, get, get_sync_authorization, list, set_sync_authorization and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an Apigee organization. See\n[Create an\norganization](https://docs.apigee.com/hybrid/latest/precog-provision).");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the profile for an Apigee organization.\nSee\n[Organizations](https://docs.apigee.com/hybrid/latest/terminology#organizations).");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_sync_authorization").about("Lists the service accounts with the permissions required to allow\nthe Synchronizer to download environment data from the control plane.\n\nAn ETag is returned in the response to `getSyncAuthorization`.\nPass that ETag when calling [setSyncAuthorization](setSyncAuthorization)\nto ensure that you are updating the correct version. If you don\'t pass the\nETag in the call to `setSyncAuthorization`, then the existing authorization\nis overwritten indiscriminately.\n\nFor more information, see\n[Enable Synchronizer\naccess](https://docs.apigee.com/hybrid/latest/synchronizer-access#enable-synchronizer-access).\n\n**Note**: Available to Apigee hybrid only.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the Apigee organizations and associated GCP projects that you have\npermission to access. See\n[Organizations](https://docs.apigee.com/hybrid/latest/terminology#organizations).");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_sync_authorization").about("Sets the permissions required to allow the Synchronizer to download\nenvironment data from the control plane. You must call this API to enable\nproper functioning of hybrid.\n\nPass the ETag when calling `setSyncAuthorization` to ensure that\nyou are updating the correct version. To get an ETag,\ncall [getSyncAuthorization](getSyncAuthorization).\nIf you don\'t pass the ETag in the call to `setSyncAuthorization`, then the\nexisting authorization is overwritten indiscriminately.\n\nFor more information, see\n[Enable Synchronizer\naccess](https://docs.apigee.com/hybrid/latest/synchronizer-access#enable-synchronizer-access).\n\n**Note**: Available to Apigee hybrid only.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the properties for an Apigee organization. No other fields in the\norganization profile will be updated.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        let mut issuers1 = SubCommand::with_name("issuers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists hybrid services and its trusted issuers service account ids.\nThis api is authenticated and unauthorized(allow all the users) and used by\nruntime authn-authz service to query control plane\'s issuer service account\nids.");
            issuers1 = issuers1.subcommand(mcmd);
        }
        let mut apiproducts1 = SubCommand::with_name("apiproducts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: attributes_method, create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("attributes_method").about("Updates or creates API product attributes. This API **replaces** the\ncurrent list of attributes with the attributes specified in the request\nbody. In this way, you can update existing attributes, add new attributes,\nor delete existing attributes by omitting them from the request body.\n\nOAuth access tokens and Key Management Service (KMS) entities (apps,\ndevelopers, and API products) are cached for 180 seconds (current default).\nAny custom attributes associated with entities also get cached for at least\n180 seconds after entity is accessed during runtime.\nIn this case, the `ExpiresIn` element on the OAuthV2 policy won\'t be able\nto expire an access token in less than 180 seconds.");
            apiproducts1 = apiproducts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates an API product in an organization.\nYou create API products after\nyou have proxied backend services using API proxies.\nAn API product is a\ncollection of API resources combined with quota settings and metadata that\nyou can use to deliver customized and productized API bundles to your\ndeveloper community. This metadata can include:\n\n- Scope\n- Environments\n- API proxies\n- Extensible profile\n\nAPI products enable you repackage APIs\non-the-fly, without having to do any additional coding or configuration.\nApigee recommends that you start with a simple API product including only\nrequired elements. You then provision credentials to apps to enable them to\nstart testing your APIs.\n\nAfter you have authentication and authorization\nworking against a simple API product, you can iterate to create finer\ngrained API products, defining different sets of API resources for each API\nproduct.\n\n<aside class=\"warning\"><strong>WARNING:</strong>\n\n- If you don\'t specify an API proxy in the request body, <em>any</em> app\nassociated with the product can make calls to <em>any</em> API in your\nentire organization.\n- If you don\'t specify an environment in the request body, the product\nallows access to all environments.\n\n</aside>\n\nFor more information, see {{what_api_product}}");
            apiproducts1 = apiproducts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an API product from an organization.\n\nDeleting an API product\ncauses app requests to the resource URIs defined in the API product to\nfail.\n\nEnsure that you create a new API product to serve existing apps, unless\nyour intention is to disable access to the resources defined in the API\nproduct.\n\nThe API product name required in the request URL is the internal name of\nthe product, not the display name. While they may be the same, it depends\non whether the API product was created via the UI or the API. View the list\nof API products to verify the internal name.");
            apiproducts1 = apiproducts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets configuration details for an API product.\n\nThe API product name required in the request URL is the internal name of\nthe product, not the display name. While they may be the same, it depends\non whether the API product was created via the UI or the API. View the list\nof API products to verify the internal name.");
            apiproducts1 = apiproducts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all API product names for an organization.\nFilter the list by passing an `attributename` and `attibutevalue`.\n\nThe limit on the number of API products returned by the API is 1000. You\ncan paginate the list of API products returned using the `startKey` and\n`count` query parameters.");
            apiproducts1 = apiproducts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing API product. You must include all required values,\nwhether or not you are updating them, as well as any optional values that\nyou are updating.\n\nThe API product name required in the request URL is the\ninternal name of the product, not the Display Name. While they may be the\nsame, it depends on whether the API product was created via UI or API. View\nthe list of API products to identify their internal names.");
            apiproducts1 = apiproducts1.subcommand(mcmd);
        }
        let mut apis1 = SubCommand::with_name("apis")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an API proxy.\nThe API proxy created will not be accessible at runtime until it is\ndeployed to an environment.\n\nCreate a new API proxy by setting the `name` query parameter to the\nname of the API proxy.\n\nImport an API proxy configuration bundle stored in zip format\non your local machine to your organization by doing the following:\n\n* Set the `name` query parameter to the name of the API proxy.\n* Set the `action` query parameter to `import`.\n* Set the `Content-Type` header to `multipart/form-data`.\n* Pass as a file the name of API proxy\n  configuration bundle stored in zip format on your local machine using\n  the `file` form field.\n\n**Note**: To validate the API proxy configuration bundle only\n  without importing it, set the `action` query\n  parameter to `validate`.\n\nWhen importing an API proxy configuration bundle, if the API proxy\ndoes not exist, it will be created.\nIf the API proxy exists, then a new revision is created. Invalid API\nproxy configurations are rejected, and a list of validation errors is\nreturned to the client.");
            apis1 = apis1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an API proxy and all associated endpoints, policies, resources, and\nrevisions. The API proxy must be undeployed before you can delete it.");
            apis1 = apis1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets an API proxy including a list of existing revisions.");
            apis1 = apis1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the names of all API proxies in an organization. The names returned\ncorrespond to the names defined in the configuration files for each API\nproxy.");
            apis1 = apis1.subcommand(mcmd);
        }
        let mut apps1 = SubCommand::with_name("apps")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the app profile for the specified app ID.");
            apps1 = apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists IDs of apps within an organization that have the specified app status\n(approved or revoked) or are of the specified app type\n(developer or company).");
            apps1 = apps1.subcommand(mcmd);
        }
        let mut deployments1 = SubCommand::with_name("deployments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all deployments of API proxies or shared flows.");
            deployments1 = deployments1.subcommand(mcmd);
        }
        let mut developers1 = SubCommand::with_name("developers")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: attributes_method, create, delete, get, list, set_developer_status and update");
        {
            let mcmd = SubCommand::with_name("attributes_method").about("Updates developer attributes.\n\nThis API replaces the\nexisting attributes with those specified in the request.\nAdd new attributes, and include or exclude any existing\nattributes that you want to retain or\nremove, respectively.\n\nThe custom attribute limit is 18.\n\n**Note**: OAuth access tokens and Key Management Service (KMS) entities\n(apps, developers, and API products) are cached for 180 seconds\n(default). Any custom attributes associated with these entities\nare cached for at least 180 seconds after the entity is accessed at\nruntime. Therefore, an `ExpiresIn` element on the OAuthV2 policy\nwon\'t be able to expire an access token in less than 180 seconds.");
            developers1 = developers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a developer. Once created,\nthe developer can register an app and obtain an API key.\n\nAt creation time, a developer is set as `active`. To change the developer\nstatus, use the SetDeveloperStatus API.");
            developers1 = developers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a developer. All apps and API keys associated\nwith the developer are also removed.\n\n**Warning**: This API will permanently delete the developer\nand related artifacts.\n\nTo avoid permanently deleting developers and their artifacts,\nset the developer status to `inactive` using\nthe SetDeveloperStatus API.\n\n**Note**: The delete operation is asynchronous. The developer app is\ndeleted immediately,\nbut its associated resources, such as apps and API keys, may take anywhere\nfrom a few seconds to a few minutes to be deleted.");
            developers1 = developers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the developer details, including the\ndeveloper\'s name, email address, apps, and other information.\n\n**Note**: The response includes only the first 100 developer apps.");
            developers1 = developers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all developers in an organization by email address.\n\nBy default,\nthe response does not include company developers. Set the `includeCompany`\nquery parameter to `true` to include company developers.\n\n**Note**: A maximum of 1000 developers are returned in the response. You\npaginate the list of developers returned using the `startKey` and `count`\nquery parameters.");
            developers1 = developers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_developer_status").about("Sets the status of a developer. Valid values are `active` or `inactive`.\n\nA developer is `active` by default. If you set a developer\'s status to\n`inactive`, the API keys assigned to the developer apps are no longer valid\neven though the API keys are set to `approved`. Inactive developers\ncan still sign in to the developer portal and create apps; however, any\nnew API keys generated during app creation won\'t work.\n\nIf successful, the API call returns the\nfollowing HTTP status code: `204 No Content`");
            developers1 = developers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a developer.\n\nThis API replaces the existing developer details with those specified\nin the request. Include or exclude any existing details that\nyou want to retain or delete, respectively.\n\nThe custom attribute limit is 18.\n\n**Note**: OAuth access tokens and Key Management Service (KMS) entities\n(apps, developers, and API products) are cached for 180 seconds\n(current default). Any custom attributes associated with these entities\nare cached for at least 180 seconds after the entity is accessed at\nruntime. Therefore, an `ExpiresIn` element on the OAuthV2 policy\nwon\'t be able to expire an access token in less than 180 seconds.");
            developers1 = developers1.subcommand(mcmd);
        }
        let mut environments1 = SubCommand::with_name("environments")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_debugmask, get_deployed_config, get_iam_policy, set_iam_policy, subscribe, test_iam_permissions, unsubscribe, update, update_debugmask and update_environment");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates an environment in an organization.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes an environment from an organization.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets environment details.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_debugmask")
                .about("Gets the debug mask singleton resource for an environment.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_deployed_config")
                .about("Gets the deployed configuration for an environment.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the IAM policy on an environment. For more information, see\n[Manage users, roles, and permissions\nusing the API](https://docs.apigee.com/hybrid/latest/manage-users-roles).\n\nYou must have the `apigee.environments.getIamPolicy` permission to call\nthis API.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the IAM policy on an environment, if the policy already\nexists it will be replaced. For more information, see\n[Manage users, roles, and permissions\nusing the API](https://docs.apigee.com/hybrid/latest/manage-users-roles).\n\nYou must have the `apigee.environments.setIamPolicy` permission to\ncall this API.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("subscribe").about("Creates a subscription for the environment\'s Pub/Sub topic.\nThe server will assign a random name for this subscription.\nThe \"name\" and \"push_config\" must *not* be specified.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Tests the permissions of a user on an environment,\nand returns a subset of permissions that the user has on the environment.\nIf the environment does not exist, an empty permission set is returned\n(a NOT_FOUND error is not returned).");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unsubscribe")
                .about("Deletes a subscription for the environment\'s Pub/Sub topic.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing environment.\n\nWhen updating properties, you must pass all existing properties to the API,\neven if they are not being changed. If you omit properties from the\npayload, the properties are removed. To get the current list of\nproperties for the environment, use the [Get Environment API](get).");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_debugmask")
                .about("Updates the debug mask singleton resource for an environment.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_environment").about("Updates an existing environment.\n\nWhen updating properties, you must pass all existing properties to the API,\neven if they are not being changed. If you omit properties from the\npayload, the properties are removed. To get the current list of\nproperties for the environment, use the [Get Environment API](get).");
            environments1 = environments1.subcommand(mcmd);
        }
        let mut keyvaluemaps1 = SubCommand::with_name("keyvaluemaps")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and delete");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a key value map in an organization.");
            keyvaluemaps1 = keyvaluemaps1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Delete a key value map in an organization.");
            keyvaluemaps1 = keyvaluemaps1.subcommand(mcmd);
        }
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the\nserver doesn\'t support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut reports1 = SubCommand::with_name("reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Custom Report for an Organization. A Custom Report\nprovides Apigee Customers to create custom dashboards in addition\nto the standard dashboards which are provided. The Custom Report in its\nsimplest form contains specifications about metrics, dimensions and\nfilters. It is important to note that the custom report by itself does not\nprovide an executable entity. The Edge UI converts the custom report\ndefinition into an analytics query and displays the result in a chart.");
            reports1 = reports1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes an existing custom report definition");
            reports1 = reports1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve a custom report definition.");
            reports1 = reports1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Return a list of Custom Reports");
            reports1 = reports1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Update an existing custom report definition");
            reports1 = reports1.subcommand(mcmd);
        }
        let mut sharedflows1 = SubCommand::with_name("sharedflows")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Uploads a ZIP-formatted shared flow configuration bundle to an\norganization. If the shared flow already exists, this creates a new\nrevision of it. If the shared flow does not exist, this creates it.\n\nOnce imported, the shared flow revision must be deployed before it can be\naccessed at runtime.\n\nThe size limit of a shared flow bundle is 15 MB.");
            sharedflows1 = sharedflows1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a shared flow and all it\'s revisions. The shared flow must be\nundeployed before you can delete it.");
            sharedflows1 = sharedflows1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a shared flow by name, including a list of its revisions.");
            sharedflows1 = sharedflows1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all shared flows in the organization.");
            sharedflows1 = sharedflows1.subcommand(mcmd);
        }
        let mut attributes2 = SubCommand::with_name("attributes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list and update_api_product_attribute");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an API product attribute.");
            attributes2 = attributes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns the value of an API product attribute.");
            attributes2 = attributes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns a list of all API product attributes.");
            attributes2 = attributes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_api_product_attribute").about("Updates the value of an API product attribute. Limitations are:\n\nOAuth access tokens and Key Management Service (KMS) entities (apps,\ndevelopers, and API products) are cached for 180 seconds (current default).\nAny custom attributes associated with entities also get cached for at least\n180 seconds after entity is accessed during runtime.\nIn this case, the `ExpiresIn` element on the OAuthV2 policy won\'t be able\nto expire an access token in less than 180 seconds.");
            attributes2 = attributes2.subcommand(mcmd);
        }
        let mut deployments2 = SubCommand::with_name("deployments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all deployments of an API proxy.");
            deployments2 = deployments2.subcommand(mcmd);
        }
        let mut keyvaluemaps2 = SubCommand::with_name("keyvaluemaps")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and delete");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a key value map in an api proxy.");
            keyvaluemaps2 = keyvaluemaps2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Delete a key value map in an api proxy.");
            keyvaluemaps2 = keyvaluemaps2.subcommand(mcmd);
        }
        let mut revisions2 = SubCommand::with_name("revisions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and update_api_proxy_revision");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an API proxy revision and all policies, resources, endpoints,\nand revisions associated with it. The API proxy revision must be undeployed\nbefore you can delete it.");
            revisions2 = revisions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an API proxy revision.\n\nTo download the API proxy configuration bundle for the specified revision\nas a zip file, do the following:\n\n * Set the `format` query parameter to `bundle`.\n * Set the `Accept` header to `application/zip`.\n\nIf you are using curl, specify `-o filename.zip` to save the output to a\nfile; otherwise, it displays to `stdout`. Then, develop the API proxy\nconfiguration locally and upload the updated API proxy configuration\nrevision, as described in\n[updateApiProxyRevision](updateApiProxyRevision).");
            revisions2 = revisions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_api_proxy_revision").about("Updates an existing API proxy revision by uploading the API proxy\nconfiguration bundle as a zip file from your local machine.\n\nYou can update only API proxy revisions\nthat have never been deployed. After deployment, an API proxy revision\nbecomes immutable, even if it is undeployed.\n\nSet the `Content-Type` header to either\n`multipart/form-data` or `application/octet-stream`.");
            revisions2 = revisions2.subcommand(mcmd);
        }
        let mut apps2 = SubCommand::with_name("apps")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: attributes_method, create, delete, generate_key_pair_or_update_developer_app_status, get, list and update");
        {
            let mcmd = SubCommand::with_name("attributes_method").about("Updates attributes for a developer app. This API replaces the\ncurrent attributes with those specified in the request.");
            apps2 = apps2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates an app associated with a developer. This API associates the\ndeveloper app with the specified API\nproduct and auto-generates an API key for the app to use in calls to API\nproxies inside that API product.\n\nThe `name` is the unique ID of the app\nthat you can use in API calls. The `DisplayName` (set as an\nattribute) appears in the UI. If you don\'t set the\n`DisplayName` attribute, the `name` appears in the UI.");
            apps2 = apps2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a developer app.\n\n**Note**: The delete operation is asynchronous. The developer app is\ndeleted immediately,\nbut its associated resources, such as app\nkeys or access tokens, may take anywhere from a few seconds to a\nfew minutes to be deleted.");
            apps2 = apps2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_key_pair_or_update_developer_app_status").about("Manages access to a developer app by enabling you to:\n\n* Approve or revoke a developer app\n* Generate a new consumer key and secret for a developer app\n\nTo approve or revoke a developer app, set the `action` query parameter to\n`approved` or `revoked`, respectively, and the\n`Content-Type` header to `application/octet-stream`. If a developer app is\nrevoked, none of its API keys are valid for API calls even though\nthe keys are still `approved`. If successful, the API call returns the\nfollowing HTTP status code: `204 No Content`\n\nTo generate a new consumer key and secret for a developer\napp, pass the new key/secret details. Rather than\nreplace an existing key, this API generates a new\nkey. In this case, multiple key\npairs may be associated with a single developer app. Each key pair has an\nindependent status (`approved` or `revoked`) and expiration time.\nAny approved, non-expired key can be used in an API call.\n\nFor example, if you\'re using API key rotation, you can generate new\nkeys with expiration times that overlap keys that are going to expire.\nYou might also generate a new consumer key/secret if the security of the\noriginal key/secret is compromised.\n\nThe `keyExpiresIn` property defines the\nexpiration time for the API key in milliseconds. If you don\'t set\nthis property or set it to `-1`, the API key never expires.\n\n**Notes**:\n\n* When generating a new key/secret, this API replaces the\nexisting attributes, notes, and callback URLs with those specified in the\nrequest. Include or exclude any existing information that you want to\nretain or delete, respectively.\n* To migrate existing consumer keys and secrets to hybrid from another\nsystem, see the\nCreateDeveloperAppKey API.");
            apps2 = apps2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Returns the details for a developer app.");
            apps2 = apps2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all apps created by a developer in an Apigee organization.\nOptionally, you can request an expanded view of the developer apps.\n\nA maximum of 100 developer apps are returned per API call. You can paginate\nthe list of deveoper apps returned using the `startKey` and `count` query\nparameters.");
            apps2 = apps2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the details for a developer app. In addition, you can\nadd an API product to a developer app and automatically generate\nan API key for the app to use when calling APIs in the API product.\n\nIf you want to use an existing API key for the API product,\nadd the API product to the API key using the\nUpdateDeveloperAppKey\nAPI.\n\nUsing this API, you cannot update the following:\n\n* App name as it is the primary key used to identify the app and cannot\n  be changed.\n* Scopes associated with the app. Instead, use the\n  ReplaceDeveloperAppKey API.\n\nThis API replaces the\nexisting attributes with those specified in the request.\nInclude or exclude any existing attributes that you want to retain or\ndelete, respectively.");
            apps2 = apps2.subcommand(mcmd);
        }
        let mut attributes2 = SubCommand::with_name("attributes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list and update_developer_attribute");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a developer attribute.");
            attributes2 = attributes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns the value of the specified developer attribute.");
            attributes2 = attributes2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Returns a list of all developer attributes.");
            attributes2 = attributes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_developer_attribute").about("Updates a developer attribute.\n\n**Note**: OAuth access tokens and Key Management Service (KMS) entities\n(apps, developers, and API products) are cached for 180 seconds\n(default). Any custom attributes associated with these entities\nare cached for at least 180 seconds after the entity is accessed at\nruntime. Therefore, an `ExpiresIn` element on the OAuthV2 policy\nwon\'t be able to expire an access token in less than 180 seconds.");
            attributes2 = attributes2.subcommand(mcmd);
        }
        let mut analytics2 = SubCommand::with_name("analytics")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: admin");
        let mut apis2 = SubCommand::with_name("apis")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: deployments and revisions");
        let mut caches2 = SubCommand::with_name("caches")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a cache.");
            caches2 = caches2.subcommand(mcmd);
        }
        let mut deployments2 = SubCommand::with_name("deployments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all deployments of API proxies or shared flows in an environment.");
            deployments2 = deployments2.subcommand(mcmd);
        }
        let mut flowhooks2 = SubCommand::with_name("flowhooks")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: attach_shared_flow_to_flow_hook, detach_shared_flow_from_flow_hook and get");
        {
            let mcmd = SubCommand::with_name("attach_shared_flow_to_flow_hook")
                .about("Attaches a shared flow to a flow hook.");
            flowhooks2 = flowhooks2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("detach_shared_flow_from_flow_hook")
                .about("Detaches a shared flow from a flow hook.");
            flowhooks2 = flowhooks2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the name of the shared flow attached to the specified flow hook. If\nthere\'s no shared flow attached to the flow hook, the API does not return\nan error; it simply does not return a name in the response.");
            flowhooks2 = flowhooks2.subcommand(mcmd);
        }
        let mut keystores2 = SubCommand::with_name("keystores")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete and get");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a keystore or truststore:\n * Keystore: Contains certificates and their associated keys.\n * Truststore: Contains trusted certificates used to validate a\n server\'s certificate. These certificates are typically self-signed\n certificates or certificates that are not signed by a trusted CA.");
            keystores2 = keystores2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a keystore or truststore.");
            keystores2 = keystores2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a keystore or truststore.");
            keystores2 = keystores2.subcommand(mcmd);
        }
        let mut keyvaluemaps2 = SubCommand::with_name("keyvaluemaps")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and delete");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a key value map in an environment.");
            keyvaluemaps2 = keyvaluemaps2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Delete a key value map in an environment.");
            keyvaluemaps2 = keyvaluemaps2.subcommand(mcmd);
        }
        let mut optimized_stats2 = SubCommand::with_name("optimized_stats")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("This api is similar to GetStats\nexcept that the response is less verbose.\nIn the current scheme, a query parameter _optimized instructs\nEdge Analytics to change the response but since this behavior\nis not possible with protocol buffer and since this parameter is\npredominantly used by Edge UI, we are introducing a separate api.");
            optimized_stats2 = optimized_stats2.subcommand(mcmd);
        }
        let mut queries2 = SubCommand::with_name("queries")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, get_result and list");
        {
            let mcmd = SubCommand::with_name("create").about("Submit a query to be processed in the background.\nIf the submission of the query succeeds, the API returns a 201 status and\nan ID that refer to the query. In addition to the HTTP status 201, the\n`state` of \"enqueued\" means that the request succeeded.");
            queries2 = queries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get query status\nIf the query is still in progress, the `state` is set to \"running\"\nAfter the query has completed successfully, `state` is set to \"completed\"");
            queries2 = queries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_result").about("After the query is completed, use this API to retrieve the results.\nIf the request succeeds, and there is a non-zero result set, the result is\ndownloaded to the client as a zipped JSON file.\nThe name of the downloaded file will be:\n  OfflineQueryResult-<query-id>.zip\n\nExample: `OfflineQueryResult-9cfc0d85-0f30-46d6-ae6f-318d0cb961bd.zip`");
            queries2 = queries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Return a list of Asynchronous Queries");
            queries2 = queries2.subcommand(mcmd);
        }
        let mut references2 = SubCommand::with_name("references")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and update");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a Reference in the specified environment.");
            references2 = references2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Deletes a Reference from an environment. Returns the deleted\nReference resource.",
            );
            references2 = references2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Reference resource.");
            references2 = references2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing Reference. Note that this operation has PUT\nsemantics; it will replace the entirety of the existing Reference with\nthe resource in the request body.");
            references2 = references2.subcommand(mcmd);
        }
        let mut resourcefiles2 = SubCommand::with_name("resourcefiles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, list_environment_resources and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a resource file.\n\nSpecify the `Content-Type` as `application/octet-stream` or\n`multipart/form-data`.\n\nFor more information about resource files, see\n[Resource files](/api-platform/develop/resource-files).");
            resourcefiles2 = resourcefiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a resource file.\n\nFor more information about resource files, see\n[Resource files](/api-platform/develop/resource-files).");
            resourcefiles2 = resourcefiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the contents of a resource file.\n\nFor more information about resource files, see\n[Resource files](/api-platform/develop/resource-files).");
            resourcefiles2 = resourcefiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all resource files.\n\nFor more information about resource files, see\n[Resource files](/api-platform/develop/resource-files).");
            resourcefiles2 = resourcefiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_environment_resources").about("Lists all resource files.\n\nFor more information about resource files, see\n[Resource files](/api-platform/develop/resource-files).");
            resourcefiles2 = resourcefiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a resource file.\n\nSpecify the `Content-Type` as `application/octet-stream` or\n`multipart/form-data`.\n\nFor more information about resource files, see\n[Resource files](/api-platform/develop/resource-files).");
            resourcefiles2 = resourcefiles2.subcommand(mcmd);
        }
        let mut sharedflows2 = SubCommand::with_name("sharedflows")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: deployments and revisions");
        let mut stats2 = SubCommand::with_name("stats")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve metrics grouped by dimensions.\nThe types of metrics you can retrieve include traffic, message counts,\nAPI call latency, response size, and cache hits and counts.\nDimensions let you view metrics in meaningful groups.\nThe stats api does accept dimensions as path params. The dimensions are\noptional in which case the metrics are computed on the entire data\nfor the given timerange.");
            stats2 = stats2.subcommand(mcmd);
        }
        let mut targetservers2 = SubCommand::with_name("targetservers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and update");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a TargetServer in the specified environment.");
            targetservers2 = targetservers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a TargetServer from an environment. Returns the deleted\nTargetServer resource.");
            targetservers2 = targetservers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a TargetServer resource.");
            targetservers2 = targetservers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing TargetServer. Note that this operation has PUT\nsemantics; it will replace the entirety of the existing TargetServer with\nthe resource in the request body.");
            targetservers2 = targetservers2.subcommand(mcmd);
        }
        let mut deployments2 = SubCommand::with_name("deployments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all deployments of a shared flow.");
            deployments2 = deployments2.subcommand(mcmd);
        }
        let mut revisions2 = SubCommand::with_name("revisions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and update_shared_flow_revision");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a shared flow and all associated policies, resources, and\nrevisions. You must undeploy the shared flow before deleting it.");
            revisions2 = revisions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a revision of a shared flow.\n\nIf `format=bundle` is passed, it instead outputs a shared flow revision as\na ZIP-formatted bundle of code and config files.");
            revisions2 = revisions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_shared_flow_revision").about("Updates a shared flow revision. This operation is only allowed on revisions\nwhich have never been deployed. After deployment a revision becomes\nimmutable, even if it becomes undeployed.\n\nThe payload is a ZIP-formatted shared flow.  Content type must be either\nmultipart/form-data or application/octet-stream.");
            revisions2 = revisions2.subcommand(mcmd);
        }
        let mut deployments3 = SubCommand::with_name("deployments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all deployments of an API proxy revision.");
            deployments3 = deployments3.subcommand(mcmd);
        }
        let mut attributes3 = SubCommand::with_name("attributes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list and update_developer_app_attribute");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a developer app attribute.");
            attributes3 = attributes3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a developer app attribute.");
            attributes3 = attributes3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns a list of all developer app attributes.");
            attributes3 = attributes3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_developer_app_attribute").about("Updates a developer app attribute.\n\n**Note**: OAuth access tokens and Key Management Service (KMS) entities\n(apps, developers, and API products) are cached for 180 seconds\n(current default). Any custom attributes associated with these entities\nare cached for at least 180 seconds after the entity is accessed at\nruntime. Therefore, an `ExpiresIn` element on the OAuthV2 policy\nwon\'t be able to expire an access token in less than 180 seconds.");
            attributes3 = attributes3.subcommand(mcmd);
        }
        let mut keys3 = SubCommand::with_name("keys")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create_method, delete, get, replace_developer_app_key and update_developer_app_key");
        {
            let mcmd = SubCommand::with_name("create_method").about("Creates a custom consumer key and secret for a developer app. This is\nparticularly useful if you want to migrate existing consumer keys and\nsecrets to Apigee hybrid from another system.\n\nConsumer keys and secrets can contain letters, numbers, underscores, and\nhyphens. No other special characters are allowed. To avoid service\ndisruptions, a consumer key and secret should not exceed 2 KBs each.\n\n**Note**: When creating the consumer key and secret, an association to\nAPI products will not be made. Therefore, you should not specify the\nassociated API products in your request. Instead, use the\nUpdateDeveloperAppKey API to\nmake the association after the consumer key and secret are created.\n\nIf a consumer key and secret already exist, you can keep them or\ndelete them using the\nDeleteDeveloperAppKey API.");
            keys3 = keys3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an app\'s consumer key and removes all API products\nassociated with the app. After the consumer key is deleted,\nit cannot be used to access any APIs.\n\n**Note**: After you delete a consumer key, you may want to:\n1. Create a new consumer key and secret for the developer app using the\nCreateDeveloperAppKey API, and\nsubsequently add an API product to the key using the\nUpdateDeveloperAppKey API.\n2. Delete the developer app, if it is no longer required.");
            keys3 = keys3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns details for a consumer key for a developer app, including the key\nand secret value, associated API products, and other information.");
            keys3 = keys3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_developer_app_key").about("Updates the scope of an app.\n\nThis API replaces the\nexisting scopes with those specified in the request.\nInclude or exclude any existing scopes that you want to retain or\ndelete, respectively. The specified scopes must already\nbe defined for the API products associated with the app.\n\nThis API sets the `scopes` element\nunder the `apiProducts` element in the attributes of the app.");
            keys3 = keys3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_developer_app_key").about("Adds an API product to a developer app key, enabling the app that holds\nthe key to access the API resources bundled in the API product.\n\nIn addition, you can add\nattributes to a developer app key. This API replaces the\nexisting attributes with those specified in the request.\nInclude or exclude any existing attributes that you want to retain or\ndelete, respectively.\n\nYou can use the same key to access all API products\nassociated with the app.");
            keys3 = keys3.subcommand(mcmd);
        }
        let mut admin3 = SubCommand::with_name("admin")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_schemav_2");
        {
            let mcmd = SubCommand::with_name("get_schemav_2").about("Get a list of metrics and dimensions which can be used for creating\nanalytics queries and reports.\nEach schema element contains the name of the field with its associated type\nand if it is either custom field or standard field.");
            admin3 = admin3.subcommand(mcmd);
        }
        let mut deployments3 = SubCommand::with_name("deployments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all deployments of an API proxy in an environment.");
            deployments3 = deployments3.subcommand(mcmd);
        }
        let mut revisions3 = SubCommand::with_name("revisions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: deployments and get_deployments");
        {
            let mcmd = SubCommand::with_name("deployments").about("Undeploys an API proxy revision from an environment.\n\nBecause multiple revisions of the same API proxy can be deployed in\nthe same environment if the base paths are different, you must specify the\nrevision number of the API proxy.");
            revisions3 = revisions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_deployments").about("Gets the deployment of an API proxy revision and actual state reported by\nruntime pods.");
            revisions3 = revisions3.subcommand(mcmd);
        }
        let mut aliases3 = SubCommand::with_name("aliases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, csr, delete, get, get_certificate and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an alias from a key, certificate pair.\nThe structure of the request is controlled by the `format` query parameter:\n * `keycertfile` - Separate PEM-encoded key and certificate files are\n uploaded. The request must have `Content-Type: multipart/form-data` and\n include fields `keyFile` and `certFile`. If uploading to a truststore,\n omit `keyFile`.\n* `pkcs12` - A PKCS12 file is uploaded. The request must have\n`Content-Type: multipart/form-data` with the file provided in the only\nfield.\n* `selfsignedcert` - A new private key and certificate are generated. The\nrequest must have `Content-Type: application/json` and a body of\nCertificateGenerationSpec.");
            aliases3 = aliases3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("csr").about("Generates a PKCS #10 Certificate Signing Request for the private key in\nan alias.");
            aliases3 = aliases3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an alias.");
            aliases3 = aliases3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an alias.");
            aliases3 = aliases3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_certificate")
                .about("Gets the certificate from an alias in PEM-encoded form.");
            aliases3 = aliases3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Updates the certificate in an alias.");
            aliases3 = aliases3.subcommand(mcmd);
        }
        let mut deployments3 = SubCommand::with_name("deployments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all deployments of a shared flow in an environment.");
            deployments3 = deployments3.subcommand(mcmd);
        }
        let mut revisions3 = SubCommand::with_name("revisions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: deployments and get_deployments");
        {
            let mcmd = SubCommand::with_name("deployments")
                .about("Undeploys a shared flow revision from an environment.");
            revisions3 = revisions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_deployments").about("Gets the deployment of a shared flow revision and actual state reported by\nruntime pods.");
            revisions3 = revisions3.subcommand(mcmd);
        }
        let mut deployments3 = SubCommand::with_name("deployments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all deployments of a shared flow revision.");
            deployments3 = deployments3.subcommand(mcmd);
        }
        let mut apiproducts4 = SubCommand::with_name("apiproducts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete and update_developer_app_key_api_product");
        {
            let mcmd = SubCommand::with_name("delete").about("Removes an API product from an app\'s consumer key. After the API product is\nremoved, the app cannot access the API resources defined in\nthat API product.\n\n**Note**: The consumer key is not removed, only its association with the\nAPI product.");
            apiproducts4 = apiproducts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_developer_app_key_api_product").about("Approve or revoke an app\'s consumer key. After a consumer key is approved,\nthe app can use it to access APIs.\n\nA consumer key that is revoked or pending cannot be used to access an API.\nAny access tokens associated with a revoked consumer key will remain\nactive. However, Apigee hybrid checks the status of the consumer key and\nif set to `revoked` will not allow access to the API.");
            apiproducts4 = apiproducts4.subcommand(mcmd);
        }
        let mut create4 = SubCommand::with_name("create")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a custom consumer key and secret for a developer app. This is\nparticularly useful if you want to migrate existing consumer keys and\nsecrets to Apigee hybrid from another system.\n\nConsumer keys and secrets can contain letters, numbers, underscores, and\nhyphens. No other special characters are allowed. To avoid service\ndisruptions, a consumer key and secret should not exceed 2 KBs each.\n\n**Note**: When creating the consumer key and secret, an association to\nAPI products will not be made. Therefore, you should not specify the\nassociated API products in your request. Instead, use the\nUpdateDeveloperAppKey API to\nmake the association after the consumer key and secret are created.\n\nIf a consumer key and secret already exist, you can keep them or\ndelete them using the\nDeleteDeveloperAppKey API.");
            create4 = create4.subcommand(mcmd);
        }
        let mut debugsessions4 = SubCommand::with_name("debugsessions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete_data, get and list");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a debug session for a deployed API Proxy revision.");
            debugsessions4 = debugsessions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_data").about("Deletes the data from a debug session. This does not cancel the debug\nsession or prevent further data from being collected if the session is\nstill active in runtime pods.");
            debugsessions4 = debugsessions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a debug session.");
            debugsessions4 = debugsessions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists debug sessions that are currently active in the given API Proxy\nrevision.",
            );
            debugsessions4 = debugsessions4.subcommand(mcmd);
        }
        let mut data5 = SubCommand::with_name("data")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets the debug data from a transaction.");
            data5 = data5.subcommand(mcmd);
        }
        debugsessions4 = debugsessions4.subcommand(data5);
        revisions3 = revisions3.subcommand(debugsessions4);
        keys3 = keys3.subcommand(create4);
        keys3 = keys3.subcommand(apiproducts4);
        revisions2 = revisions2.subcommand(deployments3);
        sharedflows2 = sharedflows2.subcommand(revisions3);
        sharedflows2 = sharedflows2.subcommand(deployments3);
        keystores2 = keystores2.subcommand(aliases3);
        apis2 = apis2.subcommand(revisions3);
        apis2 = apis2.subcommand(deployments3);
        analytics2 = analytics2.subcommand(admin3);
        apps2 = apps2.subcommand(keys3);
        apps2 = apps2.subcommand(attributes3);
        revisions2 = revisions2.subcommand(deployments3);
        sharedflows1 = sharedflows1.subcommand(revisions2);
        sharedflows1 = sharedflows1.subcommand(deployments2);
        environments1 = environments1.subcommand(targetservers2);
        environments1 = environments1.subcommand(stats2);
        environments1 = environments1.subcommand(sharedflows2);
        environments1 = environments1.subcommand(resourcefiles2);
        environments1 = environments1.subcommand(references2);
        environments1 = environments1.subcommand(queries2);
        environments1 = environments1.subcommand(optimized_stats2);
        environments1 = environments1.subcommand(keyvaluemaps2);
        environments1 = environments1.subcommand(keystores2);
        environments1 = environments1.subcommand(flowhooks2);
        environments1 = environments1.subcommand(deployments2);
        environments1 = environments1.subcommand(caches2);
        environments1 = environments1.subcommand(apis2);
        environments1 = environments1.subcommand(analytics2);
        developers1 = developers1.subcommand(attributes2);
        developers1 = developers1.subcommand(apps2);
        apis1 = apis1.subcommand(revisions2);
        apis1 = apis1.subcommand(keyvaluemaps2);
        apis1 = apis1.subcommand(deployments2);
        apiproducts1 = apiproducts1.subcommand(attributes2);
        organizations0 = organizations0.subcommand(sharedflows1);
        organizations0 = organizations0.subcommand(reports1);
        organizations0 = organizations0.subcommand(operations1);
        organizations0 = organizations0.subcommand(keyvaluemaps1);
        organizations0 = organizations0.subcommand(environments1);
        organizations0 = organizations0.subcommand(developers1);
        organizations0 = organizations0.subcommand(deployments1);
        organizations0 = organizations0.subcommand(apps1);
        organizations0 = organizations0.subcommand(apis1);
        organizations0 = organizations0.subcommand(apiproducts1);
        hybrid0 = hybrid0.subcommand(issuers1);
        app = app.subcommand(organizations0);
        app = app.subcommand(hybrid0);

        Self { app }
    }
}
use google_apigee1 as api;

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

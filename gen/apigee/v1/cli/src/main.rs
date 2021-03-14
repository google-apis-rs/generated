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
            .version("0.1.0-20210305")
            .about("Use the Apigee API to programmatically develop and manage APIs with a set of RESTful operations. Develop and secure API proxies, deploy and undeploy API proxy revisions, monitor APIs, configure environments, manage users, and more. Note: This product is available as a free trial for a time period of 60 days.")
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
                        .about("methods: create, delete, get, get_deployed_ingress_config, get_sync_authorization, list, set_sync_authorization and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an Apigee organization. See [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org).");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete an Apigee organization. Only supported for SubscriptionType TRIAL.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the profile for an Apigee organization. See [Understanding organizations](https://cloud.google.com/apigee/docs/api-platform/fundamentals/organization-structure).");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_deployed_ingress_config")
                .about("Gets the deployed ingress configuration for an organization.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_sync_authorization").about("Lists the service accounts with the permissions required to allow the Synchronizer to download environment data from the control plane. An ETag is returned in the response to `getSyncAuthorization`. Pass that ETag when calling [setSyncAuthorization](setSyncAuthorization) to ensure that you are updating the correct version. If you don\'t pass the ETag in the call to `setSyncAuthorization`, then the existing authorization is overwritten indiscriminately. For more information, see [Configure the Synchronizer](https://cloud.google.com/apigee/docs/hybrid/latest/synchronizer-access). **Note**: Available to Apigee hybrid only.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the Apigee organizations and associated GCP projects that you have permission to access. See [Understanding organizations](https://cloud.google.com/apigee/docs/api-platform/fundamentals/organization-structure).");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_sync_authorization").about("Sets the permissions required to allow the Synchronizer to download environment data from the control plane. You must call this API to enable proper functioning of hybrid. Pass the ETag when calling `setSyncAuthorization` to ensure that you are updating the correct version. To get an ETag, call [getSyncAuthorization](getSyncAuthorization). If you don\'t pass the ETag in the call to `setSyncAuthorization`, then the existing authorization is overwritten indiscriminately. For more information, see [Configure the Synchronizer](https://cloud.google.com/apigee/docs/hybrid/latest/synchronizer-access). **Note**: Available to Apigee hybrid only.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the properties for an Apigee organization. No other fields in the organization profile will be updated.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: provision_organization");
        {
            let mcmd = SubCommand::with_name("provision_organization").about("Provisions a new Apigee organization with a functioning runtime. This is the standard way to create trial organizations for a free Apigee trial.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut issuers1 = SubCommand::with_name("issuers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists hybrid services and its trusted issuers service account ids. This api is authenticated and unauthorized(allow all the users) and used by runtime authn-authz service to query control plane\'s issuer service account ids.");
            issuers1 = issuers1.subcommand(mcmd);
        }
        let mut analytics1 = SubCommand::with_name("analytics")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: datastores");
        let mut apiproducts1 = SubCommand::with_name("apiproducts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: attributes_method, create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("attributes_method").about("Updates or creates API product attributes. This API **replaces** the current list of attributes with the attributes specified in the request body. In this way, you can update existing attributes, add new attributes, or delete existing attributes by omitting them from the request body. **Note**: OAuth access tokens and Key Management Service (KMS) entities (apps, developers, and API products) are cached for 180 seconds (current default). Any custom attributes associated with entities also get cached for at least 180 seconds after entity is accessed during runtime. In this case, the `ExpiresIn` element on the OAuthV2 policy won\'t be able to expire an access token in less than 180 seconds.");
            apiproducts1 = apiproducts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates an API product in an organization. You create API products after you have proxied backend services using API proxies. An API product is a collection of API resources combined with quota settings and metadata that you can use to deliver customized and productized API bundles to your developer community. This metadata can include: - Scope - Environments - API proxies - Extensible profile API products enable you repackage APIs on-the-fly, without having to do any additional coding or configuration. Apigee recommends that you start with a simple API product including only required elements. You then provision credentials to apps to enable them to start testing your APIs. After you have authentication and authorization working against a simple API product, you can iterate to create finer grained API products, defining different sets of API resources for each API product. **WARNING:** - If you don\'t specify an API proxy in the request body, *any* app associated with the product can make calls to *any* API in your entire organization. - If you don\'t specify an environment in the request body, the product allows access to all environments. For more information, see What is an API product?");
            apiproducts1 = apiproducts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an API product from an organization. Deleting an API product causes app requests to the resource URIs defined in the API product to fail. Ensure that you create a new API product to serve existing apps, unless your intention is to disable access to the resources defined in the API product. The API product name required in the request URL is the internal name of the product, not the display name. While they may be the same, it depends on whether the API product was created via the UI or the API. View the list of API products to verify the internal name.");
            apiproducts1 = apiproducts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets configuration details for an API product. The API product name required in the request URL is the internal name of the product, not the display name. While they may be the same, it depends on whether the API product was created via the UI or the API. View the list of API products to verify the internal name.");
            apiproducts1 = apiproducts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all API product names for an organization. Filter the list by passing an `attributename` and `attibutevalue`. The limit on the number of API products returned by the API is 1000. You can paginate the list of API products returned using the `startKey` and `count` query parameters.");
            apiproducts1 = apiproducts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing API product. You must include all required values, whether or not you are updating them, as well as any optional values that you are updating. The API product name required in the request URL is the internal name of the product, not the Display Name. While they may be the same, it depends on whether the API product was created via UI or API. View the list of API products to identify their internal names.");
            apiproducts1 = apiproducts1.subcommand(mcmd);
        }
        let mut apis1 = SubCommand::with_name("apis")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an API proxy. The API proxy created will not be accessible at runtime until it is deployed to an environment. Create a new API proxy by setting the `name` query parameter to the name of the API proxy. Import an API proxy configuration bundle stored in zip format on your local machine to your organization by doing the following: * Set the `name` query parameter to the name of the API proxy. * Set the `action` query parameter to `import`. * Set the `Content-Type` header to `multipart/form-data`. * Pass as a file the name of API proxy configuration bundle stored in zip format on your local machine using the `file` form field. **Note**: To validate the API proxy configuration bundle only without importing it, set the `action` query parameter to `validate`. When importing an API proxy configuration bundle, if the API proxy does not exist, it will be created. If the API proxy exists, then a new revision is created. Invalid API proxy configurations are rejected, and a list of validation errors is returned to the client.");
            apis1 = apis1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an API proxy and all associated endpoints, policies, resources, and revisions. The API proxy must be undeployed before you can delete it.");
            apis1 = apis1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets an API proxy including a list of existing revisions.");
            apis1 = apis1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the names of all API proxies in an organization. The names returned correspond to the names defined in the configuration files for each API proxy.");
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
            let mcmd = SubCommand::with_name("list").about("Lists IDs of apps within an organization that have the specified app status (approved or revoked) or are of the specified app type (developer or company).");
            apps1 = apps1.subcommand(mcmd);
        }
        let mut datacollectors1 = SubCommand::with_name("datacollectors")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new data collector.");
            datacollectors1 = datacollectors1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a data collector.");
            datacollectors1 = datacollectors1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a data collector.");
            datacollectors1 = datacollectors1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all data collectors.");
            datacollectors1 = datacollectors1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a data collector.");
            datacollectors1 = datacollectors1.subcommand(mcmd);
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
            let mcmd = SubCommand::with_name("attributes_method").about("Updates developer attributes. This API replaces the existing attributes with those specified in the request. Add new attributes, and include or exclude any existing attributes that you want to retain or remove, respectively. The custom attribute limit is 18. **Note**: OAuth access tokens and Key Management Service (KMS) entities (apps, developers, and API products) are cached for 180 seconds (default). Any custom attributes associated with these entities are cached for at least 180 seconds after the entity is accessed at runtime. Therefore, an `ExpiresIn` element on the OAuthV2 policy won\'t be able to expire an access token in less than 180 seconds.");
            developers1 = developers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a developer. Once created, the developer can register an app and obtain an API key. At creation time, a developer is set as `active`. To change the developer status, use the SetDeveloperStatus API.");
            developers1 = developers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a developer. All apps and API keys associated with the developer are also removed. **Warning**: This API will permanently delete the developer and related artifacts. To avoid permanently deleting developers and their artifacts, set the developer status to `inactive` using the SetDeveloperStatus API. **Note**: The delete operation is asynchronous. The developer app is deleted immediately, but its associated resources, such as apps and API keys, may take anywhere from a few seconds to a few minutes to be deleted.");
            developers1 = developers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the developer details, including the developer\'s name, email address, apps, and other information. **Note**: The response includes only the first 100 developer apps.");
            developers1 = developers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all developers in an organization by email address. By default, the response does not include company developers. Set the `includeCompany` query parameter to `true` to include company developers. **Note**: A maximum of 1000 developers are returned in the response. You paginate the list of developers returned using the `startKey` and `count` query parameters.");
            developers1 = developers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_developer_status").about("Sets the status of a developer. Valid values are `active` or `inactive`. A developer is `active` by default. If you set a developer\'s status to `inactive`, the API keys assigned to the developer apps are no longer valid even though the API keys are set to `approved`. Inactive developers can still sign in to the developer portal and create apps; however, any new API keys generated during app creation won\'t work. If successful, the API call returns the following HTTP status code: `204 No Content`");
            developers1 = developers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a developer. This API replaces the existing developer details with those specified in the request. Include or exclude any existing details that you want to retain or delete, respectively. The custom attribute limit is 18. **Note**: OAuth access tokens and Key Management Service (KMS) entities (apps, developers, and API products) are cached for 180 seconds (current default). Any custom attributes associated with these entities are cached for at least 180 seconds after the entity is accessed at runtime. Therefore, an `ExpiresIn` element on the OAuthV2 policy won\'t be able to expire an access token in less than 180 seconds.");
            developers1 = developers1.subcommand(mcmd);
        }
        let mut envgroups1 = SubCommand::with_name("envgroups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new environment group.");
            envgroups1 = envgroups1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an environment group.");
            envgroups1 = envgroups1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an environment group.");
            envgroups1 = envgroups1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all environment groups.");
            envgroups1 = envgroups1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an environment group.");
            envgroups1 = envgroups1.subcommand(mcmd);
        }
        let mut environments1 = SubCommand::with_name("environments")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_debugmask, get_deployed_config, get_iam_policy, get_trace_config, set_iam_policy, subscribe, test_iam_permissions, unsubscribe, update, update_debugmask, update_environment and update_trace_config");
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
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the IAM policy on an environment. For more information, see [Manage users, roles, and permissions using the API](https://cloud.google.com/apigee/docs/api-platform/system-administration/manage-users-roles). You must have the `apigee.environments.getIamPolicy` permission to call this API.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_trace_config")
                .about("Get distributed trace configuration in an environment.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the IAM policy on an environment, if the policy already exists it will be replaced. For more information, see [Manage users, roles, and permissions using the API](https://cloud.google.com/apigee/docs/api-platform/system-administration/manage-users-roles). You must have the `apigee.environments.setIamPolicy` permission to call this API.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("subscribe").about("Creates a subscription for the environment\'s Pub/Sub topic. The server will assign a random name for this subscription. The \"name\" and \"push_config\" must *not* be specified.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Tests the permissions of a user on an environment, and returns a subset of permissions that the user has on the environment. If the environment does not exist, an empty permission set is returned (a NOT_FOUND error is not returned).");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unsubscribe")
                .about("Deletes a subscription for the environment\'s Pub/Sub topic.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing environment. When updating properties, you must pass all existing properties to the API, even if they are not being changed. If you omit properties from the payload, the properties are removed. To get the current list of properties for the environment, use the [Get Environment API](get).");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_debugmask")
                .about("Updates the debug mask singleton resource for an environment.");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_environment").about("Updates an existing environment. When updating properties, you must pass all existing properties to the API, even if they are not being changed. If you omit properties from the payload, the properties are removed. To get the current list of properties for the environment, use the [Get Environment API](get).");
            environments1 = environments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_trace_config").about("Updates the trace configurations in an environment. Note that the repeated fields have replace semantics when included in the field mask and that they will be overwritten by the value of the fields in the request body.");
            environments1 = environments1.subcommand(mcmd);
        }
        let mut host_queries1 = SubCommand::with_name("host_queries")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, get_result, get_result_view and list");
        {
            let mcmd = SubCommand::with_name("create").about("Submit a query at host level to be processed in the background. If the submission of the query succeeds, the API returns a 201 status and an ID that refer to the query. In addition to the HTTP status 201, the `state` of \"enqueued\" means that the request succeeded.");
            host_queries1 = host_queries1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get status of a query submitted at host level. If the query is still in progress, the `state` is set to \"running\" After the query has completed successfully, `state` is set to \"completed\"");
            host_queries1 = host_queries1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_result").about("After the query is completed, use this API to retrieve the results. If the request succeeds, and there is a non-zero result set, the result is downloaded to the client as a zipped JSON file. The name of the downloaded file will be: OfflineQueryResult-.zip Example: `OfflineQueryResult-9cfc0d85-0f30-46d6-ae6f-318d0cb961bd.zip`");
            host_queries1 = host_queries1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_result_view").about("");
            host_queries1 = host_queries1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Return a list of Asynchronous Queries at host level.");
            host_queries1 = host_queries1.subcommand(mcmd);
        }
        let mut host_stats1 = SubCommand::with_name("host_stats")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve metrics grouped by dimensions in host level. The types of metrics you can retrieve include traffic, message counts, API call latency, response size, and cache hits and counts. Dimensions let you view metrics in meaningful groups. The stats api does accept dimensions as path params. The dimensions are optional in which case the metrics are computed on the entire data for the given timerange.");
            host_stats1 = host_stats1.subcommand(mcmd);
        }
        let mut instances1 = SubCommand::with_name("instances")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and report_status");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an Apigee runtime instance. The instance is accessible from the authorized network configured on the organization. **Note:** Not supported for Apigee hybrid.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an Apigee runtime instance. The instance stops serving requests and the runtime data is deleted. **Note:** Not supported for Apigee hybrid.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the details for an Apigee runtime instance. **Note:** Not supported for Apigee hybrid.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all Apigee runtime instances for the organization. **Note:** Not supported for Apigee hybrid.");
            instances1 = instances1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("report_status")
                .about("Reports the latest status for a runtime instance.");
            instances1 = instances1.subcommand(mcmd);
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
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut optimized_host_stats1 = SubCommand::with_name("optimized_host_stats")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about(
                "This api is similar to GetHostStats except that the response is less verbose.",
            );
            optimized_host_stats1 = optimized_host_stats1.subcommand(mcmd);
        }
        let mut reports1 = SubCommand::with_name("reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Custom Report for an Organization. A Custom Report provides Apigee Customers to create custom dashboards in addition to the standard dashboards which are provided. The Custom Report in its simplest form contains specifications about metrics, dimensions and filters. It is important to note that the custom report by itself does not provide an executable entity. The Edge UI converts the custom report definition into an analytics query and displays the result in a chart.");
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
            let mcmd = SubCommand::with_name("create").about("Uploads a ZIP-formatted shared flow configuration bundle to an organization. If the shared flow already exists, this creates a new revision of it. If the shared flow does not exist, this creates it. Once imported, the shared flow revision must be deployed before it can be accessed at runtime. The size limit of a shared flow bundle is 15 MB.");
            sharedflows1 = sharedflows1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a shared flow and all it\'s revisions. The shared flow must be undeployed before you can delete it.");
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
        let mut sites1 = SubCommand::with_name("sites")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: apicategories");
        let mut datastores2 = SubCommand::with_name("datastores")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, test and update");
        {
            let mcmd = SubCommand::with_name("create").about("Create a Datastore for an org");
            datastores2 = datastores2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a Datastore from an org.");
            datastores2 = datastores2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get a Datastore");
            datastores2 = datastores2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List Datastores");
            datastores2 = datastores2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test").about("Test if Datastore configuration is correct. This includes checking if credentials provided by customer have required permissions in target destination storage");
            datastores2 = datastores2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Update a Datastore");
            datastores2 = datastores2.subcommand(mcmd);
        }
        let mut attributes2 = SubCommand::with_name("attributes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list and update_api_product_attribute");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an API product attribute.");
            attributes2 = attributes2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets the value of an API product attribute.");
            attributes2 = attributes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all API product attributes.");
            attributes2 = attributes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_api_product_attribute").about("Updates the value of an API product attribute. **Note**: OAuth access tokens and Key Management Service (KMS) entities (apps, developers, and API products) are cached for 180 seconds (current default). Any custom attributes associated with entities also get cached for at least 180 seconds after entity is accessed during runtime. In this case, the `ExpiresIn` element on the OAuthV2 policy won\'t be able to expire an access token in less than 180 seconds.");
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
            let mcmd = SubCommand::with_name("delete").about("Deletes an API proxy revision and all policies, resources, endpoints, and revisions associated with it. The API proxy revision must be undeployed before you can delete it.");
            revisions2 = revisions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an API proxy revision. To download the API proxy configuration bundle for the specified revision as a zip file, set the `format` query parameter to `bundle`. If you are using curl, specify `-o filename.zip` to save the output to a file; otherwise, it displays to `stdout`. Then, develop the API proxy configuration locally and upload the updated API proxy configuration revision, as described in [updateApiProxyRevision](updateApiProxyRevision).");
            revisions2 = revisions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_api_proxy_revision").about("Updates an existing API proxy revision by uploading the API proxy configuration bundle as a zip file from your local machine. You can update only API proxy revisions that have never been deployed. After deployment, an API proxy revision becomes immutable, even if it is undeployed. Set the `Content-Type` header to either `multipart/form-data` or `application/octet-stream`.");
            revisions2 = revisions2.subcommand(mcmd);
        }
        let mut apps2 = SubCommand::with_name("apps")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: attributes_method, create, delete, generate_key_pair_or_update_developer_app_status, get, list and update");
        {
            let mcmd = SubCommand::with_name("attributes_method").about("Updates attributes for a developer app. This API replaces the current attributes with those specified in the request.");
            apps2 = apps2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates an app associated with a developer. This API associates the developer app with the specified API product and auto-generates an API key for the app to use in calls to API proxies inside that API product. The `name` is the unique ID of the app that you can use in API calls. The `DisplayName` (set as an attribute) appears in the UI. If you don\'t set the `DisplayName` attribute, the `name` appears in the UI.");
            apps2 = apps2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a developer app. **Note**: The delete operation is asynchronous. The developer app is deleted immediately, but its associated resources, such as app keys or access tokens, may take anywhere from a few seconds to a few minutes to be deleted.");
            apps2 = apps2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_key_pair_or_update_developer_app_status").about("Manages access to a developer app by enabling you to: * Approve or revoke a developer app * Generate a new consumer key and secret for a developer app To approve or revoke a developer app, set the `action` query parameter to `approved` or `revoked`, respectively, and the `Content-Type` header to `application/octet-stream`. If a developer app is revoked, none of its API keys are valid for API calls even though the keys are still `approved`. If successful, the API call returns the following HTTP status code: `204 No Content` To generate a new consumer key and secret for a developer app, pass the new key/secret details. Rather than replace an existing key, this API generates a new key. In this case, multiple key pairs may be associated with a single developer app. Each key pair has an independent status (`approved` or `revoked`) and expiration time. Any approved, non-expired key can be used in an API call. For example, if you\'re using API key rotation, you can generate new keys with expiration times that overlap keys that are going to expire. You might also generate a new consumer key/secret if the security of the original key/secret is compromised. The `keyExpiresIn` property defines the expiration time for the API key in milliseconds. If you don\'t set this property or set it to `-1`, the API key never expires. **Notes**: * When generating a new key/secret, this API replaces the existing attributes, notes, and callback URLs with those specified in the request. Include or exclude any existing information that you want to retain or delete, respectively. * To migrate existing consumer keys and secrets to hybrid from another system, see the CreateDeveloperAppKey API.");
            apps2 = apps2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Returns the details for a developer app.");
            apps2 = apps2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all apps created by a developer in an Apigee organization. Optionally, you can request an expanded view of the developer apps. A maximum of 100 developer apps are returned per API call. You can paginate the list of deveoper apps returned using the `startKey` and `count` query parameters.");
            apps2 = apps2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the details for a developer app. In addition, you can add an API product to a developer app and automatically generate an API key for the app to use when calling APIs in the API product. If you want to use an existing API key for the API product, add the API product to the API key using the UpdateDeveloperAppKey API. Using this API, you cannot update the following: * App name as it is the primary key used to identify the app and cannot be changed. * Scopes associated with the app. Instead, use the ReplaceDeveloperAppKey API. This API replaces the existing attributes with those specified in the request. Include or exclude any existing attributes that you want to retain or delete, respectively.");
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
            let mcmd = SubCommand::with_name("update_developer_attribute").about("Updates a developer attribute. **Note**: OAuth access tokens and Key Management Service (KMS) entities (apps, developers, and API products) are cached for 180 seconds (default). Any custom attributes associated with these entities are cached for at least 180 seconds after the entity is accessed at runtime. Therefore, an `ExpiresIn` element on the OAuthV2 policy won\'t be able to expire an access token in less than 180 seconds.");
            attributes2 = attributes2.subcommand(mcmd);
        }
        let mut attachments2 = SubCommand::with_name("attachments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new attachment of an environment to an environment group.");
            attachments2 = attachments2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes an environment group attachment.");
            attachments2 = attachments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an environment group attachment.");
            attachments2 = attachments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all attachments of an environment group.");
            attachments2 = attachments2.subcommand(mcmd);
        }
        let mut analytics2 = SubCommand::with_name("analytics")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: admin and exports");
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
            let mcmd = SubCommand::with_name("get").about("Returns the name of the shared flow attached to the specified flow hook. If there\'s no shared flow attached to the flow hook, the API does not return an error; it simply does not return a name in the response.");
            flowhooks2 = flowhooks2.subcommand(mcmd);
        }
        let mut keystores2 = SubCommand::with_name("keystores")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete and get");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a keystore or truststore. - Keystore: Contains certificates and their associated keys. - Truststore: Contains trusted certificates used to validate a server\'s certificate. These certificates are typically self-signed certificates or certificates that are not signed by a trusted CA.");
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
            let mcmd = SubCommand::with_name("get").about("This api is similar to GetStats except that the response is less verbose. In the current scheme, a query parameter _optimized instructs Edge Analytics to change the response but since this behavior is not possible with protocol buffer and since this parameter is predominantly used by Edge UI, we are introducing a separate api.");
            optimized_stats2 = optimized_stats2.subcommand(mcmd);
        }
        let mut queries2 = SubCommand::with_name("queries")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, get_result and list");
        {
            let mcmd = SubCommand::with_name("create").about("Submit a query to be processed in the background. If the submission of the query succeeds, the API returns a 201 status and an ID that refer to the query. In addition to the HTTP status 201, the `state` of \"enqueued\" means that the request succeeded.");
            queries2 = queries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get query status If the query is still in progress, the `state` is set to \"running\" After the query has completed successfully, `state` is set to \"completed\"");
            queries2 = queries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_result").about("After the query is completed, use this API to retrieve the results. If the request succeeds, and there is a non-zero result set, the result is downloaded to the client as a zipped JSON file. The name of the downloaded file will be: OfflineQueryResult-.zip Example: `OfflineQueryResult-9cfc0d85-0f30-46d6-ae6f-318d0cb961bd.zip`");
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
                "Deletes a Reference from an environment. Returns the deleted Reference resource.",
            );
            references2 = references2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Reference resource.");
            references2 = references2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing Reference. Note that this operation has PUT semantics; it will replace the entirety of the existing Reference with the resource in the request body.");
            references2 = references2.subcommand(mcmd);
        }
        let mut resourcefiles2 = SubCommand::with_name("resourcefiles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, list_environment_resources and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a resource file. Specify the `Content-Type` as `application/octet-stream` or `multipart/form-data`. For more information about resource files, see [Resource files](https://cloud.google.com/apigee/docs/api-platform/develop/resource-files).");
            resourcefiles2 = resourcefiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a resource file. For more information about resource files, see [Resource files](https://cloud.google.com/apigee/docs/api-platform/develop/resource-files).");
            resourcefiles2 = resourcefiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the contents of a resource file. For more information about resource files, see [Resource files](https://cloud.google.com/apigee/docs/api-platform/develop/resource-files).");
            resourcefiles2 = resourcefiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all resource files, optionally filtering by type. For more information about resource files, see [Resource files](https://cloud.google.com/apigee/docs/api-platform/develop/resource-files).");
            resourcefiles2 = resourcefiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_environment_resources").about("Lists all resource files, optionally filtering by type. For more information about resource files, see [Resource files](https://cloud.google.com/apigee/docs/api-platform/develop/resource-files).");
            resourcefiles2 = resourcefiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a resource file. Specify the `Content-Type` as `application/octet-stream` or `multipart/form-data`. For more information about resource files, see [Resource files](https://cloud.google.com/apigee/docs/api-platform/develop/resource-files).");
            resourcefiles2 = resourcefiles2.subcommand(mcmd);
        }
        let mut sharedflows2 = SubCommand::with_name("sharedflows")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: deployments and revisions");
        let mut stats2 = SubCommand::with_name("stats")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve metrics grouped by dimensions. The types of metrics you can retrieve include traffic, message counts, API call latency, response size, and cache hits and counts. Dimensions let you view metrics in meaningful groups. The stats api does accept dimensions as path params. The dimensions are optional in which case the metrics are computed on the entire data for the given timerange.");
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
            let mcmd = SubCommand::with_name("delete").about("Deletes a TargetServer from an environment. Returns the deleted TargetServer resource.");
            targetservers2 = targetservers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a TargetServer resource.");
            targetservers2 = targetservers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing TargetServer. Note that this operation has PUT semantics; it will replace the entirety of the existing TargetServer with the resource in the request body.");
            targetservers2 = targetservers2.subcommand(mcmd);
        }
        let mut trace_config2 = SubCommand::with_name("trace_config")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: overrides");
        let mut attachments2 = SubCommand::with_name("attachments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new attachment of an environment to an instance. **Note:** Not supported for Apigee hybrid.");
            attachments2 = attachments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes an attachment. **Note:** Not supported for Apigee hybrid.");
            attachments2 = attachments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets an attachment. **Note:** Not supported for Apigee hybrid.");
            attachments2 = attachments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists all attachments to an instance. **Note:** Not supported for Apigee hybrid.",
            );
            attachments2 = attachments2.subcommand(mcmd);
        }
        let mut canaryevaluations2 = SubCommand::with_name("canaryevaluations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and get");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new canary evaluation for an organization.");
            canaryevaluations2 = canaryevaluations2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets a CanaryEvaluation for an organization.");
            canaryevaluations2 = canaryevaluations2.subcommand(mcmd);
        }
        let mut nat_addresses2 = SubCommand::with_name("nat_addresses")
            .setting(AppSettings::ColoredHelp)
            .about("methods: activate, create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("activate").about("Activates the NAT address. The Apigee instance can now use this for Internet egress traffic. **Note:** Not supported for Apigee hybrid.");
            nat_addresses2 = nat_addresses2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a NAT address. The address is created in the RESERVED state and a static external IP address will be provisioned. At this time, the instance will not use this IP address for Internet egress traffic. The address can be activated for use once any required firewall IP whitelisting has been completed. **Note:** Not supported for Apigee hybrid.");
            nat_addresses2 = nat_addresses2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the NAT address. Connections that are actively using the address are drained before it is removed. **Note:** Not supported for Apigee hybrid.");
            nat_addresses2 = nat_addresses2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Gets the details of a NAT address. **Note:** Not supported for Apigee hybrid.",
            );
            nat_addresses2 = nat_addresses2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the NAT addresses for an Apigee instance. **Note:** Not supported for Apigee hybrid.");
            nat_addresses2 = nat_addresses2.subcommand(mcmd);
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
            let mcmd = SubCommand::with_name("delete").about("Deletes a shared flow and all associated policies, resources, and revisions. You must undeploy the shared flow before deleting it.");
            revisions2 = revisions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a revision of a shared flow. To download the shared flow configuration bundle for the specified revision as a zip file, set the `format` query parameter to `bundle`. If you are using curl, specify `-o filename.zip` to save the output to a file; otherwise, it displays to `stdout`. Then, develop the shared flow configuration locally and upload the updated sharedFlow configuration revision, as described in [updateSharedFlowRevision](updateSharedFlowRevision).");
            revisions2 = revisions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_shared_flow_revision").about("Updates a shared flow revision. This operation is only allowed on revisions which have never been deployed. After deployment a revision becomes immutable, even if it becomes undeployed. The payload is a ZIP-formatted shared flow. Content type must be either multipart/form-data or application/octet-stream.");
            revisions2 = revisions2.subcommand(mcmd);
        }
        let mut apicategories2 = SubCommand::with_name("apicategories")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a new category on the portal.");
            apicategories2 = apicategories2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a category from the portal.");
            apicategories2 = apicategories2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a category on the portal.");
            apicategories2 = apicategories2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the categories on the portal.");
            apicategories2 = apicategories2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a category on the portal.");
            apicategories2 = apicategories2.subcommand(mcmd);
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
            let mcmd = SubCommand::with_name("update_developer_app_attribute").about("Updates a developer app attribute. **Note**: OAuth access tokens and Key Management Service (KMS) entities (apps, developers, and API products) are cached for 180 seconds (current default). Any custom attributes associated with these entities are cached for at least 180 seconds after the entity is accessed at runtime. Therefore, an `ExpiresIn` element on the OAuthV2 policy won\'t be able to expire an access token in less than 180 seconds.");
            attributes3 = attributes3.subcommand(mcmd);
        }
        let mut keys3 = SubCommand::with_name("keys")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create_method, delete, get, replace_developer_app_key and update_developer_app_key");
        {
            let mcmd = SubCommand::with_name("create_method").about("Creates a custom consumer key and secret for a developer app. This is particularly useful if you want to migrate existing consumer keys and secrets to Apigee hybrid from another system. Consumer keys and secrets can contain letters, numbers, underscores, and hyphens. No other special characters are allowed. To avoid service disruptions, a consumer key and secret should not exceed 2 KBs each. **Note**: When creating the consumer key and secret, an association to API products will not be made. Therefore, you should not specify the associated API products in your request. Instead, use the UpdateDeveloperAppKey API to make the association after the consumer key and secret are created. If a consumer key and secret already exist, you can keep them or delete them using the DeleteDeveloperAppKey API.");
            keys3 = keys3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an app\'s consumer key and removes all API products associated with the app. After the consumer key is deleted, it cannot be used to access any APIs. **Note**: After you delete a consumer key, you may want to: 1. Create a new consumer key and secret for the developer app using the CreateDeveloperAppKey API, and subsequently add an API product to the key using the UpdateDeveloperAppKey API. 2. Delete the developer app, if it is no longer required.");
            keys3 = keys3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns details for a consumer key for a developer app, including the key and secret value, associated API products, and other information.");
            keys3 = keys3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_developer_app_key").about("Updates the scope of an app. This API replaces the existing scopes with those specified in the request. Include or exclude any existing scopes that you want to retain or delete, respectively. The specified scopes must already be defined for the API products associated with the app. This API sets the `scopes` element under the `apiProducts` element in the attributes of the app.");
            keys3 = keys3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_developer_app_key").about("Adds an API product to a developer app key, enabling the app that holds the key to access the API resources bundled in the API product. In addition, you can add attributes to a developer app key. This API replaces the existing attributes with those specified in the request. Include or exclude any existing attributes that you want to retain or delete, respectively. You can use the same key to access all API products associated with the app.");
            keys3 = keys3.subcommand(mcmd);
        }
        let mut admin3 = SubCommand::with_name("admin")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_schemav_2");
        {
            let mcmd = SubCommand::with_name("get_schemav_2").about("Get a list of metrics and dimensions which can be used for creating analytics queries and reports. Each schema element contains the name of the field with its associated type and if it is either custom field or standard field.");
            admin3 = admin3.subcommand(mcmd);
        }
        let mut exports3 = SubCommand::with_name("exports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Submit a data export job to be processed in the background. If the request is successful, the API returns a 201 status, a URI that can be used to retrieve the status of the export job, and the `state` value of \"enqueued\".");
            exports3 = exports3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the details and status of an analytics export job. If the export job is still in progress, its `state` is set to \"running\". After the export job has completed successfully, its `state` is set to \"completed\". If the export job fails, its `state` is set to `failed`.");
            exports3 = exports3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the details and status of all analytics export jobs belonging to the parent organization and environment.");
            exports3 = exports3.subcommand(mcmd);
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
            .about("methods: deploy, get_deployments and undeploy");
        {
            let mcmd = SubCommand::with_name("deploy").about("Deploys a revision of an API proxy. If another revision of the same API proxy revision is currently deployed, set the `override` parameter to `true` to have this revision replace the currently deployed revision. You cannot invoke an API proxy until it has been deployed to an environment. After you deploy an API proxy revision, you cannot edit it. To edit the API proxy, you must create and deploy a new revision. For a request path `organizations/{org}/environments/{env}/apis/{api}/revisions/{rev}/deployments`, two permissions are required: * `apigee.deployments.create` on the resource `organizations/{org}/environments/{env}` * `apigee.proxyrevisions.deploy` on the resource `organizations/{org}/apis/{api}/revisions/{rev}` ");
            revisions3 = revisions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_deployments").about("Gets the deployment of an API proxy revision and actual state reported by runtime pods.");
            revisions3 = revisions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undeploy").about("Undeploys an API proxy revision from an environment. For a request path `organizations/{org}/environments/{env}/apis/{api}/revisions/{rev}/deployments`, two permissions are required: * `apigee.deployments.delete` on the resource `organizations/{org}/environments/{env}` * `apigee.proxyrevisions.undeploy` on the resource `organizations/{org}/apis/{api}/revisions/{rev}`");
            revisions3 = revisions3.subcommand(mcmd);
        }
        let mut aliases3 = SubCommand::with_name("aliases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, csr, delete, get, get_certificate and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an alias from a key/certificate pair. The structure of the request is controlled by the `format` query parameter: - `keycertfile` - Separate PEM-encoded key and certificate files are uploaded. Set `Content-Type: multipart/form-data` and include the `keyFile`, `certFile`, and `password` (if keys are encrypted) fields in the request body. If uploading to a truststore, omit `keyFile`. - `pkcs12` - A PKCS12 file is uploaded. Set `Content-Type: multipart/form-data`, provide the file in the `file` field, and include the `password` field if the file is encrypted in the request body. - `selfsignedcert` - A new private key and certificate are generated. Set `Content-Type: application/json` and include CertificateGenerationSpec in the request body.");
            aliases3 = aliases3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("csr").about(
                "Generates a PKCS #10 Certificate Signing Request for the private key in an alias.",
            );
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
            .about("methods: deploy, get_deployments and undeploy");
        {
            let mcmd = SubCommand::with_name("deploy").about("Deploys a revision of a shared flow. If another revision of the same shared flow is currently deployed, set the `override` parameter to `true` to have this revision replace the currently deployed revision. You cannot use a shared flow until it has been deployed to an environment. For a request path `organizations/{org}/environments/{env}/sharedflows/{sf}/revisions/{rev}/deployments`, two permissions are required: * `apigee.deployments.create` on the resource `organizations/{org}/environments/{env}` * `apigee.sharedflowrevisions.deploy` on the resource `organizations/{org}/sharedflows/{sf}/revisions/{rev}`");
            revisions3 = revisions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_deployments").about("Gets the deployment of a shared flow revision and actual state reported by runtime pods.");
            revisions3 = revisions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undeploy").about("Undeploys a shared flow revision from an environment. For a request path `organizations/{org}/environments/{env}/sharedflows/{sf}/revisions/{rev}/deployments`, two permissions are required: * `apigee.deployments.delete` on the resource `organizations/{org}/environments/{env}` * `apigee.sharedflowrevisions.undeploy` on the resource `organizations/{org}/sharedflows/{sf}/revisions/{rev}`");
            revisions3 = revisions3.subcommand(mcmd);
        }
        let mut overrides3 = SubCommand::with_name("overrides")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a trace configuration override. The response contains a system-generated UUID, that can be used to view, update, or delete the configuration override. Use the List API to view the existing trace configuration overrides.");
            overrides3 = overrides3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a distributed trace configuration override.");
            overrides3 = overrides3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a trace configuration override.");
            overrides3 = overrides3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists all of the distributed trace configuration overrides in an environment.",
            );
            overrides3 = overrides3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a distributed trace configuration override. Note that the repeated fields have replace semantics when included in the field mask and that they will be overwritten by the value of the fields in the request body.");
            overrides3 = overrides3.subcommand(mcmd);
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
            let mcmd = SubCommand::with_name("delete").about("Removes an API product from an app\'s consumer key. After the API product is removed, the app cannot access the API resources defined in that API product. **Note**: The consumer key is not removed, only its association with the API product.");
            apiproducts4 = apiproducts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_developer_app_key_api_product").about("Approve or revoke an app\'s consumer key. After a consumer key is approved, the app can use it to access APIs. A consumer key that is revoked or pending cannot be used to access an API. Any access tokens associated with a revoked consumer key will remain active. However, Apigee hybrid checks the status of the consumer key and if set to `revoked` will not allow access to the API.");
            apiproducts4 = apiproducts4.subcommand(mcmd);
        }
        let mut create4 = SubCommand::with_name("create")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a custom consumer key and secret for a developer app. This is particularly useful if you want to migrate existing consumer keys and secrets to Apigee hybrid from another system. Consumer keys and secrets can contain letters, numbers, underscores, and hyphens. No other special characters are allowed. To avoid service disruptions, a consumer key and secret should not exceed 2 KBs each. **Note**: When creating the consumer key and secret, an association to API products will not be made. Therefore, you should not specify the associated API products in your request. Instead, use the UpdateDeveloperAppKey API to make the association after the consumer key and secret are created. If a consumer key and secret already exist, you can keep them or delete them using the DeleteDeveloperAppKey API.");
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
            let mcmd = SubCommand::with_name("delete_data").about("Deletes the data from a debug session. This does not cancel the debug session or prevent further data from being collected if the session is still active in runtime pods.");
            debugsessions4 = debugsessions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a debug session.");
            debugsessions4 = debugsessions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists debug sessions that are currently active in the given API Proxy revision.",
            );
            debugsessions4 = debugsessions4.subcommand(mcmd);
        }
        let mut deployments4 = SubCommand::with_name("deployments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generate_deploy_change_report and generate_undeploy_change_report");
        {
            let mcmd = SubCommand::with_name("generate_deploy_change_report").about("Generates a report for a dry run analysis of a DeployApiProxy request without committing the deployment. In addition to the standard validations performed when adding deployments, additional analysis will be done to detect possible traffic routing changes that would result from this deployment being created. Any potential routing conflicts or unsafe changes will be reported in the response. This routing analysis is not performed for a non-dry-run DeployApiProxy request. For a request path `organizations/{org}/environments/{env}/apis/{api}/revisions/{rev}/deployments:generateDeployChangeReport`, two permissions are required: * `apigee.deployments.create` on the resource `organizations/{org}/environments/{env}` * `apigee.proxyrevisions.deploy` on the resource `organizations/{org}/apis/{api}/revisions/{rev}`");
            deployments4 = deployments4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_undeploy_change_report").about("Generates a report for a dry run analysis of an UndeployApiProxy request without committing the undeploy. In addition to the standard validations performed when removing deployments, additional analysis will be done to detect possible traffic routing changes that would result from this deployment being removed. Any potential routing conflicts or unsafe changes will be reported in the response. This routing analysis is not performed for a non-dry-run UndeployApiProxy request. For a request path `organizations/{org}/environments/{env}/apis/{api}/revisions/{rev}/deployments:generateUndeployChangeReport`, two permissions are required: * `apigee.deployments.delete` on the resource `organizations/{org}/environments/{env}` * `apigee.proxyrevisions.undeploy` on the resource `organizations/{org}/apis/{api}/revisions/{rev}`");
            deployments4 = deployments4.subcommand(mcmd);
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
        revisions3 = revisions3.subcommand(deployments4);
        revisions3 = revisions3.subcommand(debugsessions4);
        keys3 = keys3.subcommand(create4);
        keys3 = keys3.subcommand(apiproducts4);
        revisions2 = revisions2.subcommand(deployments3);
        trace_config2 = trace_config2.subcommand(overrides3);
        sharedflows2 = sharedflows2.subcommand(revisions3);
        sharedflows2 = sharedflows2.subcommand(deployments3);
        keystores2 = keystores2.subcommand(aliases3);
        apis2 = apis2.subcommand(revisions3);
        apis2 = apis2.subcommand(deployments3);
        analytics2 = analytics2.subcommand(exports3);
        analytics2 = analytics2.subcommand(admin3);
        apps2 = apps2.subcommand(keys3);
        apps2 = apps2.subcommand(attributes3);
        revisions2 = revisions2.subcommand(deployments3);
        sites1 = sites1.subcommand(apicategories2);
        sharedflows1 = sharedflows1.subcommand(revisions2);
        sharedflows1 = sharedflows1.subcommand(deployments2);
        instances1 = instances1.subcommand(nat_addresses2);
        instances1 = instances1.subcommand(canaryevaluations2);
        instances1 = instances1.subcommand(attachments2);
        environments1 = environments1.subcommand(trace_config2);
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
        envgroups1 = envgroups1.subcommand(attachments2);
        developers1 = developers1.subcommand(attributes2);
        developers1 = developers1.subcommand(apps2);
        apis1 = apis1.subcommand(revisions2);
        apis1 = apis1.subcommand(keyvaluemaps2);
        apis1 = apis1.subcommand(deployments2);
        apiproducts1 = apiproducts1.subcommand(attributes2);
        analytics1 = analytics1.subcommand(datastores2);
        organizations0 = organizations0.subcommand(sites1);
        organizations0 = organizations0.subcommand(sharedflows1);
        organizations0 = organizations0.subcommand(reports1);
        organizations0 = organizations0.subcommand(optimized_host_stats1);
        organizations0 = organizations0.subcommand(operations1);
        organizations0 = organizations0.subcommand(keyvaluemaps1);
        organizations0 = organizations0.subcommand(instances1);
        organizations0 = organizations0.subcommand(host_stats1);
        organizations0 = organizations0.subcommand(host_queries1);
        organizations0 = organizations0.subcommand(environments1);
        organizations0 = organizations0.subcommand(envgroups1);
        organizations0 = organizations0.subcommand(developers1);
        organizations0 = organizations0.subcommand(deployments1);
        organizations0 = organizations0.subcommand(datacollectors1);
        organizations0 = organizations0.subcommand(apps1);
        organizations0 = organizations0.subcommand(apis1);
        organizations0 = organizations0.subcommand(apiproducts1);
        organizations0 = organizations0.subcommand(analytics1);
        hybrid0 = hybrid0.subcommand(issuers1);
        app = app.subcommand(projects0);
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

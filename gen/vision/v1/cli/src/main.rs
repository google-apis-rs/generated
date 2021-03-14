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
        let mut app = App::new("vision1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20210306")
            .about("Integrates Google Vision features, including image labeling, face, logo, and landmark detection, optical character recognition (OCR), and detection of explicit content, into applications.")
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
        let mut files0 = SubCommand::with_name("files")
            .setting(AppSettings::ColoredHelp)
            .about("methods: annotate and async_batch_annotate");
        {
            let mcmd = SubCommand::with_name("annotate").about("Service that performs image detection and annotation for a batch of files. Now only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported. This service will extract at most 5 (customers can specify which 5 in AnnotateFileRequest.pages) frames (gif) or pages (pdf or tiff) from each file provided and perform detection and annotation for each image extracted.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of generic files, such as PDF files, which may contain multiple pages and multiple images per page. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `OperationMetadata` (metadata). `Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results).");
            files0 = files0.subcommand(mcmd);
        }
        let mut images0 = SubCommand::with_name("images")
            .setting(AppSettings::ColoredHelp)
            .about("methods: annotate and async_batch_annotate");
        {
            let mcmd = SubCommand::with_name("annotate")
                .about("Run image detection and annotation for a batch of images.");
            images0 = images0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of images. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `OperationMetadata` (metadata). `Operation.response` contains `AsyncBatchAnnotateImagesResponse` (results). This service will write image annotation outputs to json files in customer GCS bucket, each json file containing BatchAnnotateImagesResponse proto.");
            images0 = images0.subcommand(mcmd);
        }
        let mut locations0 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: operations");
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn\'t support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn\'t support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: files, images, locations and operations");
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut files1 = SubCommand::with_name("files")
            .setting(AppSettings::ColoredHelp)
            .about("methods: annotate and async_batch_annotate");
        {
            let mcmd = SubCommand::with_name("annotate").about("Service that performs image detection and annotation for a batch of files. Now only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported. This service will extract at most 5 (customers can specify which 5 in AnnotateFileRequest.pages) frames (gif) or pages (pdf or tiff) from each file provided and perform detection and annotation for each image extracted.");
            files1 = files1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of generic files, such as PDF files, which may contain multiple pages and multiple images per page. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `OperationMetadata` (metadata). `Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results).");
            files1 = files1.subcommand(mcmd);
        }
        let mut images1 = SubCommand::with_name("images")
            .setting(AppSettings::ColoredHelp)
            .about("methods: annotate and async_batch_annotate");
        {
            let mcmd = SubCommand::with_name("annotate")
                .about("Run image detection and annotation for a batch of images.");
            images1 = images1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of images. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `OperationMetadata` (metadata). `Operation.response` contains `AsyncBatchAnnotateImagesResponse` (results). This service will write image annotation outputs to json files in customer GCS bucket, each json file containing BatchAnnotateImagesResponse proto.");
            images1 = images1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: files, images, operations, product_sets and products");
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut files2 = SubCommand::with_name("files")
            .setting(AppSettings::ColoredHelp)
            .about("methods: annotate and async_batch_annotate");
        {
            let mcmd = SubCommand::with_name("annotate").about("Service that performs image detection and annotation for a batch of files. Now only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported. This service will extract at most 5 (customers can specify which 5 in AnnotateFileRequest.pages) frames (gif) or pages (pdf or tiff) from each file provided and perform detection and annotation for each image extracted.");
            files2 = files2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of generic files, such as PDF files, which may contain multiple pages and multiple images per page. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `OperationMetadata` (metadata). `Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results).");
            files2 = files2.subcommand(mcmd);
        }
        let mut images2 = SubCommand::with_name("images")
            .setting(AppSettings::ColoredHelp)
            .about("methods: annotate and async_batch_annotate");
        {
            let mcmd = SubCommand::with_name("annotate")
                .about("Run image detection and annotation for a batch of images.");
            images2 = images2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of images. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `OperationMetadata` (metadata). `Operation.response` contains `AsyncBatchAnnotateImagesResponse` (results). This service will write image annotation outputs to json files in customer GCS bucket, each json file containing BatchAnnotateImagesResponse proto.");
            images2 = images2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut product_sets2 = SubCommand::with_name("product_sets")
            .setting(AppSettings::ColoredHelp)
            .about(
                "methods: add_product, create, delete, get, import, list, patch and remove_product",
            );
        {
            let mcmd = SubCommand::with_name("add_product").about("Adds a Product to the specified ProductSet. If the Product is already present, no change is made. One Product can be added to at most 100 ProductSets. Possible errors: * Returns NOT_FOUND if the Product or the ProductSet doesn\'t exist.");
            product_sets2 = product_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates and returns a new ProductSet resource. Possible errors: * Returns INVALID_ARGUMENT if display_name is missing, or is longer than 4096 characters.");
            product_sets2 = product_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes a ProductSet. Products and ReferenceImages in the ProductSet are not deleted. The actual image files are not deleted from Google Cloud Storage.");
            product_sets2 = product_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information associated with a ProductSet. Possible errors: * Returns NOT_FOUND if the ProductSet does not exist.");
            product_sets2 = product_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Asynchronous API that imports a list of reference images to specified product sets based on a list of image information. The google.longrunning.Operation API can be used to keep track of the progress and results of the request. `Operation.metadata` contains `BatchOperationMetadata`. (progress) `Operation.response` contains `ImportProductSetsResponse`. (results) The input source of this method is a csv file on Google Cloud Storage. For the format of the csv file please see ImportProductSetsGcsSource.csv_file_uri.");
            product_sets2 = product_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists ProductSets in an unspecified order. Possible errors: * Returns INVALID_ARGUMENT if page_size is greater than 100, or less than 1.");
            product_sets2 = product_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Makes changes to a ProductSet resource. Only display_name can be updated currently. Possible errors: * Returns NOT_FOUND if the ProductSet does not exist. * Returns INVALID_ARGUMENT if display_name is present in update_mask but missing from the request or longer than 4096 characters.");
            product_sets2 = product_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_product")
                .about("Removes a Product from the specified ProductSet.");
            product_sets2 = product_sets2.subcommand(mcmd);
        }
        let mut products2 = SubCommand::with_name("products")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and purge");
        {
            let mcmd = SubCommand::with_name("create").about("Creates and returns a new product resource. Possible errors: * Returns INVALID_ARGUMENT if display_name is missing or longer than 4096 characters. * Returns INVALID_ARGUMENT if description is longer than 4096 characters. * Returns INVALID_ARGUMENT if product_category is missing or invalid.");
            products2 = products2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes a product and its reference images. Metadata of the product and all its images will be deleted right away, but search queries against ProductSets containing the product may still work until all related caches are refreshed.");
            products2 = products2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information associated with a Product. Possible errors: * Returns NOT_FOUND if the Product does not exist.");
            products2 = products2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists products in an unspecified order. Possible errors: * Returns INVALID_ARGUMENT if page_size is greater than 100 or less than 1.");
            products2 = products2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Makes changes to a Product resource. Only the `display_name`, `description`, and `labels` fields can be updated right now. If labels are updated, the change will not be reflected in queries until the next index time. Possible errors: * Returns NOT_FOUND if the Product does not exist. * Returns INVALID_ARGUMENT if display_name is present in update_mask but is missing from the request or longer than 4096 characters. * Returns INVALID_ARGUMENT if description is present in update_mask but is longer than 4096 characters. * Returns INVALID_ARGUMENT if product_category is present in update_mask.");
            products2 = products2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("purge").about("Asynchronous API to delete all Products in a ProductSet or all Products that are in no ProductSet. If a Product is a member of the specified ProductSet in addition to other ProductSets, the Product will still be deleted. It is recommended to not delete the specified ProductSet until after this operation has completed. It is also recommended to not add any of the Products involved in the batch delete to a new ProductSet while this operation is running because those Products may still end up deleted. It\'s not possible to undo the PurgeProducts operation. Therefore, it is recommended to keep the csv files used in ImportProductSets (if that was how you originally built the Product Set) before starting PurgeProducts, in case you need to re-import the data after deletion. If the plan is to purge all of the Products from a ProductSet and then re-use the empty ProductSet to re-import new Products into the empty ProductSet, you must wait until the PurgeProducts operation has finished for that ProductSet. The google.longrunning.Operation API can be used to keep track of the progress and results of the request. `Operation.metadata` contains `BatchOperationMetadata`. (progress)");
            products2 = products2.subcommand(mcmd);
        }
        let mut products3 = SubCommand::with_name("products")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the Products in a ProductSet, in an unspecified order. If the ProductSet does not exist, the products field of the response will be empty. Possible errors: * Returns INVALID_ARGUMENT if page_size is greater than 100 or less than 1.");
            products3 = products3.subcommand(mcmd);
        }
        let mut reference_images3 = SubCommand::with_name("reference_images")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates and returns a new ReferenceImage resource. The `bounding_poly` field is optional. If `bounding_poly` is not specified, the system will try to detect regions of interest in the image that are compatible with the product_category on the parent product. If it is specified, detection is ALWAYS skipped. The system converts polygons into non-rotated rectangles. Note that the pipeline will resize the image if the image resolution is too large to process (above 50MP). Possible errors: * Returns INVALID_ARGUMENT if the image_uri is missing or longer than 4096 characters. * Returns INVALID_ARGUMENT if the product does not exist. * Returns INVALID_ARGUMENT if bounding_poly is not provided, and nothing compatible with the parent product\'s product_category is detected. * Returns INVALID_ARGUMENT if bounding_poly contains more than 10 polygons.");
            reference_images3 = reference_images3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes a reference image. The image metadata will be deleted right away, but search queries against ProductSets containing the image may still work until all related caches are refreshed. The actual image files are not deleted from Google Cloud Storage.");
            reference_images3 = reference_images3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information associated with a ReferenceImage. Possible errors: * Returns NOT_FOUND if the specified image does not exist.");
            reference_images3 = reference_images3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists reference images. Possible errors: * Returns NOT_FOUND if the parent product does not exist. * Returns INVALID_ARGUMENT if the page_size is greater than 100, or less than 1.");
            reference_images3 = reference_images3.subcommand(mcmd);
        }
        products2 = products2.subcommand(reference_images3);
        product_sets2 = product_sets2.subcommand(products3);
        locations1 = locations1.subcommand(products2);
        locations1 = locations1.subcommand(product_sets2);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(images2);
        locations1 = locations1.subcommand(files2);
        projects0 = projects0.subcommand(operations1);
        projects0 = projects0.subcommand(locations1);
        projects0 = projects0.subcommand(images1);
        projects0 = projects0.subcommand(files1);
        locations0 = locations0.subcommand(operations1);
        app = app.subcommand(projects0);
        app = app.subcommand(operations0);
        app = app.subcommand(locations0);
        app = app.subcommand(images0);
        app = app.subcommand(files0);

        Self { app }
    }
}
use google_vision1 as api;

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

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
            .version("0.1.0-20200417")
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
            let mcmd = SubCommand::with_name("annotate").about("Service that performs image detection and annotation for a batch of files.\nNow only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported.\n\nThis service will extract at most 5 (customers can specify which 5 in\nAnnotateFileRequest.pages) frames (gif) or pages (pdf or tiff) from each\nfile provided and perform detection and annotation for each image\nextracted.");
            files0 = files0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of generic\nfiles, such as PDF files, which may contain multiple pages and multiple\nimages per page. Progress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results).");
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
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of images.\n\nProgress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateImagesResponse` (results).\n\nThis service will write image annotation outputs to json files in customer\nGCS bucket, each json file containing BatchAnnotateImagesResponse proto.");
            images0 = images0.subcommand(mcmd);
        }
        let mut locations0 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: operations");
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is\nno longer interested in the operation result. It does not cancel the\noperation. If the server doesn\'t support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.");
            operations0 = operations0.subcommand(mcmd);
        }
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
            .about("sub-resources: files, images, locations and operations");
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut files1 = SubCommand::with_name("files")
            .setting(AppSettings::ColoredHelp)
            .about("methods: annotate and async_batch_annotate");
        {
            let mcmd = SubCommand::with_name("annotate").about("Service that performs image detection and annotation for a batch of files.\nNow only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported.\n\nThis service will extract at most 5 (customers can specify which 5 in\nAnnotateFileRequest.pages) frames (gif) or pages (pdf or tiff) from each\nfile provided and perform detection and annotation for each image\nextracted.");
            files1 = files1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of generic\nfiles, such as PDF files, which may contain multiple pages and multiple\nimages per page. Progress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results).");
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
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of images.\n\nProgress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateImagesResponse` (results).\n\nThis service will write image annotation outputs to json files in customer\nGCS bucket, each json file containing BatchAnnotateImagesResponse proto.");
            images1 = images1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: files, images, operations, product_sets and products");
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut files2 = SubCommand::with_name("files")
            .setting(AppSettings::ColoredHelp)
            .about("methods: annotate and async_batch_annotate");
        {
            let mcmd = SubCommand::with_name("annotate").about("Service that performs image detection and annotation for a batch of files.\nNow only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported.\n\nThis service will extract at most 5 (customers can specify which 5 in\nAnnotateFileRequest.pages) frames (gif) or pages (pdf or tiff) from each\nfile provided and perform detection and annotation for each image\nextracted.");
            files2 = files2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of generic\nfiles, such as PDF files, which may contain multiple pages and multiple\nimages per page. Progress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results).");
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
            let mcmd = SubCommand::with_name("async_batch_annotate").about("Run asynchronous image detection and annotation for a list of images.\n\nProgress and results can be retrieved through the\n`google.longrunning.Operations` interface.\n`Operation.metadata` contains `OperationMetadata` (metadata).\n`Operation.response` contains `AsyncBatchAnnotateImagesResponse` (results).\n\nThis service will write image annotation outputs to json files in customer\nGCS bucket, each json file containing BatchAnnotateImagesResponse proto.");
            images2 = images2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut product_sets2 = SubCommand::with_name("product_sets")
            .setting(AppSettings::ColoredHelp)
            .about(
                "methods: add_product, create, delete, get, import, list, patch and remove_product",
            );
        {
            let mcmd = SubCommand::with_name("add_product").about("Adds a Product to the specified ProductSet. If the Product is already\npresent, no change is made.\n\nOne Product can be added to at most 100 ProductSets.\n\nPossible errors:\n\n* Returns NOT_FOUND if the Product or the ProductSet doesn\'t exist.");
            product_sets2 = product_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates and returns a new ProductSet resource.\n\nPossible errors:\n\n* Returns INVALID_ARGUMENT if display_name is missing, or is longer than\n  4096 characters.");
            product_sets2 = product_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes a ProductSet. Products and ReferenceImages in the\nProductSet are not deleted.\n\nThe actual image files are not deleted from Google Cloud Storage.");
            product_sets2 = product_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information associated with a ProductSet.\n\nPossible errors:\n\n* Returns NOT_FOUND if the ProductSet does not exist.");
            product_sets2 = product_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Asynchronous API that imports a list of reference images to specified\nproduct sets based on a list of image information.\n\nThe google.longrunning.Operation API can be used to keep track of the\nprogress and results of the request.\n`Operation.metadata` contains `BatchOperationMetadata`. (progress)\n`Operation.response` contains `ImportProductSetsResponse`. (results)\n\nThe input source of this method is a csv file on Google Cloud Storage.\nFor the format of the csv file please see\nImportProductSetsGcsSource.csv_file_uri.");
            product_sets2 = product_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists ProductSets in an unspecified order.\n\nPossible errors:\n\n* Returns INVALID_ARGUMENT if page_size is greater than 100, or less\n  than 1.");
            product_sets2 = product_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Makes changes to a ProductSet resource.\nOnly display_name can be updated currently.\n\nPossible errors:\n\n* Returns NOT_FOUND if the ProductSet does not exist.\n* Returns INVALID_ARGUMENT if display_name is present in update_mask but\n  missing from the request or longer than 4096 characters.");
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
            let mcmd = SubCommand::with_name("create").about("Creates and returns a new product resource.\n\nPossible errors:\n\n* Returns INVALID_ARGUMENT if display_name is missing or longer than 4096\n  characters.\n* Returns INVALID_ARGUMENT if description is longer than 4096 characters.\n* Returns INVALID_ARGUMENT if product_category is missing or invalid.");
            products2 = products2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes a product and its reference images.\n\nMetadata of the product and all its images will be deleted right away, but\nsearch queries against ProductSets containing the product may still work\nuntil all related caches are refreshed.");
            products2 = products2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information associated with a Product.\n\nPossible errors:\n\n* Returns NOT_FOUND if the Product does not exist.");
            products2 = products2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists products in an unspecified order.\n\nPossible errors:\n\n* Returns INVALID_ARGUMENT if page_size is greater than 100 or less than 1.");
            products2 = products2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Makes changes to a Product resource.\nOnly the `display_name`, `description`, and `labels` fields can be updated\nright now.\n\nIf labels are updated, the change will not be reflected in queries until\nthe next index time.\n\nPossible errors:\n\n* Returns NOT_FOUND if the Product does not exist.\n* Returns INVALID_ARGUMENT if display_name is present in update_mask but is\n  missing from the request or longer than 4096 characters.\n* Returns INVALID_ARGUMENT if description is present in update_mask but is\n  longer than 4096 characters.\n* Returns INVALID_ARGUMENT if product_category is present in update_mask.");
            products2 = products2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("purge").about("Asynchronous API to delete all Products in a ProductSet or all Products\nthat are in no ProductSet.\n\nIf a Product is a member of the specified ProductSet in addition to other\nProductSets, the Product will still be deleted.\n\nIt is recommended to not delete the specified ProductSet until after this\noperation has completed. It is also recommended to not add any of the\nProducts involved in the batch delete to a new ProductSet while this\noperation is running because those Products may still end up deleted.\n\nIt\'s not possible to undo the PurgeProducts operation. Therefore, it is\nrecommended to keep the csv files used in ImportProductSets (if that was\nhow you originally built the Product Set) before starting PurgeProducts, in\ncase you need to re-import the data after deletion.\n\nIf the plan is to purge all of the Products from a ProductSet and then\nre-use the empty ProductSet to re-import new Products into the empty\nProductSet, you must wait until the PurgeProducts operation has finished\nfor that ProductSet.\n\nThe google.longrunning.Operation API can be used to keep track of the\nprogress and results of the request.\n`Operation.metadata` contains `BatchOperationMetadata`. (progress)");
            products2 = products2.subcommand(mcmd);
        }
        let mut products3 = SubCommand::with_name("products")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the Products in a ProductSet, in an unspecified order. If the\nProductSet does not exist, the products field of the response will be\nempty.\n\nPossible errors:\n\n* Returns INVALID_ARGUMENT if page_size is greater than 100 or less than 1.");
            products3 = products3.subcommand(mcmd);
        }
        let mut reference_images3 = SubCommand::with_name("reference_images")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates and returns a new ReferenceImage resource.\n\nThe `bounding_poly` field is optional. If `bounding_poly` is not specified,\nthe system will try to detect regions of interest in the image that are\ncompatible with the product_category on the parent product. If it is\nspecified, detection is ALWAYS skipped. The system converts polygons into\nnon-rotated rectangles.\n\nNote that the pipeline will resize the image if the image resolution is too\nlarge to process (above 50MP).\n\nPossible errors:\n\n* Returns INVALID_ARGUMENT if the image_uri is missing or longer than 4096\n  characters.\n* Returns INVALID_ARGUMENT if the product does not exist.\n* Returns INVALID_ARGUMENT if bounding_poly is not provided, and nothing\n  compatible with the parent product\'s product_category is detected.\n* Returns INVALID_ARGUMENT if bounding_poly contains more than 10 polygons.");
            reference_images3 = reference_images3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes a reference image.\n\nThe image metadata will be deleted right away, but search queries\nagainst ProductSets containing the image may still work until all related\ncaches are refreshed.\n\nThe actual image files are not deleted from Google Cloud Storage.");
            reference_images3 = reference_images3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information associated with a ReferenceImage.\n\nPossible errors:\n\n* Returns NOT_FOUND if the specified image does not exist.");
            reference_images3 = reference_images3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists reference images.\n\nPossible errors:\n\n* Returns NOT_FOUND if the parent product does not exist.\n* Returns INVALID_ARGUMENT if the page_size is greater than 100, or less\n  than 1.");
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

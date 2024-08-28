use polars::io::cloud::CloudWriter;
use polars::prelude::*;
use cloud::AmazonS3ConfigKey as Key;

const TEST_S3_LOCATION: &str = "s3://polarstesting/polars_write_example_cloud.parquet";

#[tokio::main]
async fn main() {
    let cloud_options = cloud::CloudOptions::default().with_aws([
        (Key::AccessKeyId, "admin".to_string()),
        (Key::SecretAccessKey, "admin123".to_string()),
        (Key::Endpoint, "http://localhost:9000".to_string()),
        (Key::Region, "us-east-1".to_string()),
    ]);

    let writer = CloudWriter::new(TEST_S3_LOCATION, Some(&cloud_options)).await.unwrap();
    let writer = ParquetWriter::new(writer);

    let mut df = df![
        "ints" => [1, 2, 3]
    ].unwrap();

    tokio::task::spawn_blocking(move || {
        eprintln!("About to call writer.finish");
        writer.finish(&mut df).unwrap();
    }).await.unwrap();

    eprintln!("End of program"); // Never gets reached
}

#[tokio::main]
async fn main_lazyframe() {
    let cloud_options = cloud::CloudOptions::default().with_aws([
        (Key::AccessKeyId, "admin".to_string()),
        (Key::SecretAccessKey, "admin123".to_string()),
        (Key::Endpoint, "http://localhost:9000".to_string()),
        (Key::Region, "us-east-1".to_string()),
    ]);

    let df = LazyFrame::scan_parquet("foods1.parquet", ScanArgsParquet::default())
        .unwrap();

    df.sink_parquet_cloud(
        TEST_S3_LOCATION.to_string(),
        Some(cloud_options),
        Default::default(),
    )
    .unwrap();
}

use polars::prelude::*;

fn main() -> PolarsResult<()> {
    let df = LazyFrame::scan_parquet("foods1.parquet", ScanArgsParquet::default())?
        .select([
            // select all columns
            all(),
        ])
        .collect()?;

    println!("df: {df}");

    Ok(())
}

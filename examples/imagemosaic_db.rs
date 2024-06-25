use std::sync::Arc;

use arrow_array::types::Float32Type;
use arrow_array::{
    FixedSizeListArray, GenericStringArray, Int32Array, RecordBatch, RecordBatchIterator,
};
use futures::TryStreamExt;

use lancedb::arrow::arrow_schema::{DataType, Field, Schema};
use lancedb::index::Index;
use lancedb::query::{ExecutableQuery, QueryBase};
use lancedb::{connect, Table as LanceDbTable};
use tokio::fs::ReadDir;

const DIM: usize = 3;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //TODO: remove this once db is seeded
    if std::path::Path::new("data").exists() {
        std::fs::remove_dir_all("data").unwrap();
    }

    let uri = "data/sample-lancedb";
    let db = connect(uri).execute().await?;

    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new(
            "vector",
            DataType::FixedSizeList(
                Arc::new(Field::new("item", DataType::Float32, true)),
                DIM as i32,
            ),
            false,
        ),
        Field::new("path", DataType::Utf8, false),
    ]));

    // read images from datasets folder using tokio
    let mut total = 0;
    let mut entries: ReadDir = tokio::fs::read_dir("datasets/cifar10/train/horse").await?;
    let mut colors = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let image = image::open(&path).unwrap();
        // whyyyy
        // let image = image.resize(100, 100, image::imageops::FilterType::CatmullRom);
        let image = image.to_rgb8();
        let image = image.into_raw();

        let average_color = image
            .chunks_exact(3)
            .fold([0, 0, 0], |acc, pixel| {
                [
                    acc[0] + pixel[0] as i32,
                    acc[1] + pixel[1] as i32,
                    acc[2] + pixel[2] as i32,
                ]
            })
            .iter()
            // 32 because dataset image size
            .map(|x| *x as f32 / (32 * 32) as f32)
            .collect::<Vec<_>>();

        let r = average_color[0] as f32 / 255.;
        let g = average_color[1] as f32 / 255.;
        let b = average_color[2] as f32 / 255.;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);

        let mut h = if max == min {
            0.
        } else if max == r {
            60. * (0. + (g - b) / (max - min))
        } else if max == g {
            60. * (2. + (b - r) / (max - min))
        } else if max == b {
            60. * (4. + (r - g) / (max - min))
        } else {
            unreachable!()
        };

        if h < 0. {
            h += 360.
        }
        let s = if max == min { 0. } else { (max - min) / (max) };
        let v = max;
        let average_color = [h, s, v];

        println!("Reading image: {total}; {:?}", average_color);

        colors.push((path.to_str().unwrap().to_string(), average_color));
        total += 1;
    }

    let batches = RecordBatchIterator::new(
        vec![RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(Int32Array::from_iter_values(0..total as i32)),
                Arc::new(
                    FixedSizeListArray::from_iter_primitive::<Float32Type, _, _>(
                        (0..total).map(|i| {
                            Some(
                                colors[i]
                                    .1
                                    .clone()
                                    .iter()
                                    .map(|x| Some(*x))
                                    .collect::<Vec<_>>(),
                            )
                        }),
                        DIM as i32,
                    ),
                ),
                Arc::new(GenericStringArray::<i32>::from_iter_values(
                    (0..total).map(|i| colors[i].0.clone()),
                )),
            ],
        )
        .unwrap()]
        .into_iter()
        .map(Ok),
        schema.clone(),
    );

    let table = db
        .create_table("images", Box::new(batches))
        .execute()
        .await
        .unwrap();

    table
        .create_index(&["vector"], Index::Auto)
        .execute()
        .await?;

    Ok(())
}

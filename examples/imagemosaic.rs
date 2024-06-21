// Copyright 2024 Lance Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This example demonstrates basic usage of LanceDb.
//!
//! Snippets from this example are used in the quickstart documentation.

use std::sync::Arc;

use arrow_array::types::{Float32Type, Utf8Type};
use arrow_array::{FixedSizeListArray, GenericStringArray, Int32Array, RecordBatch, RecordBatchIterator};
use futures::TryStreamExt;

use lancedb::arrow::arrow_schema::{DataType, Field, Schema};
use lancedb::arrow::IntoArrow;
use lancedb::connection::Connection;
use lancedb::index::Index;
use lancedb::query::{ExecutableQuery, QueryBase};
use lancedb::{connect, Result, Table as LanceDbTable};

const TOTAL: usize = 1000;
const DIM: usize = 3;

#[tokio::main]
async fn main() -> Result<()> {
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

    let batches = RecordBatchIterator::new(
        vec![RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(Int32Array::from_iter_values(0..TOTAL as i32)),
                Arc::new(
                    FixedSizeListArray::from_iter_primitive::<Float32Type, _, _>(
                        (0..TOTAL).map(|_| Some(vec![Some(1.0); DIM])),
                        DIM as i32,
                    ),
                ),
                Arc::new(
                    FixedSizeListArray::from_iter_primitive::<Utf8Type, _, _>(
                        (0..TOTAL).map(|i| Some(vec![Some(format!("image-{}.png", i)); 1])),
                        1,
                    ),
                ),
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

    table.create_index(&["vector_index"], Index::Auto).execute().await?;

    let paths = table.query().limit(1).nearest_to(&[1.0; DIM])?.execute().await?;
    let paths = paths.try_collect::<Vec<_>>().await?;
    dbg!(paths);

    Ok(())
}

async fn search(table: &LanceDbTable) -> Result<Vec<RecordBatch>> {
    // --8<-- [start:search]
    table
        .query()
        .limit(2)
        .nearest_to(&[1.0; DIM])?
        .execute()
        .await?
        .try_collect::<Vec<_>>()
        .await
    // --8<-- [end:search]
}

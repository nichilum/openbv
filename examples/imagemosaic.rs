use arrow_array::GenericStringArray;
use futures::TryStreamExt;

use image::GenericImage;
use lancedb::connect;
use lancedb::query::{ExecutableQuery, QueryBase};
use openbv::open_rgb;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let uri = "data/sample-lancedb";
    let db = connect(uri).execute().await?;

    let table = db.open_table("images").execute().await.unwrap();

    let pixel_height = 10;
    let mut image = open_rgb("images/bridge.jpg").unwrap();
    for x in 0..image.width() / pixel_height {
        println!("{:.2}%", x as f32 * 100. / (image.width() / pixel_height) as f32);
        for y in 0..image.height() / pixel_height {
            let sub_image = image
                .sub_image(
                    x * pixel_height,
                    y * pixel_height,
                    pixel_height,
                    pixel_height,
                )
                .to_image()
                .into_raw();

            let average_color = sub_image
                .chunks_exact(3)
                .fold([0, 0, 0], |acc, pixel| {
                    [
                        acc[0] + pixel[0] as i32,
                        acc[1] + pixel[1] as i32,
                        acc[2] + pixel[2] as i32,
                    ]
                })
                .iter()
                .map(|x| *x as f32 / (pixel_height * pixel_height) as f32)
                .collect::<Vec<_>>();

            let paths = table
                .query()
                .limit(1)
                .nearest_to(&[average_color[0], average_color[1], average_color[2]])?
                .distance_type(lancedb::DistanceType::L2)
                .execute()
                .await?;

            // color values are all the same
            let paths = paths.try_collect::<Vec<_>>().await?;
            let path: &arrow_array::GenericByteArray<arrow_array::types::GenericStringType<i32>> = paths[0]
                .column(2)
                .as_any()
                .downcast_ref::<GenericStringArray<i32>>()
                .unwrap();
            let path = path.value(0).to_string();
            let copy_image = image::open(path).unwrap();
            let copy_image = copy_image.resize(
                pixel_height,
                pixel_height,
                image::imageops::FilterType::CatmullRom,
            );
            let copy_image = copy_image.to_rgb8();
            // this errors on the boundaries
            let _ = image.copy_from(&copy_image, x * pixel_height, y * pixel_height);
        }
    }

    image.save("export/bridge.png").unwrap();

    Ok(())
}

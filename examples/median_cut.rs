use openbv::{nonlinearops::median_cut::MedianCutExt, open_gray};

fn main() {
    let mut image = open_gray("./images/Set03_single.jpg").unwrap();
    image.median_cut(4);
    image.save("export/median_cut.png").unwrap();
}

use charming::{
    component::Legend,
    element::ItemStyle,
    series::{Pie, PieRoseType},
    Chart, ImageRenderer
};
use jni::JNIEnv;
use jni::objects::*;


#[no_mangle]
pub unsafe extern "C" fn Java_top_magicpotato_RustJNI_init(env: JNIEnv, _class: JClass) {
    println!("rust-java-demo inited");
    let chart = Chart::new()
        .legend(Legend::new().top("bottom"))
        .series(
            Pie::new()
                .name("Nightingale Chart")
                .rose_type(PieRoseType::Radius)
                .radius(vec!["50", "250"])
                .center(vec!["50%", "50%"])
                .item_style(ItemStyle::new().border_radius(8))
                .data(vec![
                    (40.0, "rose 1"),
                    (38.0, "rose 2"),
                    (32.0, "rose 3"),
                    (30.0, "rose 4"),
                    (28.0, "rose 5"),
                    (26.0, "rose 6"),
                    (22.0, "rose 7"),
                    (18.0, "rose 8"),
                ]),
        );

    let mut renderer = ImageRenderer::new(1000, 800);
    renderer.save(&chart, "nightingale.svg").unwrap();

}
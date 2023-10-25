use charming::{Chart, component::Legend, element::ItemStyle, ImageFormat, ImageRenderer, series::{Pie, PieRoseType}};
use jni::JNIEnv;
use jni::objects::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Module {
    name: String,
    path:String,
    data: Vec<(f64, String)>,
}

#[derive(Deserialize)]
struct PieWrapper{
    data:Pie
}

#[no_mangle]
pub unsafe extern "C" fn Java_top_magicpotato_RustJNI_init(mut env: JNIEnv, _class: JClass, data: JString) {
    let data: String = env.get_string(&data).expect("Couldn't get java string!").into();
    let module: Module = serde_json::from_str(&data).expect("数据类型无法反序列化");
    // println!("rust-java-demo inited {}", data);

    let chart = Chart::new()
        .legend(Legend::new().top("bottom"))
        .series(
            Pie::new()
                .name(module.name)
                .rose_type(PieRoseType::Radius)
                .radius(vec!["50", "250"])
                .center(vec!["50%", "50%"])
                .item_style(ItemStyle::new().border_radius(8))
                .data(module.data),
        );

    let mut renderer = ImageRenderer::new(1000, 800);
    renderer.save_format(parse_extension_by_path(&module.path), &chart, &module.path).unwrap();
}

/// 通过文件名字解析扩展名
fn parse_extension_by_path(path: &str) -> ImageFormat {
    let index = path.rfind('.').expect("路径参数错误，无法识别扩展名");
    let extension = &path[index + 1..];
    ImageFormat::from_extension(extension).expect(&format!("不支持的扩展名:{}", extension))
}
use std::path::Path;

use charming::{ImageFormat, ImageRenderer};
use jni::JNIEnv;
use jni::objects::*;
use jni::sys::jint;

/// 跟进路径和原始json渲染图表
#[no_mangle]
pub unsafe extern "C" fn Java_top_magicpotato_Echarts_save(mut env: JNIEnv, _class: JClass, width: jint, height: jint, path: JString, data: JString) {
    let data: String = env.get_string(&data).expect("Couldn't get data string!").into();
    let path: String = env.get_string(&path).expect("Couldn't get path string!").into();
    let mut renderer = ImageRenderer::new(width as u32, height as u32);

    let path = Path::new(&path);
    if path.extension().expect("无法获取文件扩展名") == "svg" {
        renderer.save_by_json(data, path).unwrap();
    } else {
        renderer.save_format_by_json(parse_extension_by_path(path), data, path).unwrap();
    }
}

/// 跟进json渲染数据  返回byte数组
#[no_mangle]
pub unsafe extern "C" fn Java_top_magicpotato_Echarts_render<'local>(mut env: JNIEnv<'local>, _class: JClass, width: jint, height: jint, extension: JString<'local>, data: JString<'local>) -> JByteArray<'local> {
    let data: String = env.get_string(&data).expect("Couldn't get data string!").into();
    let extension: String = env.get_string(&extension).expect("Couldn't get path string!").into();
    let mut renderer = ImageRenderer::new(width as u32, height as u32);

    if extension == "svg" {
        let x = renderer.render_by_json(data).unwrap();
        env.byte_array_from_slice(x.as_bytes()).unwrap()
    } else {
        let x = renderer.render_format_by_json(parse_extension(&extension), data).unwrap();
        env.byte_array_from_slice(x.as_slice()).unwrap()
    }
}

/// 通过文件名字解析扩展名
fn parse_extension_by_path(path: &Path) -> ImageFormat {
    ImageFormat::from_path(path).expect(&format!("不支持的扩展名"))
}

fn parse_extension(ext: &str) -> ImageFormat {
    ImageFormat::from_extension(ext).expect(&format!("不支持的扩展名"))
}
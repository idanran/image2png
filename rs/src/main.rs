use image::GenericImageView;

// 定义一个函数，将jpeg或webp转换为png
fn convert_image_to_png(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(input).unwrap();
    println!("dimensions {:?}", img.dimensions());
    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());
    // Write the contents of this image to the Writer in PNG format.
    img.save(output).unwrap();
    // 返回成功结果
    Ok(())
}

// 主函数
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 获取命令行参数
    let args: Vec<String> = std::env::args().collect();
    // 检查是否有足够的参数
    if args.len() < 3 {
        // 如果没有，打印用法信息并退出
        println!("Usage: image-cli <input_file> <output_file>");
        std::process::exit(1);
    }
    // 获取第一个参数作为webp或jpeg文件的路径
    let input = &args[1];
    // 获取第二个参数作为png文件的路径
    let output = &args[2];
    // 调用转换函数，处理可能的错误
    convert_image_to_png(input, output)?;
    // 返回成功结果
    Ok(())
}

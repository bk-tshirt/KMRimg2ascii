///这是一个我做的测试项目，顺便拿去参加学校的电脑制作大赛
/// 
/// 项目名称：图像转ascii字符画
/// 作者：bk_tshirt
/// 日期：2024年11月24日
/// 版本：1.0
///由于时间仓促，有一些功能还未实现
/// 吐槽看README.md


extern crate opencv;

use opencv::{
    core,
    imgcodecs,
    imgproc
};
use opencv::prelude::*;
use std::{
    env,
    fs::File,
    io::Write
};


//调用的外部库

fn main() {
    let str1:&str = "欢迎使用KMRimg2ascii! ";
    //使用命令行参数
    println!("{}",str1);
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: {} <image_path> <width> <height>", args[0]);
        return ;

    }


   //读取图片
   let image = imgcodecs::imread(args[1].as_str(), imgcodecs::IMREAD_COLOR).unwrap();


   //image灰度化
   let mut gray_image = Mat::default();
   imgproc::cvt_color(&image, &mut gray_image, imgproc::COLOR_BGR2GRAY, 0).unwrap();
   let desired_width = args[2].parse::<i32>().unwrap();
   let desired_height = args[3].parse::<i32>().unwrap(); //设置图片大小

   // 在保持长宽比的同时计算比例因子
   let scale_w = desired_width as f64 / image.cols() as f64;
   let scale_h = desired_height as f64 / image.rows() as f64;
   let scale = scale_w.min(scale_h);
   let new_width = (image.cols() as f64 * scale) as i32;

   let new_height = (image.rows() as f64 * scale) as i32;
   let mut resized = Mat::default(); //调整图片大小

   imgproc::resize(&gray_image,
    &mut resized,
    core::Size::new(new_width, new_height),
    0.0,0.0,
    imgproc::INTER_LINEAR,).unwrap();

    //将灰度图像转换为ASCII字符,以下是准备好的ascii字符串

    let ascii_chars = "<$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. >";

    let mut ascii_art = String::new();

    for i in 0..resized.rows() {

        for j in 0..resized.cols() {

            let pixel_value = resized.at_2d::<u8>(i, j).unwrap();

            let ascii_index = *pixel_value as usize * ascii_chars.len()/256;
            
            ascii_art.push(ascii_chars.chars().nth(ascii_index).unwrap_or(' '));
    }
    ascii_art.push('\n');
}

    //save
    let mut file = File::create(format!("{}.txt",args[1].split('.').collect::<Vec<&str>>()[0])).unwrap();

    file.write_all(ascii_art.as_bytes()).unwrap();

    println!("{}",ascii_art);

    println!("finished!");

}

fn add(a,b)-> i32 {

    a + b

}


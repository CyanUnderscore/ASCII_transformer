use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1]; // "/home/cyansky/Images/fille contente.jpg" ; // &args[1];
    let def = *&args[2].parse::<usize>().unwrap();//1000;
    // create verification
    let mut img: image::DynamicImage = image::open(Path::new(path)).unwrap();
    let height = img.height();
    let width = img.width();
    let img_bytes: &[u8] =img.as_bytes();
    println!("{} {} {}", height, width, img_bytes.len());
    let mut pixels_val: Vec<u8> = vec![];
    let mut i: usize = 0;
    let lenght: usize = img_bytes.len();
    while i < lenght {
        println!("{} {}", lenght/3, i);
        pixels_val.push((((img_bytes[i]as u16) + (img_bytes[i+1]as u16) + (img_bytes[i+2] as u16))/3 )as u8);
        i+=3;
    }
    let mut image_map: Vec<Vec<u8>> =vec![];
    let mut n =0;
    for i in 0..height{
        image_map.push(vec![]);
        for j in 0..width{
            image_map[i as usize].push(pixels_val[n]); 
            n +=1;
        }
    }
    let a = 0;
    println!("ligne {}", image_map.len());
    for i in 0..image_map.len(){
        println!("{}",image_map[i].len());

    }
    let compressed_image: Vec<Vec<u8>> =  compress(image_map, def);
    let mut transformed_image: Vec<Vec<char>> = vec![];
    for i in 0..compressed_image.len(){
        transformed_image.push(vec![]);
        for j in 0..compressed_image[i].len(){
            transformed_image[i].push(ASCII(compressed_image[i][j]))
        }
    }
    for i in transformed_image{
        for j in i{
            print!("{j}{j}");
        }
        println!("");
    }
}

fn compress(map : Vec<Vec<u8>>, def:usize) -> Vec<Vec<u8>>{
    print!("|");
    let map_row = map.len();
    let map_col = map[0].len();
    if map_row<=def && map[0].len()<=def{
        return map;
    }
    print!("|");
    let mut new_map: Vec<Vec<u8>> = vec![];
    let mut col = 0;
    let mut row = 0;
    let mut nm_row = 0;
    while row < map_row-3{
        print!("&");
        new_map.push(vec![]);
        while col <map_col-3{
            print!("col: {}, len col: {}| ", col, map_col);
            let sum: u16 =
            (map[row][col] as u16) + (map[row][col + 1] as u16) + (map[row][col + 2] as u16) +
            (map[row + 1][col] as u16) + (map[row + 1][col + 1] as u16) + (map[row + 1][col + 2] as u16) +
            (map[row + 2][col] as u16) + (map[row + 2][col + 1] as u16) + (map[row + 2][col + 2] as u16);
            println!("{}", new_map.len());
            new_map[nm_row].push((sum/9) as u8);
            col+=3
        }
        row +=3;
        nm_row +=1;
        col = 0;
        println!("row: {}, len row {} |", row, map_row);
    }
    println!("");
    return compress(new_map, def);
}

fn ASCII(num : u8) -> char{
    match num {
        0..=25 => return ' ',
        26..=50 => return '.',
        51..=75 => return ':',
        76..=100 => return '-',
        101..=125 => return '=',
        126..=150 => return '+',
        151..=175 => return '*',
        176..=200 => return '#',
        201..=225 => return '&',
        226..=255 => return '@',
    }
}
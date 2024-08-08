extern crate neon;
extern crate stl_io;
extern crate image;
extern crate rayon;

use neon::prelude::*;
use stl_io::{Vertex, IndexedMesh};
use image::{ImageBuffer, Rgb};
use rayon::prelude::*;


fn project_vertex(vertex: &Vertex, bounds: (f32, f32, f32, f32)) -> (i32, i32) {
    let (min_x, max_x, min_y, max_y) = bounds;
    let scale_x = 180.0 / (max_x - min_x);
    let scale_y = 180.0 / (max_y - min_y);
    let x = ((vertex[0] - min_x) * scale_x + 10.0) as i32;
    let y = ((vertex[1] - min_y) * scale_y + 10.0) as i32;
    (x, y)
}

fn draw_line(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, start: (i32, i32), end: (i32, i32), color: Rgb<u8>) {
    let (mut x, mut y) = start;
    let dx = (end.0 - start.0).abs();
    let dy = -((end.1 - start.1).abs());
    let sx = if start.0 < end.0 { 1 } else { -1 };
    let sy = if start.1 < end.1 { 1 } else { -1 };
    let mut err = dx + dy;

    while x != end.0 || y != end.1 {
        if x >= 0 && x < img.width() as i32 && y >= 0 && y < img.height() as i32 {
            img.put_pixel(x as u32, y as u32, color);
        }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x += sx; }
        if e2 <= dx { err += dx; y += sy; }
    }
}

fn project_and_render(mesh: IndexedMesh) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = 200;
    let height = 200;
    let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);

    let (mut min_x, mut max_x, mut min_y, mut max_y) = (f32::INFINITY, f32::NEG_INFINITY, f32::INFINITY, f32::NEG_INFINITY);
    for vertex in &mesh.vertices {
        min_x = min_x.min(vertex[0]);
        max_x = max_x.max(vertex[0]);
        min_y = min_y.min(vertex[1]);
        max_y = max_y.max(vertex[1]);
    }
    let bounds = (min_x, max_x, min_y, max_y);

    let lines: Vec<_> = mesh.faces.iter().flat_map(|triangle| {
        let vertices = triangle.vertices.map(|i| mesh.vertices[i]);
        let points = vertices.map(|vertex| project_vertex(&vertex, bounds));
        (0..3).map(move |i| (points[i], points[(i + 1) % 3])).collect::<Vec<_>>()
    }).collect();

    for (start, end) in lines {
        draw_line(&mut img, start, end, Rgb([255, 255, 255]));
    }

    img
}
fn generate_thumbnail(mut cx: FunctionContext) -> JsResult<JsString> {
    let file_path = cx.argument::<JsString>(0)?.value();
    let img_path = cx.argument::<JsString>(1)?.value();

    let mut stl_file = std::fs::File::open(&file_path)
        .or_else(|err| cx.throw_error(format!("Failed to open STL file: {}", err)))?;
    let mesh = stl_io::read_stl(&mut stl_file)
        .or_else(|err| cx.throw_error(format!("Failed to read STL file: {}", err)))?;

    let img = project_and_render(mesh);
    img.save(img_path.clone())
        .or_else(|err| cx.throw_error(format!("Failed to save image: {}", err)))?;

    Ok(cx.string(img_path))
}
register_module!(mut cx, {
    cx.export_function("generateThumbnail", generate_thumbnail)?;
    Ok(())
});



// Prolly packed will be better solution
//#[repr(C)]
//#[repr(packed)]
struct TGAHdr {
    id_length: i8,
    color_map_type: i8,
    data_type_code: i8,
    color_map_origin: i16,
    color_map_length: i16,
    color_map_depth: i8,
    x_origin: i16,
    y_origin: i16,
    width: i16,
    height: i16,
    bits_per_pixel: i8,
    image_descriptor: i8
}
enum RGBARepr {
    bgra(u8, u8, u8, u8),
    raw([u8; 4]),
    val(u32)
}

struct TGAColor {
    un: RGBARepr,
    bytes_pp: i32
}

impl TGAColor {
    fn get_default_tga_color() -> TGAColor {
        TGAColor {un: RGBARepr::val(0), bytes_pp: 1}
    }
    fn get_tga_color_with_rgba(R: u8, G: u8, B: u8, A: u8) -> TGAColor {
        TGAColor {un: RGBARepr::bgra(B, G, R, A), bytes_pp: 4}
    }
    fn get_tga_color_with_val(v: u32, b: i32) -> TGAColor {
        TGAColor {un: RGBARepr::val(v), bytes_pp: b}
    }
    fn get_tga_color_from(other: &TGAColor) -> TGAColor {
        TGAColor {un: *other.un, bytes_pp: *other.bytes_pp}
    }
}



struct TGAImage {
    data: Vec<u8>,
    width: i32,
    height: i32,
    bytes_pp: i32,
    //fmt: (u8, u8, u8)
}

impl TGAImage {
    fn get_default_tga_image() -> TGAImage {
        TGAImage {data: {}, width: 0, height: 0, bytes_pp: 0}
    }
    fn get_tga_image_with(&w: i32, h: i32, b: i32) -> TGAImage {
        TGAImage {data: [0, w * h * b], width: w,
            height: h, bytes_pp: b}
    }
    fn get_tga_image_from(other: &TGAImage) -> TGAImage {
        TGAImage {data: *other.data, width: *other.width,
            height: *other.height, bytes_pp: *other.bytes_pp}
    }
}

impl TGAImage {
    fn read_tga_file(path: &str) {
        if !data.isEmpty(){
            data.clean();
        }
    }
}
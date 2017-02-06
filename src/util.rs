use regex::Regex;

pub fn rgb_to_short(rgb: &str) -> usize {
	let matches = RE.captures(rgb).unwrap();
	let parts = vec!(
		u8::from_str_radix(matches.at(1).unwrap(), 16).unwrap(),
		u8::from_str_radix(matches.at(2).unwrap(), 16).unwrap(),
		u8::from_str_radix(matches.at(3).unwrap(), 16).unwrap(),
	);

    let mut best = 0;
    let mut best_distance = 255 * 255 * 3 + 1;
    for i in 16..255 {
        let ansi_color = ANSI_COLORS[i];
        let dr = ansi_color[0] - parts[0] as i32;
        let dg = ansi_color[1] - parts[1] as i32;
        let db = ansi_color[2] - parts[2] as i32;
        let distance = dr * dr + dg * dg + db * db;

        if distance < best_distance {
            best_distance = distance;
            best = i as usize;
        }
    }

    best
}
static ANSI_COLORS: [[i32; 3]; 256] = [
    [ 0x00, 0x00, 0x00 ],[ 0x80, 0x00, 0x00 ],[ 0x00, 0x80, 0x00 ],[ 0x80, 0x80, 0x00 ],[ 0x00, 0x00, 0x80 ],
    [ 0x80, 0x00, 0x80 ],[ 0x00, 0x80, 0x80 ],[ 0xc0, 0xc0, 0xc0 ],[ 0x80, 0x80, 0x80 ],[ 0xff, 0x00, 0x00 ],
    [ 0x00, 0xff, 0x00 ],[ 0xff, 0xff, 0x00 ],[ 0x00, 0x00, 0xff ],[ 0xff, 0x00, 0xff ],[ 0x00, 0xff, 0xff ],
    [ 0xff, 0xff, 0xff ],[ 0x00, 0x00, 0x00 ],[ 0x00, 0x00, 0x5f ],[ 0x00, 0x00, 0x87 ],[ 0x00, 0x00, 0xaf ],
    [ 0x00, 0x00, 0xd7 ],[ 0x00, 0x00, 0xff ],[ 0x00, 0x5f, 0x00 ],[ 0x00, 0x5f, 0x5f ],[ 0x00, 0x5f, 0x87 ],
    [ 0x00, 0x5f, 0xaf ],[ 0x00, 0x5f, 0xd7 ],[ 0x00, 0x5f, 0xff ],[ 0x00, 0x87, 0x00 ],[ 0x00, 0x87, 0x5f ],
    [ 0x00, 0x87, 0x87 ],[ 0x00, 0x87, 0xaf ],[ 0x00, 0x87, 0xd7 ],[ 0x00, 0x87, 0xff ],[ 0x00, 0xaf, 0x00 ],
    [ 0x00, 0xaf, 0x5f ],[ 0x00, 0xaf, 0x87 ],[ 0x00, 0xaf, 0xaf ],[ 0x00, 0xaf, 0xd7 ],[ 0x00, 0xaf, 0xff ],
    [ 0x00, 0xd7, 0x00 ],[ 0x00, 0xd7, 0x5f ],[ 0x00, 0xd7, 0x87 ],[ 0x00, 0xd7, 0xaf ],[ 0x00, 0xd7, 0xd7 ],
    [ 0x00, 0xd7, 0xff ],[ 0x00, 0xff, 0x00 ],[ 0x00, 0xff, 0x5f ],[ 0x00, 0xff, 0x87 ],[ 0x00, 0xff, 0xaf ],
    [ 0x00, 0xff, 0xd7 ],[ 0x00, 0xff, 0xff ],[ 0x5f, 0x00, 0x00 ],[ 0x5f, 0x00, 0x5f ],[ 0x5f, 0x00, 0x87 ],
    [ 0x5f, 0x00, 0xaf ],[ 0x5f, 0x00, 0xd7 ],[ 0x5f, 0x00, 0xff ],[ 0x5f, 0x5f, 0x00 ],[ 0x5f, 0x5f, 0x5f ],
    [ 0x5f, 0x5f, 0x87 ],[ 0x5f, 0x5f, 0xaf ],[ 0x5f, 0x5f, 0xd7 ],[ 0x5f, 0x5f, 0xff ],[ 0x5f, 0x87, 0x00 ],
    [ 0x5f, 0x87, 0x5f ],[ 0x5f, 0x87, 0x87 ],[ 0x5f, 0x87, 0xaf ],[ 0x5f, 0x87, 0xd7 ],[ 0x5f, 0x87, 0xff ],
    [ 0x5f, 0xaf, 0x00 ],[ 0x5f, 0xaf, 0x5f ],[ 0x5f, 0xaf, 0x87 ],[ 0x5f, 0xaf, 0xaf ],[ 0x5f, 0xaf, 0xd7 ],
    [ 0x5f, 0xaf, 0xff ],[ 0x5f, 0xd7, 0x00 ],[ 0x5f, 0xd7, 0x5f ],[ 0x5f, 0xd7, 0x87 ],[ 0x5f, 0xd7, 0xaf ],
    [ 0x5f, 0xd7, 0xd7 ],[ 0x5f, 0xd7, 0xff ],[ 0x5f, 0xff, 0x00 ],[ 0x5f, 0xff, 0x5f ],[ 0x5f, 0xff, 0x87 ],
    [ 0x5f, 0xff, 0xaf ],[ 0x5f, 0xff, 0xd7 ],[ 0x5f, 0xff, 0xff ],[ 0x87, 0x00, 0x00 ],[ 0x87, 0x00, 0x5f ],
    [ 0x87, 0x00, 0x87 ],[ 0x87, 0x00, 0xaf ],[ 0x87, 0x00, 0xd7 ],[ 0x87, 0x00, 0xff ],[ 0x87, 0x5f, 0x00 ],
    [ 0x87, 0x5f, 0x5f ],[ 0x87, 0x5f, 0x87 ],[ 0x87, 0x5f, 0xaf ],[ 0x87, 0x5f, 0xd7 ],[ 0x87, 0x5f, 0xff ],
    [ 0x87, 0x87, 0x00 ],[ 0x87, 0x87, 0x5f ],[ 0x87, 0x87, 0x87 ],[ 0x87, 0x87, 0xaf ],[ 0x87, 0x87, 0xd7 ],
    [ 0x87, 0x87, 0xff ],[ 0x87, 0xaf, 0x00 ],[ 0x87, 0xaf, 0x5f ],[ 0x87, 0xaf, 0x87 ],[ 0x87, 0xaf, 0xaf ],
    [ 0x87, 0xaf, 0xd7 ],[ 0x87, 0xaf, 0xff ],[ 0x87, 0xd7, 0x00 ],[ 0x87, 0xd7, 0x5f ],[ 0x87, 0xd7, 0x87 ],
    [ 0x87, 0xd7, 0xaf ],[ 0x87, 0xd7, 0xd7 ],[ 0x87, 0xd7, 0xff ],[ 0x87, 0xff, 0x00 ],[ 0x87, 0xff, 0x5f ],
    [ 0x87, 0xff, 0x87 ],[ 0x87, 0xff, 0xaf ],[ 0x87, 0xff, 0xd7 ],[ 0x87, 0xff, 0xff ],[ 0xaf, 0x00, 0x00 ],
    [ 0xaf, 0x00, 0x5f ],[ 0xaf, 0x00, 0x87 ],[ 0xaf, 0x00, 0xaf ],[ 0xaf, 0x00, 0xd7 ],[ 0xaf, 0x00, 0xff ],
    [ 0xaf, 0x5f, 0x00 ],[ 0xaf, 0x5f, 0x5f ],[ 0xaf, 0x5f, 0x87 ],[ 0xaf, 0x5f, 0xaf ],[ 0xaf, 0x5f, 0xd7 ],
    [ 0xaf, 0x5f, 0xff ],[ 0xaf, 0x87, 0x00 ],[ 0xaf, 0x87, 0x5f ],[ 0xaf, 0x87, 0x87 ],[ 0xaf, 0x87, 0xaf ],
    [ 0xaf, 0x87, 0xd7 ],[ 0xaf, 0x87, 0xff ],[ 0xaf, 0xaf, 0x00 ],[ 0xaf, 0xaf, 0x5f ],[ 0xaf, 0xaf, 0x87 ],
    [ 0xaf, 0xaf, 0xaf ],[ 0xaf, 0xaf, 0xd7 ],[ 0xaf, 0xaf, 0xff ],[ 0xaf, 0xd7, 0x00 ],[ 0xaf, 0xd7, 0x5f ],
    [ 0xaf, 0xd7, 0x87 ],[ 0xaf, 0xd7, 0xaf ],[ 0xaf, 0xd7, 0xd7 ],[ 0xaf, 0xd7, 0xff ],[ 0xaf, 0xff, 0x00 ],
    [ 0xaf, 0xff, 0x5f ],[ 0xaf, 0xff, 0x87 ],[ 0xaf, 0xff, 0xaf ],[ 0xaf, 0xff, 0xd7 ],[ 0xaf, 0xff, 0xff ],
    [ 0xd7, 0x00, 0x00 ],[ 0xd7, 0x00, 0x5f ],[ 0xd7, 0x00, 0x87 ],[ 0xd7, 0x00, 0xaf ],[ 0xd7, 0x00, 0xd7 ],
    [ 0xd7, 0x00, 0xff ],[ 0xd7, 0x5f, 0x00 ],[ 0xd7, 0x5f, 0x5f ],[ 0xd7, 0x5f, 0x87 ],[ 0xd7, 0x5f, 0xaf ],
    [ 0xd7, 0x5f, 0xd7 ],[ 0xd7, 0x5f, 0xff ],[ 0xd7, 0x87, 0x00 ],[ 0xd7, 0x87, 0x5f ],[ 0xd7, 0x87, 0x87 ],
    [ 0xd7, 0x87, 0xaf ],[ 0xd7, 0x87, 0xd7 ],[ 0xd7, 0x87, 0xff ],[ 0xd7, 0xaf, 0x00 ],[ 0xd7, 0xaf, 0x5f ],
    [ 0xd7, 0xaf, 0x87 ],[ 0xd7, 0xaf, 0xaf ],[ 0xd7, 0xaf, 0xd7 ],[ 0xd7, 0xaf, 0xff ],[ 0xd7, 0xd7, 0x00 ],
    [ 0xd7, 0xd7, 0x5f ],[ 0xd7, 0xd7, 0x87 ],[ 0xd7, 0xd7, 0xaf ],[ 0xd7, 0xd7, 0xd7 ],[ 0xd7, 0xd7, 0xff ],
    [ 0xd7, 0xff, 0x00 ],[ 0xd7, 0xff, 0x5f ],[ 0xd7, 0xff, 0x87 ],[ 0xd7, 0xff, 0xaf ],[ 0xd7, 0xff, 0xd7 ],
    [ 0xd7, 0xff, 0xff ],[ 0xff, 0x00, 0x00 ],[ 0xff, 0x00, 0x5f ],[ 0xff, 0x00, 0x87 ],[ 0xff, 0x00, 0xaf ],
    [ 0xff, 0x00, 0xd7 ],[ 0xff, 0x00, 0xff ],[ 0xff, 0x5f, 0x00 ],[ 0xff, 0x5f, 0x5f ],[ 0xff, 0x5f, 0x87 ],
    [ 0xff, 0x5f, 0xaf ],[ 0xff, 0x5f, 0xd7 ],[ 0xff, 0x5f, 0xff ],[ 0xff, 0x87, 0x00 ],[ 0xff, 0x87, 0x5f ],
    [ 0xff, 0x87, 0x87 ],[ 0xff, 0x87, 0xaf ],[ 0xff, 0x87, 0xd7 ],[ 0xff, 0x87, 0xff ],[ 0xff, 0xaf, 0x00 ],
    [ 0xff, 0xaf, 0x5f ],[ 0xff, 0xaf, 0x87 ],[ 0xff, 0xaf, 0xaf ],[ 0xff, 0xaf, 0xd7 ],[ 0xff, 0xaf, 0xff ],
    [ 0xff, 0xd7, 0x00 ],[ 0xff, 0xd7, 0x5f ],[ 0xff, 0xd7, 0x87 ],[ 0xff, 0xd7, 0xaf ],[ 0xff, 0xd7, 0xd7 ],
    [ 0xff, 0xd7, 0xff ],[ 0xff, 0xff, 0x00 ],[ 0xff, 0xff, 0x5f ],[ 0xff, 0xff, 0x87 ],[ 0xff, 0xff, 0xaf ],
    [ 0xff, 0xff, 0xd7 ],[ 0xff, 0xff, 0xff ],[ 0x08, 0x08, 0x08 ],[ 0x12, 0x12, 0x12 ],[ 0x1c, 0x1c, 0x1c ],
    [ 0x26, 0x26, 0x26 ],[ 0x30, 0x30, 0x30 ],[ 0x3a, 0x3a, 0x3a ],[ 0x44, 0x44, 0x44 ],[ 0x4e, 0x4e, 0x4e ],
    [ 0x58, 0x58, 0x58 ],[ 0x60, 0x60, 0x60 ],[ 0x66, 0x66, 0x66 ],[ 0x76, 0x76, 0x76 ],[ 0x80, 0x80, 0x80 ],
    [ 0x8a, 0x8a, 0x8a ],[ 0x94, 0x94, 0x94 ],[ 0x9e, 0x9e, 0x9e ],[ 0xa8, 0xa8, 0xa8 ],[ 0xb2, 0xb2, 0xb2 ],
    [ 0xbc, 0xbc, 0xbc ],[ 0xc6, 0xc6, 0xc6 ],[ 0xd0, 0xd0, 0xd0 ],[ 0xda, 0xda, 0xda ],[ 0xe4, 0xe4, 0xe4 ],
    [ 0xee, 0xee, 0xee ]];

lazy_static! {
    static ref RE: Regex = Regex::new("(..)(..)(..)").unwrap();
}

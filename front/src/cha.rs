/*
 * https://crates.io/crates/cha
 */

pub(crate) struct Color(u8, u8, u8);

impl Color {
    pub fn from(input: &str) -> Color {
        let mut color = Color(0, 0, 0);
        color.update(input);
        color
    }

    pub fn update(&mut self, input: &str) -> &Color {
        use sha2::Digest as _;

        let raw = sha2::Sha256::digest(input.as_bytes());

        self.0 = raw[0];
        self.1 = raw[1];
        self.2 = raw[2];
        self
    }

    pub fn to_color_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }

    pub fn is_dark(&self) -> bool {
        self.hsp() < 127.5
    }

    /*
     * http://alienryderflex.com/hsp.htm
     */
    fn hsp(&self) -> f32 {
        let r = self.0 as u32;
        let g = self.1 as u32;
        let b = self.2 as u32;

        (0.299 * r.pow(2) as f32 + 0.587 * g.pow(2) as f32 + 0.114 * b.pow(2) as f32).sqrt()
    }
}

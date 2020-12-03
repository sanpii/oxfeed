/*
 * https://crates.io/crates/cha
 */

#[derive(Debug)]
pub(crate) struct Color(u8, u8, u8);

impl Color {
    pub(crate) fn from(input: &str) -> Color {
        let mut color = Color(0, 0, 0);
        color.update(input);
        color
    }

    pub(crate) fn update(&mut self, input: &str) -> &Color {
        let raw = ring::digest::digest(&ring::digest::SHA256, input.as_bytes());
        let raw = raw.as_ref();

        self.0 = raw[0];
        self.1 = raw[1];
        self.2 = raw[2];
        self
    }

    pub(crate) fn to_color_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }

    pub(crate) fn is_dark(&self) -> bool {
        self.hsp() < 127.5
    }

    /*
     * http://alienryderflex.com/hsp.htm
     */
    fn hsp(&self) -> f32 {
        (0.299 * self.0.pow(2) as f32 + 0.587 * self.1.pow(2) as f32 + 0.114 * self.2.pow(2) as f32)
            .sqrt()
    }
}

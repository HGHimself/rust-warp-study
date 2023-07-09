pub struct Background {
    count: u32,
    frequency: u32,
    x_amplitude: u32,
    y_amplitude: u32,
    x_multiplier: u32,
    y_multiplier: u32,
    color: u32,
    thickness: u32,
}

impl Background {
    pub fn new(
        count: u32,
        frequency: u32,
        x_amplitude: u32,
        y_amplitude: u32,
        x_multiplier: u32,
        y_multiplier: u32,
        color: u32,
        thickness: u32,
    ) -> Self {
        Self {
            count,
            frequency,
            x_amplitude,
            y_amplitude,
            x_multiplier,
            y_multiplier,
            color,
            thickness,
        }
    }

    pub fn to_call(&self) -> String {
        format!(
            "<script>showBackground({{
        count: {},
        frequency: {},
        xAmplitude: {},
        yAmplitude: {},
        xMultiplier: {},
        yMultiplier: {},
        color: {},
        thickness: {},
    }})</script>",
            self.count,
            self.frequency,
            self.x_amplitude,
            self.y_amplitude,
            self.x_multiplier,
            self.y_multiplier,
            self.color,
            self.thickness
        )
    }
}

pub fn background_random() -> String {
    String::from("<script>showBackground({})</script>")
}

pub fn index() -> String {
    Background::new(81, 10, 1798, 1571, 14, 11, 5, 97).to_call()
}

pub fn login() -> String {
    Background::new(91, 3, 1346, 903, 7, 14, 1985, 53).to_call()
}

pub fn signup() -> String {
    Background::new(79,
        7,
        2066,
        1165,
        2,
        13,
        415,
        101,).to_call()
}



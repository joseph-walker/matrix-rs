use std::fmt::Display;

#[derive(Debug)]
pub struct Vector(pub Vec<f64>);

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();

        buffer.push_str("<");
        buffer.push_str(
            &self.0
                .iter()
                .map(|v| format!("{}", v))
                .collect::<Vec<String>>()
                .join(","),
        );
        buffer.push_str(">");

        write!(f, "{}", buffer)
    }
}

impl Vector {
    pub fn new_from_slice(values: Box<[f64]>) -> Vector {
        Vector(values.into())
    }

    pub fn scale(&mut self, scale: f64) -> () {
        for n in self.0.iter_mut() {
            *n *= scale;
        }
    }

    pub fn drop(&mut self, n: usize) -> () {
        self.0.drain(0..n);
    }
}

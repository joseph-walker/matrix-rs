use std::fmt::Display;

pub struct Vector {
    pub size: usize,
    pub values: Box<[f64]>,
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();

        buffer.push_str("<");
        buffer.push_str(
            &self
                .values
                .into_iter()
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
        Vector {
            size: values.len(),
            values,
        }
    }

    pub fn scale(&mut self, scale: f64) -> () {
        for n in self.values.iter_mut() {
            *n *= scale;
        }
    }
}


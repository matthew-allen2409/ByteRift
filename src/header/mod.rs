#[derive(Debug, PartialEq)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl Header {
    pub fn new(name: String, value: String) -> Result<Self, String> {
        match (name.is_empty(), value.is_empty()) {
            (true, _) | (_, true) => return Err(format!("name and value cannot be empty!")),
            _ => ()
        };

        Ok(Self {name, value})
    }

    pub fn from_string(str: &String) -> Result<Self, String> {
        let mut parts = str.split(':').map(|part| part.trim());
        let name = match parts.next() {
            Some(name) => String::from(name),
            None => return Err(format!("Invalid header: {str}")),
        };
        let value = match parts.next() {
            Some(value) => String::from(value),
            None => return Err(format!("Invalid header: {str}")),
        };

        Ok(Self::new(name, value)?)
    }

    pub fn to_string(&self) -> String {
        format!("{}: {}\r\n", self.name, self.value)
    }
}

#[cfg(test)]
mod tests;

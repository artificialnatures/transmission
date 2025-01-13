#[derive(Debug, Clone)]
pub struct TransmissionError {
    description: String
}

impl TransmissionError {
    pub fn new(description: &str) -> TransmissionError {
        TransmissionError { description: description.to_string() }
    }
}

impl std::fmt::Display for TransmissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}
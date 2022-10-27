#[derive(Debug, Copy, Clone)]
pub enum RuntimeValue {
    Number(f64),
    Boolean(bool),
    Null,
}

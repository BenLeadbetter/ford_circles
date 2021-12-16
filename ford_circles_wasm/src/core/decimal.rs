pub trait Decimal {
    fn to_f64(&self) -> f64;
    fn to_f32(&self) -> f32;
}
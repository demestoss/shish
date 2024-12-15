#[warn(variant_size_differences)]
pub enum TokenModifier {
    Optional,
    OneOrMore,
    Exact(usize),
    AtLeast(usize),
    Between(usize, usize),
}

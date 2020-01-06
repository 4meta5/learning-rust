/// Magma
///
/// a set equipped with a single binary operation that is closed

trait Magma {
    fn mul(Self, Self) -> Self;
}
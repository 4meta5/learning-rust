/// Monoid
///
/// a semigroup with an identity element

use semigroup::Semigroup;

trait Monoid: Semigroup {
    fn unit() -> Self;
}
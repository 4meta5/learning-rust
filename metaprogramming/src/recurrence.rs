/// > [source](https://danielkeep.github.io/tlborm/book/pim-README.html)

/// Abstract implementation of recurrence relations using macros
///
/// let fib = recurrence![a[n] = 0, 1, ..., a[n-1] + a[n-2]];
///
/// for e in fib.take(10) { println!("{}", e) }

/// Defining a macro called recurrence!
/// This macro has a single parsing rule -- says the input must match
/// * the literal token sequence `a` `[` `n` `]` `=`
/// * a repreating (the `$(...)`) sequence, using `,` as a separator, and one or more (`+`) repeats of:
///     * a valid expression captured into the variable `inits` (`$inits:expr`)
/// * the literal token sequence `,` `....` `,`
/// * a valid expression captured into the variable `recur` (`$recur:expr`)
/// The rule says that if the input matches this rule, the macro invocation is replaced by the token sequence in {}
macro_rules! count_exprs {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),*) => (1 + count_exprs!($($tail),*));
}

macro_rules! recurrence {
    ( $seq:ident [ $ind:ident ]: $sty:ty = $($inits:expr),+ ... $recur:expr ) => {
//    ^~~~~~~~~~   ^~~~~~~~~~ changed
        {
            use std::ops::Index;

            const MEM_SIZE: usize = count_exprs!($($inits),+);

            struct Recurrence {
                mem: [$sty; MEM_SIZE],
                pos: usize,
            }

            struct IndexOffset<'a> {
                slice: &'a [$sty; MEM_SIZE],
                offset: usize,
            }

            impl<'a> Index<usize> for IndexOffset<'a> {
                type Output = $sty;

                #[inline(always)]
                fn index<'b>(&'b self, index: usize) -> &'b $sty {
                    use std::num::Wrapping;

                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(MEM_SIZE);

                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }

            impl Iterator for Recurrence {
                type Item = $sty;

                #[inline]
                fn next(&mut self) -> Option<$sty> {
                    if self.pos < MEM_SIZE {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let $ind = self.pos;
//                              ^~~~ changed
                            let $seq = IndexOffset { slice: &self.mem, offset: $ind };
//                              ^~~~ changed
                            $recur
                        };

                        {
                            use std::mem::swap;

                            let mut swap_tmp = next_val;
                            for i in (0..MEM_SIZE).rev() {
                                swap(&mut swap_tmp, &mut self.mem[i]);
                            }
                        }

                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }

            Recurrence { mem: [$($inits),+], pos: 0 }
        }
    };
}

fn main() {
    let fib = recurrence![a[n]: u64 = 0, 1 ... a[n-1] + a[n-2]];

    for e in fib.take(10) { println!("{}", e) }
}
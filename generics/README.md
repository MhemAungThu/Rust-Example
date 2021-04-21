# Generics

First, defined trait `DoubleDrop` on _src/lib.rs_ which is a generic trait over a parameter
`T` and generic caller `U`.

> I've tried to bound the type `U` with `Copy + Clone` and called on the type which does
> not implement `Copy + Clone`, but the call is work well.

Code examples and explainations for the generic trait, and why should we use associative type for return.
Check [RFCs-0195](https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md) for more informations.

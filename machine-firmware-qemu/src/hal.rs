// 这个包应该在hal库里面，这里就相当于帮着实现了这个hal库
// 这样就能用整个rust hal的生态圈了
// Ref: MeowSBI

mod ns16550a;
pub use ns16550a::Ns16550a;

mod clint;
pub use clint::Clint;

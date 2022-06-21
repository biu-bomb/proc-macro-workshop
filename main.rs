// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

use seq::seq;
// 传入某个宏，处理256
macro_rules! pass_nproc {
    ($mac:ident) => {
        $mac! { 256 }
    };
}

// 直接返回传入的字面量
macro_rules! literal_identity_macro {
    ($nproc:literal) => {
        $nproc
    };
}

// Expands to: `const NPROC: usize = 256;`
// 处理某个宏，展开后返回一个定义的字面量
const NPROC: usize = pass_nproc!(literal_identity_macro);

struct Proc;

impl Proc {
    const fn new() -> Self {
        Proc
    }
}

// 传入某个字面量，然后进行宏展开
macro_rules! make_procs_array {
    ($nproc:literal) => {
        seq!(N in 0..$nproc { [#(Proc::new(),)*] })
    }
}

// Expands to: `static PROCS: [Proc; NPROC] = [Proc::new(), ..., Proc::new()];`
static PROCS: [Proc; NPROC] = pass_nproc!(make_procs_array);

fn main() {}

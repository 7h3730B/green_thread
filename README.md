# green_thread
My own little thread implementation based on: https://cfsamson.gitbook.io/green-threads-explained-in-200-lines-of-rust/background-information

for x86-64

## This should never be used in production and is not really practicle or portable in anyway

> Should need nightly?

### TODO:
- [  ] Port to new `asm!` Macro ### Needs some investigating. Throws ACCESS_VIOLATIONs at Runtime (not with llvm_asm)
- [  ] arm support
- [x] use vec.into_boxed_slice() to avoid reallocation problem
- [  ] Windows support: https://cfsamson.gitbook.io/green-threads-explained-in-200-lines-of-rust/supporting-windows
    - [  ] Add floating point registers to be call safe
    - [  ] Change the stack layout for windows

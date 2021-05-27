#![feature(llvm_asm)]

pub const SSIZE: isize = 48;

#[derive(Debug, Default)]
#[repr(C)]
pub struct ThreadContext {
    pub rsp: u64,
}

pub unsafe fn gt_switch(new: *const ThreadContext) {
    // asm!("mov rsp, [{0}]", in(reg) new);
    llvm_asm!("
        mov     0x00($0), %rsp
        ret"
    :
    : "r"(new)
    :
    : "alignstack"
    );
}

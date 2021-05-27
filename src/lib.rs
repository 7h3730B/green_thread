#![feature(llvm_asm, naked_functions, asm)]
static mut RUNTIME: usize = 0;

#[derive(Debug)]
pub struct Runtime {
    threads: Vec<Thread>,
    current: usize,
    stack_size: usize,
}

impl Runtime {
    pub fn new(stack_size: usize) -> Self {
        let threads: Vec<Thread> = vec![Thread {
            id: 0,
            stack: vec![0_u8; stack_size].into_boxed_slice(),
            ctx: ThreadContext::default(),
            state: State::Running,
        }];

        Runtime {
            threads,
            current: 0,
            stack_size: stack_size,
        }
    }

    fn new_thread(&mut self) -> &mut Thread {
        println!("new_thread: {}", self.threads.len());
        self.threads
            .push(Thread::new(self.threads.len() + 1, self.stack_size));
        self.threads.last_mut().unwrap()
    }

    pub fn spawn(&mut self, f: fn()) {
        let mut available = self
            .threads
            .iter_mut()
            .find(|t| t.state == State::Available);

        if let None = available {
            self.new_thread();
            return self.spawn(f);
        }

        let available = available.unwrap();

        let size = available.stack.len();
        unsafe {
            let s_ptr = available.stack.as_mut_ptr().offset(size as isize);
            let s_ptr = (s_ptr as usize & !15) as *mut u8;
            std::ptr::write(s_ptr.offset(-16) as *mut u64, guard as u64);
            std::ptr::write(s_ptr.offset(-24) as *mut u64, skip as u64);
            std::ptr::write(s_ptr.offset(-32) as *mut u64, f as u64);
            available.ctx.rsp = s_ptr.offset(-32) as u64;
        }
        available.state = State::Ready;
    }

    pub fn init(&self) {
        unsafe {
            let r_ptr: *const Runtime = self;
            RUNTIME = r_ptr as usize;
        }
    }

    pub fn run(&mut self) -> ! {
        while self.t_yield() {}
        std::process::exit(0);
    }

    fn t_return(&mut self) {
        if self.current != 0 {
            self.threads[self.current].state = State::Available;
            self.t_yield();
        }
    }

    // Round Robin
    fn t_yield(&mut self) -> bool {
        let mut pos = self.current;
        while self.threads[pos].state != State::Ready {
            pos += 1;
            if pos == self.threads.len() {
                pos = 0;
            }
            if pos == self.current {
                return false;
            }
        }

        if self.threads[self.current].state != State::Available {
            self.threads[self.current].state = State::Ready;
        }

        self.threads[pos].state = State::Running;
        let old_pos = self.current;
        self.current = pos;

        unsafe {
            let old: *mut ThreadContext = &mut self.threads[old_pos].ctx;
            let new: *const ThreadContext = &self.threads[pos].ctx;
            llvm_asm!(
                "mov $0, %rdi
                mov $1, %rsi"::"r"(old), "r"(new)
            );
            switch();
        }
        self.threads.len() > 0
    }
}

fn guard() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_return();
    }
}

#[naked]
fn skip() {
    unsafe {
        asm!("nop");
    }
}

pub fn yield_thread() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_yield();
    }
}

#[naked]
#[inline(never)]
unsafe fn switch() {
    llvm_asm!(
        "
            mov     %rsp, 0x00(%rdi)
            mov     %r15, 0x08(%rdi)
            mov     %r14, 0x10(%rdi)
            mov     %r13, 0x18(%rdi)
            mov     %r12, 0x20(%rdi)
            mov     %rbx, 0x28(%rdi)
            mov     %rbp, 0x30(%rdi)

            mov     0x00(%rsi), %rsp
            mov     0x08(%rsi), %r15
            mov     0x10(%rsi), %r14
            mov     0x18(%rsi), %r13
            mov     0x20(%rsi), %r12
            mov     0x28(%rsi), %rbx
            mov     0x30(%rsi), %rbp
            "
    );
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    Available,
    Running,
    Ready,
}

#[derive(Debug)]
struct Thread {
    pub id: usize,
    pub stack: Box<[u8]>,
    pub ctx: ThreadContext,
    pub state: State,
}

impl Thread {
    fn new(id: usize, stack_size: usize) -> Self {
        Thread {
            id,
            stack: vec![0_u8; stack_size].into_boxed_slice(),
            ctx: ThreadContext::default(),
            state: State::Available,
        }
    }
}

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
}

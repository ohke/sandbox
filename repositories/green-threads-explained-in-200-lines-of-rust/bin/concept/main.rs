use core::arch::asm;
use std::io::Write;

const SSIZE: isize = 1024;
static mut S_PTR: *const u8 = 0 as *const u8;

#[derive(Debug, Default)]
#[repr(C)] // Cの構造体のメモリマップにする
struct ThreadContext {
    rsp: u64, // stack pointer
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
}

fn print_stack(filename: &str) {
    let mut f = std::fs::File::create(filename).unwrap();
    unsafe {
        for i in (0..SSIZE).rev() {
            writeln!(
                f,
                "{:x} {:x}",
                S_PTR.offset(i as isize) as usize,
                *S_PTR.offset(i as isize)
            );
        }
    }
}

fn hello() -> ! {
    println!("I LOVE WAKING UP ON A NEW STACK!");
    print_stack("AFTER.txt");
    loop {}
}

unsafe fn gt_switch(new: *const ThreadContext) {
    asm!(
        "mov rsp, [{0} + 0x00]", // スタックの最上位アドレスの値 (リターン先) をrspへセット
        "ret", // スタック
        in(reg) new,
    );
}

fn main() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0_u8; SSIZE as usize];
    let stack_ptr = stack.as_mut_ptr();

    unsafe {
        let stack_bottom = stack.as_mut_ptr().offset(SSIZE);
        // 16バイトアラインメント (stack_bottomの先頭が16バイトで割り切れない場合は切り捨てる)
        let sb_aligned = (stack_bottom as usize & !15) as *mut u8;
        S_PTR = sb_aligned;
        // hello関数のアドレスで書き換える
        std::ptr::write(stack_ptr.offset(SSIZE - 16) as *mut u64, hello as u64);
        print_stack("BEFORE.txt");
        // スタックポインタを書き換えたアドレスをセット
        ctx.rsp = stack_ptr.offset(SSIZE - 16) as u64;

        gt_switch(&mut ctx);
    }
}

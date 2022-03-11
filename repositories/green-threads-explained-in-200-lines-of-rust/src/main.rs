use core::arch::asm;

const SSIZE: isize = 48;

#[derive(Debug, Default)]
#[repr(C)] // Cの構造体のメモリマップにする
struct ThreadContext {
    rsp: u64, // stack pointer
}

fn hello() -> ! {
    println!("I LOVE WAKING UP ON A NEW STACK!");
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

    unsafe {
        let stack_bottom = stack.as_mut_ptr().offset(SSIZE);
        // 16バイトアラインメント (stack_bottomの先頭が16バイトで割り切れない場合は切り捨てる)
        let sb_aligned = (stack_bottom as usize & !15) as *mut u8;
        // hello関数のアドレスで書き換える
        std::ptr::write(sb_aligned.offset(-16) as *mut u64, hello as u64);
        // スタックポインタを書き換えたアドレスをセット
        ctx.rsp = sb_aligned.offset(-16) as u64;
        gt_switch(&mut ctx);
    }
}

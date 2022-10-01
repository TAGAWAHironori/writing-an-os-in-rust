//標準ライブラリの無効(panic_handlerとlanguage itemに変わるものが必要)
#![no_std]
//通常のエントリポイントを使用しないことを宣言
#![no_main]

use core::panic::PanicInfo;

//パニックハンドラーの実装
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

//関数名をマングルしないように指示(名前修飾をしない)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop{}
}
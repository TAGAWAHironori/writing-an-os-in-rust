//標準ライブラリの無効(panic_handlerとlanguage itemに変わるものが必要)
#![no_std]
//通常のエントリポイントを使用しないことを宣言
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

//パニックハンドラーの実装
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//関数名をマングルしないように指示(名前修飾をしない)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    loop{}
}

//標準ライブラリの無効(panic_handlerとlanguage itemに変わるものが必要)
#![no_std]
//通常のエントリポイントを使用しないことを宣言
#![no_main]

use core::panic::PanicInfo;

//関数名をマングルしないように指示
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop{}
}

//パニックハンドラーの実装
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
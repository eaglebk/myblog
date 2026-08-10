// ?ignore
// ?hidden:start
#![no_std]
#![no_main]

use core::panic::PanicInfo;
// ?hidden:end

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // В реальной прошивке здесь логгируют ошибку в UART
    loop {}
}

fn main() {
    // Точка входа прошивки без стандартной библиотеки std
}

// ! **Разбор каркаса #![no_std]**
// ! - **#![no_std]**: Отключает тяжелый C-runtime и стандартную библиотеку OS.
// ! - **#![no_main]**: Отменяет поиск стандартной точки входа main.
// ! - **#[panic_handler]**: Функция, вызываемая компилятором при любой ошибке паники в микроконтроллере.

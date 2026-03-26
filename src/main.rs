#![no_std]
#![no_main]

use core::arch::asm;

#[cfg(not(test))]
use core::panic::PanicInfo;
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} // ignorar error, igual compila si usas el target x86_64-unknown-none, instalar primero
}

#[unsafe(no_mangle)]
// pub extern "C" fn _start() -> ! { // para que se mantenga en la convecion C
// pero esto tambien me funciono
pub fn _start() -> ! {
    let a = 5;
    let b = 10;

    let suma = a + b; // 15

    // el 15 se convierte en ['1', '5', '\n']
    let mut buf = [0u8; 3];
    buf[0] = (suma / 10) + b'0';
    buf[1] = (suma % 10) + b'0';
    buf[2] = b'\n';

    pintar_en_terminal(&buf);

    unsafe {
        // Syscall: exit(0) -> ESTO evita el Core Dump/SIGSEGV
        asm!(
            // lo mismo de arriba
            "syscall",             // Ejecuta la llamada
            in("rax") 60,          // ID de la syscall: 60 es 'sys_exit'
            in("rdi") 0,           // Argumento 1: Código de salida (0 significa éxito)
            options(noreturn)      // Le dice a Rust: "Este código nunca vuelve aquí"
        );
    }
}

fn pintar_en_terminal(buf: &[u8]) {
    // Syscall: write(1, buf, buf.len)
    unsafe {
        asm!(
            // comentarios hechos con IA porque no se que hacia
            "syscall",               // 1. Ejecuta la interrupción de llamada al sistema
            inout("rax") 1 => _,     // 2. ID de la syscall: 1 es 'sys_write'
            in("rdi") 1,             // 3. Argumento 1: File Descriptor (1 es stdout/pantalla)
            in("rsi") buf.as_ptr(),  // 4. Argumento 2: Puntero a la memoria donde están los datos
            in("rdx") buf.len(),     // 5. Argumento 3: Cuántos bytes escribir (en este caso 3)
            out("rcx") _,            // 6. Registros que el kernel puede ensuciar/usar
            out("r11") _,            //    (Los marcamos como ignorados '_')
        );
    }
}

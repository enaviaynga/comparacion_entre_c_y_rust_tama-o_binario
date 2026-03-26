/* #include <unistd.h>

int main() {
    const char msg[] = "Hola desde Rust real\n";
    // STDOUT_FILENO es el file descriptor 1
    // Usamos write directamente para evitar el overhead de <stdio.h>
    write(STDOUT_FILENO, msg, sizeof(msg) - 1);
    return 0;
}
*/

/*
#include <stdio.h>

int main() {
    int a = 5;
    int b = 10;
    int suma = a + b;
    printf("%d\n", suma);
    return 0;
}
*/

// Compilar con: gcc -nostdlib -o c_mini main.c
void _start() {
    int a = 5;
    int b = 10;
    int suma = a + b;

    char buf[3];
    buf[0] = (suma / 10) + '0';
    buf[1] = (suma % 10) + '0';
    buf[2] = '\n';

    // Llamada al sistema write en x86_64
    long syscall_write = 1;
    long fd = 1;
    __asm__(
        "syscall"
        :
        : "a"(syscall_write), "D"(fd), "S"(buf), "d"(3)
        : "rcx", "r11"
    );

    // Llamada al sistema exit para evitar SIGSEGV
    long syscall_exit = 60;
    __asm__(
        "syscall"
        :
        : "a"(syscall_exit), "D"(0)
        : "rcx", "r11"
    );
}

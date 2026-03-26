## 🔍 Crónica de una curiosidad: C vs Rust en "Low Level" puro

Inspirado por un video sobre la optimizacion de tamaños de los binarios ([video](https://youtu.be/5Dpdfwm8Hlg)), me surgió una duda: ¿Qué tan pequeños pueden ser realmente los binarios si forzamos a Rust y a C a trabajar en condiciones similares? 

Normalmente, Rust tiene la fama de generar binarios pesados debido a su gestión de `panics`, `unwinding` y metadatos. Sin embargo, al realizar esta prueba, usando **ensamblador directo para las Syscalls**, para que sea las condiciones mas iguales posibles, me llevé una sorpresa que no esperaba:

### 📊 Los Resultados
| Lenguaje | Tamaño del Binario | Configuración | Salida |
| :--- | :--- | :--- | :--- |
| **Rust** 🦀 | **1.5 KiB** | `#![no_std]`, `#![no_main]`, target `x86_64-unknown-none` | 15 |
| **C** 🟦 | **13.0 KiB** | `gcc -nostdlib` | 15 |
| **C** 🟦 | **4.0 KiB** | `gcc -nostdlib -s -static -O3 -fno-asynchronous-unwind-tables -Wl,--build-id=none main.c -o c_mini` | nada |

> **Nota:** Recalco, mi primer lenguaje fue Rust, no se programar C. Este experimento lo inicie pensando que seria mas parejo o rust pesando mas, pero a diferencia de "Linux con Last Dragon", no voy a afirmar que uno sea mejor que otro en este caso, porque no se si c se pueda compilar mejor o he pecado de ignorancia en alguna parte. Aun asi me gustaria aprender mas.

---

### 💻 El Experimento: Suma y Syscalls

En ambos casos, sume un 10 y un 5, evitando las funciones de impresión estándar y bajé al nivel del kernel usando la interrupción `syscall` de x86_64 para escribir en la terminal y salir del proceso de forma limpia. De esta forma sera igual para ambos casos, no hay errores comparativos por usr un \[u8\] vs String o una simple salida de terminal contra una macro que contruye un texto antes de imprimir (`printf` y `println!`).

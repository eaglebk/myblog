// Шаг 1: Автоматическая векторизация обычного цикла
fn add_slices(a: &[f32], b: &[f32], c: &mut [f32]) {
    let len = a.len().min(b.len()).min(c.len());
    for i in 0..len {
        c[i] = a[i] + b[i];
    }
}

fn main() {
    let a = [1.0_f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let b = [10.0_f32, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0];
    let mut c = [0.0_f32; 8];

    add_slices(&a, &b, &mut c);
    println!("Результат автовекторизации: {:?}", c);
}

// ! **1. Автоматическая векторизация (Auto-vectorization)**
// ! - Код выглядит как самый обычный скалярный цикл без специальных API.
// ! - При включённой оптимизации (`--release`) LLVM может сам заменить цикл на векторные инструкции.
// ! - Итоговый результат зависит от целевой архитектуры, флагов сборки и отсутствия зависимостей между итерациями.

// ---

// Шаг 2: Прямое управление через std::arch (Intrinsics)
fn add_simd_4(a: &[f32; 4], b: &[f32; 4]) -> [f32; 4] {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use std::arch::x86_64::{_mm_add_ps, _mm_loadu_ps, _mm_storeu_ps};
        let va = _mm_loadu_ps(a.as_ptr());
        let vb = _mm_loadu_ps(b.as_ptr());
        let sum = _mm_add_ps(va, vb);
        let mut out = [0.0_f32; 4];
        _mm_storeu_ps(out.as_mut_ptr(), sum);
        return out;
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        use std::arch::aarch64::{vaddq_f32, vld1q_f32, vst1q_f32};
        let va = vld1q_f32(a.as_ptr());
        let vb = vld1q_f32(b.as_ptr());
        let sum = vaddq_f32(va, vb);
        let mut out = [0.0_f32; 4];
        vst1q_f32(out.as_mut_ptr(), sum);
        return out;
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
    }
}

fn main() {
    let a = [1.0_f32, 2.0, 3.0, 4.0];
    let b = [10.0_f32, 20.0, 30.0, 40.0];
    let result = add_simd_4(&a, &b);
    println!("Результат std::arch: {:?}", result);
}

// ! **2. Интринсики через `std::arch`**
// ! - На `x86_64` используется SSE (`_mm_loadu_ps` + `_mm_add_ps`), на `aarch64` - NEON (`vld1q_f32` + `vaddq_f32`).
// ! - Суффикс `u` в `_mm_loadu_ps` означает *unaligned* - загрузка без требования строгого выравнивания адреса в памяти.
// ! - Прямая работа с интринсиками требует блока `unsafe` и ручного учёта остатка элементов, если длина массива не кратна ширине вектора.

// ---

// ?ignore
// Шаг 3: Portable SIMD (требует nightly и #![feature(portable_simd)])
#![feature(portable_simd)]
use std::simd::Simd;

fn main() {
    let a = Simd::<f32, 4>::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = Simd::<f32, 4>::from_array([10.0, 20.0, 30.0, 40.0]);
    let result = a + b;
    println!("Результат Portable SIMD: {:?}", result);
}

// ! **3. Portable SIMD (`std::simd`)**
// ! - Позволяет писать переносимый векторный код без привязки к SSE, AVX2 или NEON.
// ! - Привычные операторы вроде `+` работают сразу над целым вектором `Simd<f32, 4>`.
// ! - На момент написания статьи `std::simd` остаётся экспериментальным API и доступен только на nightly-версии Rust.

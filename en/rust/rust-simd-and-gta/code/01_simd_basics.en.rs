// Step 1: Auto-vectorization of a plain loop
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
    println!("Auto-vectorization result: {:?}", c);
}

// ! **1. Auto-vectorization**
// ! - The code looks like a regular scalar loop with no special APIs.
// ! - In `--release` builds, LLVM can replace the loop with vector instructions on its own.
// ! - Whether it actually vectorizes depends on the target CPU, compiler flags, and absence of loop-carried dependencies.

// ---

// Step 2: Direct control via std::arch (Intrinsics)
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
    println!("std::arch result: {:?}", result);
}

// ! **2. Intrinsics via `std::arch`**
// ! - Uses SSE on `x86_64` (`_mm_loadu_ps` + `_mm_add_ps`) and NEON on `aarch64` (`vld1q_f32` + `vaddq_f32`).
// ! - The `u` suffix in `_mm_loadu_ps` stands for *unaligned*: loading without strict memory alignment requirements.
// ! - Working directly with intrinsics requires `unsafe` blocks and manual handling of any tail elements that do not fill a full vector.

// ---

// ?ignore
// Step 3: Portable SIMD (requires nightly and #![feature(portable_simd)])
#![feature(portable_simd)]
use std::simd::Simd;

fn main() {
    let a = Simd::<f32, 4>::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = Simd::<f32, 4>::from_array([10.0, 20.0, 30.0, 40.0]);
    let result = a + b;
    println!("Portable SIMD result: {:?}", result);
}

// ! **3. Portable SIMD (`std::simd`)**
// ! - Lets you write portable vector code without tying yourself to SSE, AVX2, or NEON.
// ! - Familiar operators like `+` work directly on the `Simd<f32, 4>` vector type.
// ! - `std::simd` is still experimental and only available on nightly Rust.

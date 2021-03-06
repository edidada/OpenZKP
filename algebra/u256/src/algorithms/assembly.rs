use crate::U256;
use std::mem::MaybeUninit;

// TODO: Square asm

#[inline(always)]
pub fn full_mul_asm(x: &U256, y: &U256) -> (U256, U256) {
    const ZERO: u64 = 0;
    let x = x.as_limbs();
    let y = y.as_limbs();
    let mut lo = MaybeUninit::<[u64; 4]>::uninit();
    let mut hi = MaybeUninit::<[u64; 4]>::uninit();

    unsafe { asm!(r"
        xor %rax, %rax               // CF, OF cleared

        // Set x[0] * y
        // [lo[0] r8 r9 r10 r11]
        mov  0($2), %rdx             // x[0]
        mulx 0($3), %rax, %r8        // * y[0]
        mov  %rax, 0($0)             // Store lowest limb
        mulx 8($3), %rax, %r9        // * y[1]
        adcx %rax, %r8
        mulx 16($3), %rax, %r10      // * y[2]
        adcx %rax, %r9
        mulx 24($3), %rax, %r11      // * y[3]
        adcx %rax, %r10
        adcx $4, %r11                // No carry, CF cleared

        // Add x[1] * y
        // [lo[1] r9 r10 r11 r8]
        mov  8($2), %rdx             // x[1]
        mulx 0($3), %rax, %rbx       // * y[0]
        adcx %rax, %r8
        adox %rbx, %r9
        mov  %r8, 8($0)              // Store and free r8
        mulx 8($3), %rax, %rbx       // * y[1]
        adcx %rax, %r9
        adox %rbx, %r10
        mulx 16($3), %rax, %rbx      // * y[2]
        adcx %rax, %r10
        adox %rbx, %r11
        mulx 24($3), %rax, %r8       // * y[3]
        adcx %rax, %r11
        adox $4, %r8                 // No carry, OF cleared
        adcx $4, %r8                 // No carry, CF cleared

        // Add x[2] * y
        // [lo[2] r10 r11 r8 r9]
        mov  16($2), %rdx            // x[2]
        mulx 0($3), %rax, %rbx       // * y[0]
        adcx %rax, %r9
        adox %rbx, %r10
        mov  %r9, 16($0)             // Store and free r9
        mulx 8($3), %rax, %rbx       // * y[1]
        adcx %rax, %r10
        adox %rbx, %r11
        mulx 16($3), %rax, %rbx      // * y[2]
        adcx %rax, %r11
        adox %rbx, %r8
        mulx 24($3), %rax, %r9       // * y[3]
        adcx %rax, %r8
        adox $4, %r9                 // No carry, OF cleared
        adcx $4, %r9                 // No carry, CF cleared

        // Add x[3] * y
        // [lo[3] r11 r8 r9 r10]
        mov  24($2), %rdx            // x[3]
        mulx 0($3), %rax, %rbx       // * y[0]
        adcx %rax, %r10
        adox %rbx, %r11
        mov  %r10, 24($0)            // Store and free r9
        mulx 8($3), %rax, %rbx       // * y[1]
        adcx %rax, %r11
        adox %rbx, %r8
        mulx 16($3), %rax, %rbx      // * y[2]
        adcx %rax, %r8
        adox %rbx, %r9
        mulx 24($3), %rax, %r10      // * y[3]
        adcx %rax, %r9
        adox $4, %r10                // No carry, OF cleared
        adcx $4, %r10                // No carry, CF cleared

        // Store high limbs
        mov %r11, 0($1)
        mov %r8, 8($1)
        mov %r9, 16($1)
        mov %r10, 24($1)
        "
        :
        : "r"(lo.as_mut_ptr()), "r"(hi.as_mut_ptr()), "r"(x), "r"(y), "m"(ZERO)
        : "rax", "rbx", "rdx", "r8", "r9", "r10", "r11", "cc", "memory"
    )}
    let lo = unsafe { lo.assume_init() };
    let hi = unsafe { hi.assume_init() };

    (U256::from_limbs(lo), U256::from_limbs(hi))
}

#[inline(always)]
pub fn proth_redc_asm<M: Parameters>(lo: &U256, hi: &U256) -> U256 {
    const ZERO: u64 = 0;
    assert_eq!(M::MODULUS.as_limbs()[0], 1);
    assert_eq!(M::MODULUS.as_limbs()[1], 0);
    assert_eq!(M::MODULUS.as_limbs()[2], 0);
    assert_eq!(M::M64, u64::max_value());
    
    // TODO: Make modulus[3] a runtime variable (since this is low overhead)
    let modulus = M::MODULUS.as_limbs();
    let lo = lo.as_limbs();
    let hi = hi.as_limbs();
    let mut result = MaybeUninit::<[u64; 4]>::uninit();

    unsafe { asm!(r"
        // RDX contains M3 and we keep it there the whole time.
        // OPT: Use operand constraints to put it there.
        mov $4, %rdx

        // [r8, r9, r10, CF] = -[lo[0] lo[1] lo[2]]
        mov 0($1), %r8
        xor %r9, %r9
        xor %r10, %r10
        neg %r8
        sbb 8($1), %r9
        sbb 16($1), %r10
        // Remaining CF is for lo[3]

        // Clear OF (by adding zero+OF to zero)
        mov  $$0, %rax             // Note: we can't use xor here
        adox %rax, %rax

        // Add m3 * [k0 k1 k2] to [lo[3]+CF hi[0] hi[1] hi[2] hi[3]] and store in [r8 r11 r9 r10, r12]
        mulx %r8, %r8, %r11
        adcx 24($1), %r8
        adox 0($2), %r11
        mulx %r9, %rax, %r9
        adcx %rax, %r11
        adox 8($2), %r9
        mulx %r10, %rax, %r10
        adcx %rax, %r9
        adox 16($2), %r10
        adcx $3, %r10
        adox 24($2), %r12
        adcx $3, %r12

        // Compute k3, CF is for r11
        neg  %r8
        adcx $3, %r11
        adcx $3, %r9

        // Add m3 * k3 to [r10 r12]
        mulx %r8, %rax, %rbx
        adcx %rax, %r10
        adcx %rbx, %r12                    // No carry, CF = 0

        // Store result
        mov %r11, 0($0)
        mov %r9, 8($0)
        mov %r10, 16($0)
        mov %r12, 24($0)

        "
        :
        : "r"(result.as_mut_ptr()), "r"(lo), "r"(hi), "m"(ZERO), "m"(modulus[3])
        : "rax", "rbx", "rdx", "r8", "r9", "r10", "r11", "r12", "cc", "memory"
    )}
    let result = unsafe { result.assume_init() };

    // Final reduction
    let mut r = U256::from_limbs(result);
    if r >= M::MODULUS {
        r -= &M::MODULUS;
    }
    r
}

// https://doc.rust-lang.org/1.12.0/book/inline-assembly.html
// https://llvm.org/docs/LangRef.html#inline-assembler-expressions
// https://www.intel.com/content/dam/www/public/us/en/documents/white-papers/large-integer-squaring-ia-paper.pdf
// 

// LEA amd INC can add without affecting flags.
// NOT INC  can be used for a carry free NEG
// NEG sets CF and clobbers OF.

#[inline(always)]
pub fn mul_redc(a: &U256, b: &U256) -> U256 {
    const ZERO: u64 = 0; // $3
    const MODULUS_0: u64 = 1; // $4
    const MODULUS_1: u64 = 0; // $5
    const MODULUS_2: u64 = 0; // $6
    const MODULUS_3: u64 = 0x0800000000000011; // $7
    const M64: u64 = 0xffff_ffff_ffff_ffff; // -1 $8
                                            // TODO: Optimize for special primes where the above values hold
    let a = a.as_limbs();
    let b = b.as_limbs();
    let mut result: [u64; 4] = [0, 0, 0, 0];
    // MULX dst_high, dst_low, src_b (src_a = %rdx)
    // src_b can be register or memory, not immediate
    unsafe {
        asm!(r"
            // Assembly from Aztec's Barretenberg implementation, see 
            // <https://github.com/AztecProtocol/barretenberg/blob/master/src/barretenberg/fields/asm_macros.hpp>
            movq 0($1), %rdx
            xorq %r8, %r8
            mulxq 8($2), %r8, %r9
            mulxq 24($2), %rdi, %r12
            mulxq 0($2), %r13, %r14
            mulxq 16($2), %r15, %r10
            movq %r13, %rdx
            mulxq $8, %rdx, %r11
            adcxq %r8, %r14
            adoxq %rdi, %r10
            adcxq %r9, %r15
            adoxq $3, %r12
            adcxq $3, %r10
            mulxq $4, %r8, %r9
            mulxq $5, %rdi, %r11
            adoxq %r8, %r13
            adcxq %rdi, %r14
            adoxq %r9, %r14
            adcxq %r11, %r15
            mulxq $6, %r8, %r9
            mulxq $7, %rdi, %r11
            adoxq %r8, %r15
            adcxq %rdi, %r10
            adoxq %r9, %r10
            adcxq %r11, %r12
            adoxq $3, %r12
            movq 8($1), %rdx
            mulxq 0($2), %r8, %r9
            mulxq 8($2), %rdi, %r11
            adcxq %r8, %r14
            adoxq %r9, %r15
            adcxq %rdi, %r15
            adoxq %r11, %r10
            mulxq 16($2), %r8, %r9
            mulxq 24($2), %rdi, %r13
            adcxq %r8, %r10
            adoxq %rdi, %r12
            adcxq %r9, %r12
            adoxq $3, %r13
            adcxq $3, %r13
            movq %r14, %rdx
            mulxq $8, %rdx, %r8
            mulxq $4, %r8, %r9
            mulxq $5, %rdi, %r11
            adoxq %r8, %r14
            adcxq %rdi, %r15
            adoxq %r9, %r15
            adcxq %r11, %r10
            mulxq $6, %r8, %r9
            mulxq $7, %rdi, %r11
            adoxq %r8, %r10
            adcxq %r9, %r12
            adoxq %rdi, %r12
            adcxq %r11, %r13
            adoxq $3, %r13
            movq 16($1), %rdx
            mulxq 0($2), %r8, %r9
            mulxq 8($2), %rdi, %r11
            adcxq %r8, %r15
            adoxq %r9, %r10
            adcxq %rdi, %r10
            adoxq %r11, %r12
            mulxq 16($2), %r8, %r9
            mulxq 24($2), %rdi, %r14
            adcxq %r8, %r12
            adoxq %r9, %r13
            adcxq %rdi, %r13
            adoxq $3, %r14
            adcxq $3, %r14
            movq %r15, %rdx
            mulxq $8, %rdx, %r8
            mulxq $4, %r8, %r9
            mulxq $5, %rdi, %r11
            adoxq %r8, %r15
            adcxq %r9, %r10
            adoxq %rdi, %r10
            adcxq %r11, %r12
            mulxq $6, %r8, %r9
            mulxq $7, %rdi, %r11
            adoxq %r8, %r12
            adcxq %r9, %r13
            adoxq %rdi, %r13
            adcxq %r11, %r14
            adoxq $3, %r14
            movq 24($1), %rdx
            mulxq 0($2), %r8, %r9
            mulxq 8($2), %rdi, %r11
            adcxq %r8, %r10
            adoxq %r9, %r12
            adcxq %rdi, %r12
            adoxq %r11, %r13
            mulxq 16($2), %r8, %r9
            mulxq 24($2), %rdi, %r15
            adcxq %r8, %r13
            adoxq %r9, %r14
            adcxq %rdi, %r14
            adoxq $3, %r15
            adcxq $3, %r15
            movq %r10, %rdx
            mulxq $8, %rdx, %r8
            mulxq $4, %r8, %r9
            mulxq $5, %rdi, %r11
            adoxq %r8, %r10
            adcxq %r9, %r12
            adoxq %rdi, %r12
            adcxq %r11, %r13
            mulxq $6, %r8, %r9
            mulxq $7, %rdi, %rdx
            adoxq %r8, %r13
            adcxq %r9, %r14
            adoxq %rdi, %r14
            adcxq %rdx, %r15
            adoxq $3, %r15

            movq %r12, 0($0)
            movq %r13, 8($0)
            movq %r14, 16($0)
            movq %r15, 24($0)
            "
            :
            : "r"(&result), "r"(a), "r"(b),
              "m"(ZERO), "m"(MODULUS_0), "m"(MODULUS_1), "m"(MODULUS_2), "m"(MODULUS_3), "m"(M64)
            : "rdx", "rdi", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15", "cc", "memory"
        );
    }
    U256::from_limbs(result)
}

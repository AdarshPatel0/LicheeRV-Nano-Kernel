#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trap_handler_entry() {
    core::arch::naked_asm!(
        r#"
        addi    sp,     sp,     -528

        sd      ra,     0*8(sp)
        sd      sp,     1*8(sp)
        sd      gp,     2*8(sp)
        sd      tp,     3*8(sp)

        sd      t0,     4*8(sp)
        sd      t1,     5*8(sp)
        sd      t2,     6*8(sp)
        sd      t3,     7*8(sp)
        sd      t4,     8*8(sp)
        sd      t5,     9*8(sp)
        sd      t6,     10*8(sp)

        sd      a0,     11*8(sp)
        sd      a1,     12*8(sp)
        sd      a2,     13*8(sp)
        sd      a3,     14*8(sp)
        sd      a4,     15*8(sp)
        sd      a5,     16*8(sp)
        sd      a6,     17*8(sp)
        sd      a7,     18*8(sp)

        sd      s0,     19*8(sp)
        sd      s1,     20*8(sp)
        sd      s2,     21*8(sp)
        sd      s3,     22*8(sp)
        sd      s4,     23*8(sp)
        sd      s5,     24*8(sp)
        sd      s6,     25*8(sp)
        sd      s7,     26*8(sp)
        sd      s8,     27*8(sp)
        sd      s9,     28*8(sp)
        sd      s10,    29*8(sp)
        sd      s11,    30*8(sp)

        fsd     f0,     31*8(sp)
        fsd     f1,     32*8(sp)
        fsd     f2,     33*8(sp)
        fsd     f3,     34*8(sp)
        fsd     f4,     35*8(sp)
        fsd     f5,     36*8(sp)
        fsd     f6,     37*8(sp)
        fsd     f7,     38*8(sp)
        fsd     f8,     39*8(sp)
        fsd     f9,     40*8(sp)
        fsd     f10,    41*8(sp)
        fsd     f11,    42*8(sp)
        fsd     f12,    43*8(sp)
        fsd     f13,    44*8(sp)
        fsd     f14,    45*8(sp)
        fsd     f15,    46*8(sp)
        fsd     f16,    47*8(sp)
        fsd     f17,    48*8(sp)
        fsd     f18,    49*8(sp)
        fsd     f19,    50*8(sp)
        fsd     f20,    51*8(sp)
        fsd     f21,    52*8(sp)
        fsd     f22,    53*8(sp)
        fsd     f23,    54*8(sp)
        fsd     f24,    55*8(sp)
        fsd     f25,    56*8(sp)
        fsd     f26,    57*8(sp)
        fsd     f27,    58*8(sp)
        fsd     f28,    59*8(sp)
        fsd     f29,    60*8(sp)
        fsd     f30,    61*8(sp)
        fsd     f31,    62*8(sp)

        csrr    t0,     fcsr
        sd      t0,     63*8(sp)

        csrr    t0,     sstatus
        sd      t0,     64*8(sp)
        csrr    t0,     sepc
        sd      t0,     65*8(sp)

        csrr    t0,     sscratch
        sd      t0,     1*8(sp)

        addi    t0,     sp,     528
        sd      t0,     1*8(sp)

        call    trap_handler

        ld      t0,     63*8(sp)
        csrw    fcsr,   t0

        ld      t0,     64*8(sp)
        csrw    sstatus,    t0

        ld      t0,     65*8(sp)
        csrw    sepc,   t0

        ld      ra,     0*8(sp)
        ld      gp,     2*8(sp)
        ld      tp,     3*8(sp)

        ld      t0,     4*8(sp)
        ld      t1,     5*8(sp)
        ld      t2,     6*8(sp)
        ld      t3,     7*8(sp)
        ld      t4,     8*8(sp)
        ld      t5,     9*8(sp)
        ld      t6,     10*8(sp)

        ld      a0,     11*8(sp)
        ld      a1,     12*8(sp)
        ld      a2,     13*8(sp)
        ld      a3,     14*8(sp)
        ld      a4,     15*8(sp)
        ld      a5,     16*8(sp)
        ld      a6,     17*8(sp)
        ld      a7,     18*8(sp)

        ld      s0,     19*8(sp)
        ld      s1,     20*8(sp)
        ld      s2,     21*8(sp)
        ld      s3,     22*8(sp)
        ld      s4,     23*8(sp)
        ld      s5,     24*8(sp)
        ld      s6,     25*8(sp)
        ld      s7,     26*8(sp)
        ld      s8,     27*8(sp)
        ld      s9,     28*8(sp)
        ld      s10,    29*8(sp)
        ld      s11,    30*8(sp)

        fld     f0,     31*8(sp)
        fld     f1,     32*8(sp)
        fld     f2,     33*8(sp)
        fld     f3,     34*8(sp)
        fld     f4,     35*8(sp)
        fld     f5,     36*8(sp)
        fld     f6,     37*8(sp)
        fld     f7,     38*8(sp)
        fld     f8,     39*8(sp)
        fld     f9,     40*8(sp)
        fld     f10,    41*8(sp)
        fld     f11,    42*8(sp)
        fld     f12,    43*8(sp)
        fld     f13,    44*8(sp)
        fld     f14,    45*8(sp)
        fld     f15,    46*8(sp)
        fld     f16,    47*8(sp)
        fld     f17,    48*8(sp)
        fld     f18,    49*8(sp)
        fld     f19,    50*8(sp)
        fld     f20,    51*8(sp)
        fld     f21,    52*8(sp)
        fld     f22,    53*8(sp)
        fld     f23,    54*8(sp)
        fld     f24,    55*8(sp)
        fld     f25,    56*8(sp)
        fld     f26,    57*8(sp)
        fld     f27,    58*8(sp)
        fld     f28,    59*8(sp)
        fld     f29,    60*8(sp)
        fld     f30,    61*8(sp)
        fld     f31,    62*8(sp)

        ld      sp,     1*8(sp)

        sret
        "#
    );
}
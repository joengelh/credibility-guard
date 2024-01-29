; ModuleID = 'probe4.236231848f447740-cgu.0'
source_filename = "probe4.236231848f447740-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

@alloc_466c66c10691aa7db76070c7f38c3e28 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/6ae4cfbbb080cafea7f6be48ce47678ee057352c/library/core/src/num/mod.rs" }>, align 1
@alloc_69485a0c07b731b7bb78312842308ae1 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_466c66c10691aa7db76070c7f38c3e28, [12 x i8] c"K\00\00\00{\04\00\00\05\00\00\00" }>, align 4
@str.0 = internal unnamed_addr constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe45probe17h19fdf725b757a428E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h9448ffbbc2ebe27eE.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h904eec1a7ed2de8cE(ptr align 1 @str.0, i32 25, ptr align 4 @alloc_69485a0c07b731b7bb78312842308ae1) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h9448ffbbc2ebe27eE.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare hidden i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking5panic17h904eec1a7ed2de8cE(ptr align 1, i32, ptr align 4) unnamed_addr #2

attributes #0 = { nounwind "target-cpu"="mvp" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn nounwind "target-cpu"="mvp" }
attributes #3 = { noreturn nounwind }

!llvm.ident = !{!0}

!0 = !{!"rustc version 1.77.0-nightly (6ae4cfbbb 2024-01-17)"}

(module
 (type $none_=>_none (func))
 (type $none_=>_i32 (func (result i32)))
 (type $i32_i32_=>_i32 (func (param i32 i32) (result i32)))
 (type $i32_i32_i32_=>_i32 (func (param i32 i32 i32) (result i32)))
 (import "env" "emscripten_asm_const_int" (func $emscripten_asm_const_int (param i32 i32 i32) (result i32)))
 (memory $0 2)
 (data $.rodata (i32.const 568) "\00ii\00i\00")
 (data $em_asm (i32.const 574) "{ Module.print(\"Hello world\"); }\00{ return $0 + $1; }\00{ Module.print(\"Got \" + $0); }\00")
 (table $0 1 1 funcref)
 (global $__stack_pointer (mut i32) (i32.const 66208))
 (global $global$1 i32 (i32.const 574))
 (global $global$2 i32 (i32.const 658))
 (export "memory" (memory $0))
 (export "__wasm_call_ctors" (func $__wasm_call_ctors))
 (export "main" (func $main))
 (export "__start_em_asm" (global $global$1))
 (export "__stop_em_asm" (global $global$2))
 (func $__wasm_call_ctors
 )
 (func $__original_main (result i32)
  (local $0 i32)
  (global.set $__stack_pointer
   (local.tee $0
    (i32.sub
     (global.get $__stack_pointer)
     (i32.const 32)
    )
   )
  )
  (drop
   (call $emscripten_asm_const_int
    (i32.const 574)
    (i32.const 568)
    (i32.const 0)
   )
  )
  (i64.store offset=16
   (local.get $0)
   (i64.const 115964117005)
  )
  (i32.store
   (local.get $0)
   (call $emscripten_asm_const_int
    (i32.const 607)
    (i32.const 569)
    (i32.add
     (local.get $0)
     (i32.const 16)
    )
   )
  )
  (drop
   (call $emscripten_asm_const_int
    (i32.const 627)
    (i32.const 572)
    (local.get $0)
   )
  )
  (global.set $__stack_pointer
   (i32.add
    (local.get $0)
    (i32.const 32)
   )
  )
  (i32.const 0)
 )
 (func $main (param $0 i32) (param $1 i32) (result i32)
  (call $__original_main)
 )
 ;; custom section "producers", size 112
)


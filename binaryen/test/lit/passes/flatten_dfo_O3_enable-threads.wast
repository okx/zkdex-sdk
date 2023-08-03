;; NOTE: Assertions have been generated by update_lit_checks.py --all-items and should not be edited.
;; NOTE: This test was ported using port_test.py and could be cleaned up.

;; RUN: foreach %s %t wasm-opt --flatten --dfo -O3 --enable-threads -S -o - | filecheck %s

(module
 ;; CHECK:      (type $none_=>_none (func))

 ;; CHECK:      (type $i32_i32_=>_i32 (func (param i32 i32) (result i32)))

 ;; CHECK:      (type $i64_i32_=>_f64 (func (param i64 i32) (result f64)))

 ;; CHECK:      (type $f64_=>_f64 (func (param f64) (result f64)))

 ;; CHECK:      (type $none_=>_i32 (func (result i32)))

 ;; CHECK:      (memory $0 (shared 1 1))
 (memory $0 (shared 1 1))
 (func "one"
  (loop $label$2
   (br_if $label$2
    (block $label$3 (result i32)
     (drop
      (br_if $label$3
       (i32.const 0)
       (i32.load
        (i32.const 3060)
       )
      )
     )
     (i32.const 0)
    )
   )
  )
  (unreachable)
 )
 (func "two" (param $var$0 i32) (param $var$1 i32) (result i32)
  (nop)
  (nop)
  (nop)
  (nop)
  (nop)
  (if
   (i32.const 0)
   (i32.store8
    (i32.const 8)
    (block $label$2 (result i32)
     (drop
      (br_if $label$2
       (i32.const 1)
       (i32.const 0)
      )
     )
     (if
      (i32.const 0)
      (drop
       (br_if $label$2
        (i32.const 1)
        (i32.const 1)
       )
      )
     )
     (block $label$4
      (br_if $label$4
       (i32.const 0)
      )
      (br_if $label$4
       (i32.const 0)
      )
      (drop
       (br_if $label$2
        (i32.const 1)
        (i32.const 0)
       )
      )
     )
     (i32.const 6704)
    )
   )
  )
  (nop)
  (i32.const 0)
 )
 (func "use-var" (param $var$0 i64) (param $var$1 i32) (result f64)
  (local $var$2 i32)
  (block $label$1
   (br_table $label$1 $label$1 $label$1 $label$1 $label$1 $label$1 $label$1 $label$1 $label$1 $label$1
    (i32.wrap_i64
     (if (result i64)
      (i32.const 0)
      (i64.const 1)
      (if (result i64)
       (if (result i32)
        (i32.const 0)
        (unreachable)
        (block $label$6 (result i32)
         (block $label$7
          (loop $label$8
           (br_if $label$8
            (br_if $label$6
             (local.tee $var$2
              (block $label$9 (result i32)
               (local.get $var$1)
              )
             )
             (i32.const 0)
            )
           )
           (loop $label$10
            (if
             (i32.const 0)
             (local.set $var$2
              (local.get $var$1)
             )
            )
           )
           (drop
            (i32.eqz
             (local.get $var$2)
            )
           )
          )
         )
         (unreachable)
        )
       )
       (unreachable)
       (i64.const 1)
      )
     )
    )
   )
  )
  (unreachable)
 )
 (func "bad1"
  (local $var$2 i32)
  (local $var$4 i32)
  (block $label$1
   (loop $label$2
    (local.set $var$4
     (if (result i32)
      (i32.const 0)
      (block (result i32)
       (local.set $var$4
        (local.tee $var$2
         (i32.xor
          (i32.const 0)
          (i32.const -1)
         )
        )
       )
       (i32.const 0)
      )
      (block (result i32)
       (local.set $var$4
        (local.tee $var$2
         (i32.xor
          (i32.const 0)
          (i32.const -1)
         )
        )
       )
       (i32.const 0)
      )
     )
    )
    (i32.store
     (i32.const 1)
     (i32.shl
      (local.get $var$2)
      (i32.const 14)
     )
    )
   )
  )
 )
 (func "only-dfo" (param $var$0 f64) (result f64)
  (local $var$1 i32)
  (local $var$2 i32)
  (loop $label$1
   (if
    (local.tee $var$1
     (local.tee $var$2
      (local.get $var$1)
     )
    )
    (if
     (local.get $var$2)
     (i64.atomic.store32 offset=3
      (i32.and
       (local.get $var$1) ;; only dfo can figure out that this is 0
       (i32.const 15)
      )
      (i64.const -32768)
     )
    )
   )
   (br $label$1)
  )
 )
 (func "dfo-tee-get" (result i32)
  (local $0 i32)
  (if (result i32)
   (local.tee $0
    (i32.const 1)
   )
   (loop $label$2 (result i32)
    (select
     (i32.const 1)
     (i32.const -1709605511)
     (local.get $0)
    )
   )
   (unreachable)
  )
 )
)

;; CHECK:      (export "one" (func $0))

;; CHECK:      (export "two" (func $1))

;; CHECK:      (export "use-var" (func $2))

;; CHECK:      (export "bad1" (func $3))

;; CHECK:      (export "only-dfo" (func $4))

;; CHECK:      (export "dfo-tee-get" (func $5))

;; CHECK:      (func $0 (; has Stack IR ;)
;; CHECK-NEXT:  (block $label$3
;; CHECK-NEXT:   (br_if $label$3
;; CHECK-NEXT:    (i32.load
;; CHECK-NEXT:     (i32.const 3060)
;; CHECK-NEXT:    )
;; CHECK-NEXT:   )
;; CHECK-NEXT:  )
;; CHECK-NEXT:  (unreachable)
;; CHECK-NEXT: )

;; CHECK:      (func $1 (; has Stack IR ;) (param $0 i32) (param $1 i32) (result i32)
;; CHECK-NEXT:  (i32.const 0)
;; CHECK-NEXT: )

;; CHECK:      (func $2 (; has Stack IR ;) (param $0 i64) (param $1 i32) (result f64)
;; CHECK-NEXT:  (loop $label$8
;; CHECK-NEXT:   (br_if $label$8
;; CHECK-NEXT:    (local.get $1)
;; CHECK-NEXT:   )
;; CHECK-NEXT:  )
;; CHECK-NEXT:  (unreachable)
;; CHECK-NEXT: )

;; CHECK:      (func $3 (; has Stack IR ;)
;; CHECK-NEXT:  (i32.store
;; CHECK-NEXT:   (i32.const 1)
;; CHECK-NEXT:   (i32.const -16384)
;; CHECK-NEXT:  )
;; CHECK-NEXT: )

;; CHECK:      (func $4 (; has Stack IR ;) (param $0 f64) (result f64)
;; CHECK-NEXT:  (local $1 i32)
;; CHECK-NEXT:  (loop $label$1
;; CHECK-NEXT:   (br $label$1)
;; CHECK-NEXT:  )
;; CHECK-NEXT: )

;; CHECK:      (func $5 (; has Stack IR ;) (result i32)
;; CHECK-NEXT:  (i32.const 1)
;; CHECK-NEXT: )

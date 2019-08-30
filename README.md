# hyperdrive
```
bundle exec ruby benchmark/method_calls.rb 
Rehearsal ---------------------------------------------
vm method    0.631764   0.001012   0.632776 (  0.633556)
jit method   0.070169   0.000342   0.070511 (  0.070672)

------------------------------------ total: 2.597477sec

                user     system      total        real
vm method    0.625791   0.000688   0.626479 (  0.627456)
jit method   0.065605   0.000177   0.065782 (  0.065905)


Rehearsal ----------------------------------------------
vm cfunc     1.962284   0.002417   1.964701 (  1.967366)
jit cfunc    0.674312   0.000730   0.675042 (  0.676071)
------------------------------------- total: 0.745553sec

                 user     system      total        real
vm cfunc     1.954347   0.002023   1.956370 (  1.958966)
jit cfunc    0.660276   0.000754   0.661030 (  0.662081)
```

Hyperdrive is a tracing JIT for CRuby, heavily inspired by LuaJIT. Tracing is currently out of fashion but there are some major advantages for high level lanuages like Ruby, if you don't have the developer resources to build a fully optimizing function/method JIT like C2, V8, JavaScriptCore

* Traces naturally specialise based on the types recorded
* Control flow inside a trace is essentially linear so a single simple IR can be used to apply optimizations.
* Inlining happens automatically

Current optimizations are:
* Type specialisation - no branching on object type
* Elimination of stack operations - better use of registers
* Method lookup is eliminated by reading from the bytecode inline cache - lots of indirection is compiled to a simple call

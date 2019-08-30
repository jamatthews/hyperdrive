require 'benchmark'
$LOAD_PATH.unshift File.expand_path("../../lib", __FILE__)

def add_one(x)
  x + 1
end

def method_call
  i = 0
  while i < 30_000_000
    i = add_one(i)
  end
end

def cfunc_call
  i = 0
  x = []
  while i < 30_000_000
    i += 1
    x = [2,1].reverse
  end
end

Benchmark.bmbm do |x|
  x.report("vm method") { method_call }
  x.report("vm cfunc") { cfunc_call }
end

require 'hyperdrive'

Benchmark.bmbm do |x|
  x.report("jit method") { method_call }
  x.report("jit cfunc") { cfunc_call }
end

private

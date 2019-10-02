require 'benchmark'
$LOAD_PATH.unshift File.expand_path("../../lib", __FILE__)


def while_loop
  x = []
  i = 0
  while i < 300_000_000
    x[0] = i
    i += 1
  end
end

Benchmark.bmbm do |x|
  x.report("vm") { while_loop }
end

require 'hyperdrive'

Benchmark.bmbm do |x|
  x.report("jit") { while_loop }
end

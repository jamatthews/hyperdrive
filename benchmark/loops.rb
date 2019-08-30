require 'benchmark'
$LOAD_PATH.unshift File.expand_path("../../lib", __FILE__)


def while_loop
  i = 0
  while i < 30_000_000
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

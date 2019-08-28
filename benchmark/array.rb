require 'benchmark'
$LOAD_PATH.unshift File.expand_path("../../lib", __FILE__)


def loop
  i = 0
  x = []
  while i < 30_000_000
    i += 1
    x = [2,1].reverse
  end
end

Benchmark.bmbm do |x|
  x.report("vm") { loop }
end

require 'hyperdrive'

Benchmark.bmbm do |x|
  x.report("jit") { loop }
end

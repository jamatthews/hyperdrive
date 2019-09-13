require 'benchmark'
$LOAD_PATH.unshift File.expand_path("../../lib", __FILE__)

class String
  def blank?
    empty? || !(/[[:^space:]]/ === self)
  end
end

def blank_strings
  i = 0
  while i < 30_000_000
    ''.blank?
    i = i + 1
  end
end

def not_blank_strings
  i = 0
  while i < 30_000_000
    'not_blank'.blank?
    i = i + 1
  end
end

Benchmark.bmbm do |x|
  x.report("vm blank fast path") { blank_strings }
  x.report("vm blank fast path") { not_blank_strings }
end

require 'hyperdrive'

Benchmark.bmbm do |x|
  x.report("jit blank fast patch") { blank_strings }
  x.report("jit blank slow path") { not_blank_strings }
end

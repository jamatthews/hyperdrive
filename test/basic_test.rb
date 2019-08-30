require "test_helper"

class BasicTest < Minitest::Test
  def test_basic_trace
    trace_count = Hyperdrive.trace_count
    i = 0
    while i < 1002
      i += 1
    end
    assert i, 1002
    assert_equal Hyperdrive.trace_count, trace_count + 1
  end

  def test_cfunc_call
    trace_count = Hyperdrive.trace_count
    i = 0
    x = [2,1]
    while i < 1002
      i += 1
      x = [2,1].reverse
    end
    assert x = [1,2]
    assert_equal Hyperdrive.trace_count, trace_count + 1
  end

  def test_method_call
    trace_count = Hyperdrive.trace_count
    i = 0
    while i < 1002
      i = add_one(i)
    end
    assert i = 1002
    assert_equal Hyperdrive.trace_count, trace_count + 1
  end

  private
  def add_one(x)
    x + 1
  end
end

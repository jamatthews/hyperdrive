require "test_helper"

class ArrayTest < Minitest::Test
  def test_append
    trace_count = Hyperdrive.trace_count
    x = []
    i = 0
    while i < 2000
      x << 'element'
      i = i + 1
    end
    assert_equal 2000, i
    assert_equal 2000, x.size
    assert_equal 'element', x.last
    assert_equal Hyperdrive.trace_count, trace_count + 1
  end

  def test_new
    trace_count = Hyperdrive.trace_count
    a = 'element'
    x = []
    i = 0
    while i < 2000
      x = [a]
      i = i + 1
    end
    assert_equal 2000, i
    assert_equal ['element'], x
    assert_equal Hyperdrive.trace_count, trace_count + 1
  end

  def test_reference
    trace_count = Hyperdrive.trace_count
    a = [1,2,3]
    x = nil
    i = 0
    while i < 2000
      x = a[2]
      i = i + 1
    end
    assert_equal 2000, i
    assert_equal 3, x
    assert_equal Hyperdrive.trace_count, trace_count + 1
  end
end

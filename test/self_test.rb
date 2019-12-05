require "test_helper"

class SelfTest < Minitest::Test
  Array.class_eval do
    def a_method
      self[1]
    end
  end

  def test_self
    trace_count = Hyperdrive.trace_count
    test = [1,2,3]
    x = 0
    i = 0
    while i < 2000
      x = test.a_method
      i = i + 1
    end
    assert_equal 2000, i
    assert_equal 2, x
    assert_equal Hyperdrive.trace_count, trace_count + 1
  end
end

require "test_helper"

class SideExitTest < Minitest::Test
  def test_side_exit
    trace_count = Hyperdrive.trace_count
    x = 0
    i = 0
    while i < 2000
      x = if i < 1999
        1
      else
        2
      end
      i = i + 1
    end
    assert_equal 1999, i
    assert_equal 2, x
    assert_equal Hyperdrive.trace_count, trace_count + 1
  end
end

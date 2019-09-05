require "test_helper"

class SideExitTest < Minitest::Test
  def test_side_exit
    trace_count = Hyperdrive.trace_count
    i = 0
    while i < 2000
      break if i == 1999
      i = i + 1
    end
    assert_equal 1999, i
    assert_equal Hyperdrive.trace_count, trace_count + 1
  end
end

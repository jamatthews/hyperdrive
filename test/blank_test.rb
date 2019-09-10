require "test_helper"

class BlankTest < Minitest::Test
  def test_string_blank
    trace_count = Hyperdrive.trace_count
    x = true
    i = 0
    while i < 1002
      x = ' '.empty?
      i = i + 1
    end
    assert_equal x, false
    assert_equal Hyperdrive.trace_count, trace_count + 1
  end
end

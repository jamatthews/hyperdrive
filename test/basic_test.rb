require "test_helper"

class BasicTest < Minitest::Test
  def test_trace_recorded
    assert_equal 2000, while_loop
    assert_equal 1, Hyperdrive.trace_count
  end

  private

  def while_loop
    i = 0
    while i < 2000
      i += 1
    end
    i
  end
end

require "test_helper"

class BasicTest < Minitest::Test
  def test_trace_recorded
    while_loop
    assert_equal 1, Hyperdrive.trace_count
  end

  private

  def while_loop
    i = 0
    while i < 1002
      i += 1
    end
  end
end

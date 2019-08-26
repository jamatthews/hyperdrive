require "test_helper"

class BasicTest < Minitest::Test
  def test_trace_recorded
    assert_equal 1002, while_loop
  end

  private

  def while_loop
    i = 0
    while i < 1002
      i += 1
    end
    i
  end
end

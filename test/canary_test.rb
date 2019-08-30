require "test_helper"

class BasicTest < Minitest::Test
  def test_direct_call
    assert_equal [1,2], loop
  end

  private

  def loop
    i = 0
    x = []
    while i < 1002
      i += 1
      x = [2,1].reverse
    end
    x
  end
end

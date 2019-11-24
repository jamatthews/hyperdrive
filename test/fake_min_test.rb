require "test_helper"

class Array
  def min
    ret = self[0]
    n = 1
    while n < self.size
      ret = self[n] if self[n] < ret
      n = n + 1
    end
    ret
  end
end

class FakeMinTest < Minitest::Test
  # def test_size
  #   trace_count = Hyperdrive.trace_count
  #   x = 2
  #   i = 0
  #   while i < 2000
  #     x = [2,1].size
  #     i = i + 1
  #   end
  #   assert_equal trace_count + 1, Hyperdrive.trace_count
  #   assert_equal 2000, i
  #   assert_equal 1, x
  # end
end

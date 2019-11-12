require "test_helper"

class String
  def blank?
    empty? || !(/[[:^space:]]/ === self)
  end
end

class BlankTest < Minitest::Test
  # def test_string_blank
  #   trace_count = Hyperdrive.trace_count
  #   x = false
  #   i = 0
  #   while i < 2000
  #     x = ' '.blank?
  #     i = i + 1
  #   end
  #   assert_equal 2000, i
  #   assert_equal true, x
  #   assert_equal trace_count + 1, Hyperdrive.trace_count
  # end
end

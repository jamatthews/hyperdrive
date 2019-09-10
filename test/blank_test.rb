require "test_helper"

class String
  def blank?
    empty? || !(/[[:^space:]]/ === self)
  end
end

class BlankTest < Minitest::Test
  def test_string_blank
    trace_count = Hyperdrive.trace_count
    x = true
    i = 0
    while i < 1002
      x = ''.blank?
      i = i + 1
    end
    assert_equal x, true
    assert_equal Hyperdrive.trace_count, trace_count + 1
  end
end

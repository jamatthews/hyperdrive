require "test_helper"

class HashTest < Minitest::Test
  def test_insert
    trace_count = Hyperdrive.trace_count
    x = {}
    i = 0
    while i < 2000
      key = 'key'
      value = 'value'
      x[key] = value
      i = i + 1
    end
    assert_equal 2000, i
    assert_equal({ 'key' => 'value' }, x)
    assert_equal Hyperdrive.trace_count, trace_count + 1
  end

  def test_new
    trace_count = Hyperdrive.trace_count
    x = { }
    i = 0
    while i < 2000
      key = 'key'
      value = 'value'
      x = { key => value }
      i = i + 1
    end
    assert_equal 2000, i
    assert_equal({ 'key' => 'value' }, x)
    assert_equal Hyperdrive.trace_count, trace_count + 1
  end
end

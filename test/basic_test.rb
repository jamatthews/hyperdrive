require "test_helper"

class BasicTest < Minitest::Test
  def test_dump_trace
    Hyperdrive.begin_trace
    1 + 1
    Hyperdrive.stop_recording
    Hyperdrive.dump_trace
  end
end

require "test_helper"

class BasicTest < Minitest::Test
  def test_direct_call
    #puts RubyVM::InstructionSequence.of(:minimum_loop).disasm
    assert_equal [1,2], minimum_loop
  end

  private

  def minimum_loop
    i = 0
    x = []
    while i < 1002
      i += 1
      x = [2,1].reverse
    end
    x
  end
end

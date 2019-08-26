require "test_helper"

class BasicTest < Minitest::Test
  def test_constant_folded
    #puts RubyVM::InstructionSequence.of(:minimum_loop).disasm
    assert_equal [1], minimum_loop
  end

  private

  def minimum_loop
    i = 0
    z = [1]
    while i < 1002
      i += 1
    end
    z
  end
end

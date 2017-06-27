module Dice
  ModifiedRoll = Struct.new(:roll, :modifier) do
    def scalar
      roll.scalar + (modifier || 0)
    end

    def vector
      roll.vector
    end
  end
end

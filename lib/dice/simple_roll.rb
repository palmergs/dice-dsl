module Dice
  SimpleRoll = Struct.new(:range, :count) do
    def scalar
      vector.inject(&:+)
    end

    def vector
      n = (count || 1)
      (1..n).map { rand(range) + 1 }
    end
  end
end

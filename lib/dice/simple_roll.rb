module Dice
  SimpleRoll = Struct.new(:range, :count) do
    def scalar
      vector.inject(&:+)
    end

    def vector
      n = actual_count
      (1..n).map { rand(range) + 1 }
    end

    def actual_count
      count || 1
    end

    def to_s
      "#{ actual_count }d#{ range }"
    end
  end
end

module Dice
  ExplodeEachRoll = Struct.new(:roll) do
    def scalar
      vector.inject(&:+)
    end

    def vector
      vector_with_range.map(&:first)
    end

    def vector_with_range
      arr = roll.vector_with_range
      arr.map do |pair|
        if pair[0] == pair[1]
          tmp = [roll.roll, roll.range]
          pair[0] += tmp[0]
          while tmp[0] == tmp[1]
            tmp = [roll.roll, roll.range]
            pair[0] += tmp[0]
          end
          pair
        else
          pair
        end
      end
    end

    def to_s
      "#{ roll }!!"
    end
  end
end

module Dice
  ExplodingRoll = Struct.new(:roll) do
    def scalar
      vector.inject(&:+)
    end

    def vector
      vector_with_range.map(&:first)      
    end

    def vector_with_range
      arr = roll.vector_with_range
      return arr unless arr.all? {|pair| pair[0] == pair[1]}

      explodes = [ roll.roll_one.to_a ]
      while explodes.last[0] == explodes.last[1]
        explodes << roll.roll_one.to_a
      end
      arr + explodes
    end

    def to_s
      "#{ roll }!"
    end
  end
end
    

require 'dice'

module Dice
  TakeN = Struct.new(:roll, :count, :invert) do
    def scalar
      vector.inject(&:+)
    end

    def vector
      if count
        if invert
          roll.vector.sort.first(count)
        else
          roll.vector.sort.last(count)
        end
      else
        roll.vector
      end
    end

    def vector_with_range
      roll.vector_with_range
    end

    def to_s
      if count
        if invert
          "#{ roll.to_s}`#{ count }"
        else
          "#{ roll.to_s }^#{ count }"
        end
      else
        roll.to_s
      end
    end
  end
end

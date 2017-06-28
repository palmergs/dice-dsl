module Dice
  ModifyEachRoll = Struct.new(:roll, :modifier) do
    def scalar
      vector.inject(&:+)
    end

    def vector
      vector_with_range.map(&:first)
    end

    def vector_with_range
      arr = roll.vector_with_range
      arr.map do |pair|
        [ pair[0], pair[1] + actual_modifier ]
      end
    end

    def actual_modifier
      modifier || 0
    end

    def to_s
      if actual_modifier > 0
        "#{ roll }++#{ actual_modifier }"
      elsif actual_modifier < 0
        "#{ roll }-#{ actual_modifier }"
      else
        roll.to_s
      end
    end
  end
end

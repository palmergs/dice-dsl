module Dice
  ModifiedRoll = Struct.new(:roll, :modifier) do
    def scalar
      roll.scalar + actual_modifier
    end

    def vector
      roll.vector
    end

    def actual_modifier
      modifier || 0
    end

    def to_s
      if actual_modifier > 0
        "#{ roll }+#{ actual_modifier }"
      elsif actual_modifier < 0
        "#{ roll }#{ actual_modifier }"
      else
        roll.to_s
      end
    end
  end
end

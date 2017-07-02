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
        [ pair[0] + actual_modifier, pair[1] ]
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

    def self.parse scanner
      roll = Dice::SimpleRoll.parse(scanner)
      if roll
        if arr = scanner.scan(Dice::Token::PLUS_EACH, Dice::Token::INTEGER)
          Dice::ModifyEachRoll.new(roll, arr[1])
        else
          roll
        end
      else
        nil
      end
    end
  end
end

module Dice
  ModifiedRoll = Struct.new(:roll, :modifier) do

    def roll!
      roll.roll!
    end

    def scalar
      vector.inject(&:+)
    end

    def vector
      vector_with_range.map(&:first)
    end

    def vector_with_range
      if modifier
        roll.vector_with_range << [ modifier, 0 ]
      else
        roll.vector_with_range
      end
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

    def self.parse scanner
      roll = Dice::SimpleRoll.parse(scanner.mark)
      if roll
        if mods = scanner.scan(Dice::Parser::Token::PLUS, Dice::Parser::Token::INTEGER)
          return ModifiedRoll.new(roll, mods[1])
        elsif mods = scanner.scan(Dice::Parser::Token::MINUS, Dice::Parser::Token::INTEGER)
          return ModifiedRoll.new(roll, -1 * mods[1])
        end
      end

      scanner.reset
      nil
    end
  end
end

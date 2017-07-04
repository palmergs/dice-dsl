module Dice
  ModifiedRoll = Struct.new(:roll, :modifier) do

    def roll!
      roll.roll!
    end

    def results
      if modifier
        roll.results + [ Dice::Result.new(modifier: modifier) ]
      else
        roll.results
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

  ModifiedRoll.include(Dice::HasValues)
end

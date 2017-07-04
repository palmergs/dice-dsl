module Dice
  ModifyEachRoll = Struct.new(:roll, :modifier) do

    def roll!
      roll.roll!
    end

    def results
      if modifier
        roll.results.each {|r| r.modifier = actual_modifier }
      end
      roll.results
    end

    def scalar
      vector.inject(&:+)
    end

    def vector
      results.map(&:modified_value)
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
      roll = Dice::SimpleRoll.parse(scanner.mark)
      if roll
        if arr = scanner.scan(Dice::Parser::Token::PLUS_EACH, Dice::Parser::Token::INTEGER)
          return Dice::ModifyEachRoll.new(roll, arr[1])
        end
      end

      scanner.reset
      nil
    end
  end
end

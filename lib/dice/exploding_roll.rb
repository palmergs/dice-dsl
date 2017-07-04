module Dice
  ExplodingRoll = Struct.new(:roll) do

    def roll!
      roll.roll!
    end

    def results
      arr = []
      if roll.results.all?(&:max?)
        tmp = roll.roll_one
        arr << tmp
        while tmp.max?
          tmp = roll.roll_one
          arr << tmp
        end
      end
      roll.results + arr
    end

    def to_s
      "#{ roll }!"
    end

    def self.parse scanner
      roll = Dice::SimpleRoll.parse(scanner.mark)
      if roll
        if mods = scanner.scan(Dice::Parser::Token::EXPLODE_ONCE)
          return ExplodingRoll.new(roll)
        end
      end

      scanner.reset
      nil
    end
  end

  ExplodingRoll.include(Dice::HasValues)
end
    

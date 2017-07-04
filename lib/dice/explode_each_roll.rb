module Dice
  ExplodeEachRoll = Struct.new(:roll) do

    def roll!
      roll.roll!
    end

    def results
      roll.results.map do |r|
        if r.max?
          tmp = roll.roll_one
          r.value += tmp.value
          while tmp.max?
            tmp = roll.roll_one
            r.value += tmp.value
          end
        end

        r
      end
    end

    def to_s
      "#{ roll }!!"
    end

    def self.parse scanner
      roll = Dice::SimpleRoll.parse(scanner.mark)
      if roll
        if arr = scanner.scan(Dice::Parser::Token::EXPLODE_MANY)
          return Dice::ExplodeEachRoll.new(roll)
        end
      end

      scanner.reset
      nil
    end
  end

  ExplodeEachRoll.include(Dice::HasValues)
end

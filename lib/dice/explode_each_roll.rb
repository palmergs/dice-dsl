module Dice
  ExplodeEachRoll = Struct.new(:roll) do

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
      arr = roll.vector_with_range
      arr.map do |pair|
        if pair[0] == pair[1]
          tmp = roll.roll_one.to_a
          pair[0] += tmp[0]
          while tmp[0] == tmp[1]
            tmp = roll.roll_one.to_a
            pair[0] += tmp[0]
          end
          pair
        else
          pair
        end
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
end

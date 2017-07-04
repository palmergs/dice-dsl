require 'dice'

module Dice
  TakeN = Struct.new(:roll, :count, :group) do

    HIGHEST_N = :highest
    LOWEST_N = :lowest
    MIDDLE_N = :middle
    OP = { HIGHEST_N => '^', LOWEST_N => '`', MIDDLE_N => '~' }

    def roll!
      roll.roll!
    end

    def results
      if count
        arr = roll.results.sort {|lhs,rhs| lhs.modified_value <=> rhs.modified_value }
        case actual_group
          when HIGHEST_N
            arr.last(count)
          when LOWEST_N
            arr.first(count)
          when MIDDLE_N
            middle = arr.length / 2
            half = count / 2.0
            range = [0, middle - half.floor].max ... [arr.length, middle + half.ceil].min
            arr[range]
        end
      else
        roll.results
      end
    end

    def actual_group
      group || Dice::HIGHEST_N
    end

    def to_s
      if count
        "#{ roll.to_s }#{ OP[actual_group] }#{ count }"
      else
        roll.to_s
      end
    end

    def self.parse scanner
      if vector_roll = Dice::SimpleRoll.parse(scanner.mark)
        if arr = scanner.scan(Dice::Parser::Token::HIGHEST_N, Dice::Parser::Token::INTEGER)
          return Dice::TakeN.new(vector_roll, arr[1], HIGHEST_N)
        elsif arr = scanner.scan(Dice::Parser::Token::LOWEST_N, Dice::Parser::Token::INTEGER)
          return Dice::TakeN.new(vector_roll, arr[1], LOWEST_N)
        elsif arr = scanner.scan(Dice::Parser::Token::MIDDLE_N, Dice::Parser::Token::INTEGER)
          return Dice::TakeN.new(vector_roll, arr[1], MIDDLE_N)
        end
      end

      scanner.reset
      nil
    end
  end

  TakeN.include(Dice::HasValues)
end

require 'dice'

module Dice
  TakeN = Struct.new(:roll, :count, :group) do

    HIGHEST_N = :highest
    LOWEST_N = :lowest
    MIDDLE_N = :middle

    def scalar
      vector.inject(&:+)
    end

    def vector
      if count
        case actual_group
          when HIGHEST_N
            roll.vector.sort.last(count)
          when LOWEST_N
            roll.vector.sort.first(count)
          when MIDDLE_N
            middle = roll.vector.length / 2
            half = count / 2.0
            range = [0, middle - half.floor].max ... [roll.vector.length, middle + half.ceil].min
            roll.vector.sort[range]
        end
      else
        roll.vector
      end
    end

    def vector_with_range
      roll.vector_with_range
    end

    def actual_group
      group || Dice::HIGHEST_N
    end

    def to_s
      if count
        case actual_group
          when HIGHEST_N
            "#{ roll.to_s }^#{ count }"
          when LOWEST_N
            "#{ roll.to_s }`#{ count }"
          when MIDDLE_N
            "#{ roll.to_s }~#{ count }"
        end
      else
        roll.to_s
      end
    end

    def self.parse scanner
      if vector_roll = Dice::VectorRoll.parse(scanner)
        if arr = scanner.scan(Dice::Parser::Token::HIGHEST_N, Dice::Parser::Token::INTEGER)
          Dice::TakeN.new(vector_roll, arr[1], HIGHEST_N)
        elsif arr = scanner.scan(Dice::Parser::Token::LOWEST_N, Dice::Parser::Token::INTEGER)
          Dice::TakeN.new(vector_roll, arr[1], LOWEST_N)
        elsif arr = scanner.scan(Dice::Parser::Token::MIDDLE_N, Dice::Parser::Token::INTEGER)
          Dice::TakeN.new(vector_roll, arr[1], MIDDLE_N)
        end
      else
        nil
      end
    end
  end
end

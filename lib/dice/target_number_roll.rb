module Dice
  TargetNumberRoll = Struct.new(:roll, :target, :invert) do
    def scalar
      vector.inject(&:+)
    end

    def vector
      vector_with_range.map(&:first)
    end

    def vector_with_range
      roll.vector_with_range.map do |pair|
        if invert
          pair[0] <= actual_target ? [ 1, 1 ] : [ 0, 1 ]
        else
          pair[0] >= actual_target ? [ 1, 1 ] : [ 0, 1 ]
        end
      end
    end

    def actual_target
      target || 0
    end

    def to_s
      if invert
        "#{ roll.to_s }<#{ actual_target }>"
      else
        "#{ roll.to_s }[#{ actual_target }]"
      end
    end

    def self.parse scanner
      if list = Dice::RollList.parse(scanner)
        if list.is_a?(Dice::Parser::RollList)
          if arr = scanner.scan(Dice::Parser::Token::GREATER_THAN, Dice::Parser::Token::INTEGER)
            # (2d4, 3d6, 1d8, 2d10) > 4
            TargetNumberRoll.new(list, arr[1])
          elsif arr = scanner.scan(Dice::Parser::Token::LESS_THAN, Dice::Parser::Token::INTEGER)
            # (2d4, 3d6, 1d8, 2d10) < 4
            TargetNumberRoll.new(list, arr[1], true)
          else
            list
          end
        else
          if arr = scanner.scan(Dice::Parser::Token::LEFT_BRACKET, Dice::Parser::Token::INTEGER, Dice::Parser::Token::RIGHT_BRACKET)
            # 3d6[4]
            TargetNumberRoll.new(list, arr[1])
          elsif arr = scanner.scan(Dice::Parser::Token::LESS_THAN, Dice::Parser::Token::INTEGER, Dice::Parser::Token::GREATER_THAN)
            # 3d6<4>
            TargetNumberRoll.new(list, arr[1], true)
          else
            list
          end
        end
      else
        nil
      end
    end
  end
end

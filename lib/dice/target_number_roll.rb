module Dice
  TargetNumberRoll = Struct.new(:roll, :target, :invert) do

    def roll!
      roll.roll!
    end

    def results
      roll.results.map do |r|
        if invert
          Dice::Result.new(value: r.modified_value <= actual_target ? 1 : 0, range: 1)
        else
          Dice::Result.new(value: r.modified_value >= actual_target ? 1 : 0, range: 1)
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

    def self.parse_list scanner
      if list = Dice::RollList.parse(scanner.mark)
        if arr = scanner.scan(Dice::Parser::Token::GREATER_THAN, Dice::Parser::Token::INTEGER)
          # (2d4, 3d6, 1d8, 2d10) > 4
          return TargetNumberRoll.new(list, arr[1])
        elsif arr = scanner.scan(Dice::Parser::Token::LESS_THAN, Dice::Parser::Token::INTEGER)
          # (2d4, 3d6, 1d8, 2d10) < 4
          return TargetNumberRoll.new(list, arr[1], true)
        end
      end

      scanner.reset
      nil
    end

    def self.parse scanner
      if roll = Dice::SimpleRoll.parse(scanner.mark)
        if arr = scanner.scan(Dice::Parser::Token::LEFT_BRACKET, Dice::Parser::Token::INTEGER, Dice::Parser::Token::RIGHT_BRACKET)
          # 3d6[4]
          return TargetNumberRoll.new(roll, arr[1])
        elsif arr = scanner.scan(Dice::Parser::Token::LESS_THAN, Dice::Parser::Token::INTEGER, Dice::Parser::Token::GREATER_THAN)
          # 3d6<4>
          return TargetNumberRoll.new(roll, arr[1], true)
        end
      end

      scanner.reset
      nil
    end
  end

  TargetNumberRoll.include(Dice::HasValues)
end

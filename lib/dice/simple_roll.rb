module Dice
  SimpleRoll = Struct.new(:range, :count) do

    def roll_one
      Dice::Result.new(value: rand(range) + 1, range: range)
    end

    def roll!
      @last_roll = actual_count.times.collect { roll_one }
    end

    def results
      @last_roll ||= roll!
    end

    def actual_count
      count || 1
    end

    def to_s
      "#{ actual_count }d#{ range }"
    end

    def self.parse scanner
      if arr = scanner.scan(Dice::Parser::Token::INTEGER, Dice::Parser::Token::ROLL, Dice::Parser::Token::INTEGER)
        Dice::SimpleRoll.new(arr[2], arr[0])
      elsif arr = scanner.scan(Dice::Parser::Token::ROLL, Dice::Parser::Token::INTEGER)
        Dice::SimpleRoll.new(arr[1])
      else
        nil
      end
    end
  end

  SimpleRoll.include(Dice::HasValues)
end

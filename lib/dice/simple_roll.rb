module Dice
  SimpleRoll = Struct.new(:range, :count) do

    def roll_one
      Dice::Result.new(rand(range) + 1, range)
    end

    def roll!
      @last_roll = actual_count.times.collect { roll_one }
    end

    def roll?
      !@last_roll.nil?
    end

    def roll
      @last_roll ||= roll!
    end

    def scalar
      vector.inject(&:+)
    end

    def vector
      roll.map(&:value)
    end

    def vector_with_range
      roll.map do |r| 
        [ r.value, r.range ] 
      end
    end

    def actual_count
      count || 1
    end

    def to_s
      "#{ actual_count }d#{ range }"
    end
  end
end

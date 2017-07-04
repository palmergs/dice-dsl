module Dice
  class Result
    attr_accessor :value, :range, :modifier
    attr_reader :exploded
    def initialize value: 0, range: 0, modifier: 0
      @value, @range, @modifier = value, range, modifier
      @exploded = []
    end

    def max?
      value == range
    end

    def to_a
      [ modified_value, range ]
    end

    def modified_value
      value + modifier + exploded_value
    end

    def exploded_value
      exploded.size > 0 ? exploded.map(&:modified_value).inject(&:+) : 0
    end

    def die_to_s
      range > 0 ? "1d#{ range }" : ''
    end

    def modifier_to_s
      if modifier && modifier != 0
        if modifier > 0
          "+#{ modifier }"
        else
          "#{ modifier }"
        end
      else
        ''
      end
    end

    def to_s
      arr = [ "#{ modified_value }" ]
      arr << ' ('
      arr << die_to_s
      arr << modifier_to_s
      exploded.each do |r|
        arr << ' + '
        arr << r.to_s
      end
      arr << ')'
      arr.join
    end
  end
end

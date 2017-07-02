module Dice
  class RollList
    def initialize *args
      @list = []
      args.each do |v|
        if v.is_a?(Array)
          @list += v
        else
          @list << v
        end
      end
    end

    def << roll
      @list << roll
    end

    def scalar
      vector.inject(&:+)
    end

    def vector
      vector_with_range.map(&:first)
    end

    def vector_with_range
      @list.map(&:vector_with_range).inject(&:+)
    end

    def to_s
      @list.map(&:to_s).join(', ')
    end
  end
end

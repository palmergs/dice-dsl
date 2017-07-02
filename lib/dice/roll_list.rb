module Dice
  class RollList
    attr_reader :list

    def initialize *args
      @list = []
      args.each do |v|
        if v.is_a?(Dice::RollList)
          @list += v.list
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

    def self.parse scanner
      if scanner.scan(Dice::Token::LEFT_PAREN)
        list = parse(scanner)
        raise "Unmatched parenthesis" unless scanner.scan(Dice::Token::RIGHT_PARAM)
        list
      elsif vector = Dice::VectorRoll.parse(scanner)
        if scanner.scan(Dice::Token::COMMA)
          RollList.new(vector, RollList.parse(scanner))
        else
          vector
        end
      end
    end
  end
end

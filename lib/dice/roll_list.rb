module Dice
  class RollList
    attr_reader :list

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

    def roll!
      @list.map(&:roll!)
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
      if scanner.scan(Dice::Parser::Token::LEFT_PAREN)
        list = parse(scanner)
        raise "Unmatched parenthesis" unless scanner.scan(Dice::Parser::Token::RIGHT_PARAM)
        list
      elsif roll = Dice::VectorRoll.parse(scanner)
        items = [ roll ]
        while scanner.scan(Dice::Parser::Token::COMMA)
          if roll = Dice::VectorRoll.parse(scanner)
            pp roll
            items << roll
          else
            break
          end
        end
        RollList.new(items)
      else
        nil
      end
    end
  end
end

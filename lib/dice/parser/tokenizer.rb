module Dice
  module Parser
    class Tokenizer

      def tokenize str
        arr = tokenize_to_strings(str)
        strings_to_tokens(arr)
      end

      SPLIT = /[\n\t\s0-9dD]/

      def tokenize_to_strings str
        acc = []
        tokens = []
        number = false
        whitespace = false
        str.each_char do |c|
          if c =~ /[\s\t\n]/
            unless whitespace
              tokens << acc.join unless acc.empty?
              acc.clear
              whitespace = true
              number = false
            end
          elsif c =~ /\d/
            unless number
              tokens << acc.join unless acc.empty?
              acc.clear
              number = true
              whitespace = false
            end
            acc << c
          elsif c =~ /[dD]/
            tokens << acc.join unless acc.empty?
            acc.clear
            number = false
            whitespace = false
            tokens << 'd'
          elsif c =~ /,/
            tokens << acc.join unless acc.empty?
            acc.clear
            number = false
            whitespace = false
            tokens << ','
          else
            if number
              tokens << acc.join
              acc.clear
              number = false
            end
            whitespace = false
            acc << c
          end
        end
        tokens << acc.join unless acc.empty?
        tokens
      end

      def strings_to_tokens arr
        arr.map do |str|
          case str
            when /\d+/
              if str == '00'
                100
              elsif str == '000'
                1000
              else
                Integer(str)
              end
            when '%'
              100
            when '%%'
              1000
            else
              Dice::Parser::Token::TOKENS[str] or raise "Unrecognized token '#{ str }'"
          end
        end
      end
    end
  end
end

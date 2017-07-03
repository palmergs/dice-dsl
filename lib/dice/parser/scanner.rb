class Dice::Parser::Scanner

  attr_reader :tokens

  def initialize str
    @tokens = Dice::Parser::Tokenizer.new.tokenize(str)
    @index = -1
    @mark = []
  end

  def pos
    @index
  end

  def pos= n
    @index = n
  end

  def mark
    @mark.push(pos)
    self
  end

  def reset
    self.pos= @mark.pop
    self
  end

  def scan *pattern
    start_idx = @index
    pattern.map do |token|
      if match?(token, next_token)
        peek
      else
        @index = start_idx
        return nil
      end
    end
  end

  def next_token
    @index += 1
    peek
  end

  def match? token, scanned_token
    (token == Dice::Parser::Token::INTEGER && scanned_token.is_a?(Integer)) || (token == scanned_token)
  end

  def peek
    if @index >= 0 && @index < tokens.length
      tokens[@index]
    else
      nil
    end
  end
end

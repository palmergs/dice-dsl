module Dice
  class Token
    ROLL = 'd'
    COMMA = ','
    OR = '|'
    AND = '&'
    PLUS = '+'
    MINUS = '-'
    PLUS_EACH = '++'
    MINUS_EACH = '--'
    PERCENT = '%'
    PERMILLE = '%%'

    LEFT_PAREN = '('
    RIGHT_PAREN = ')'
    LEFT_BRACKET = '['
    RIGHT_BRACKET = ']'
    LEFT_BRACE = '{'
    RIGHT_BRACE = ']'

    SUCCESS_LESS = '<<'
    SUCCESS_GREATER = '>>'

    SUCCESS_LESS_EACH = '<'
    SUCCESS_GREATER_EACH = '>'

    EXPLODE_ONCE = '!'
    EXPLODE_MANY = '!!'

    EXPLODE_EACH_ONCE = '*'
    EXPLODE_EACH_MANY = '**'

    HIGHEST_N = '^'
    LOWEST_N = '`'
    MIDDLE_N = '~'

    TOKENS = {
      ROLL => ROLL,
      'D' => ROLL,
      COMMA => COMMA,
      OR => OR,
      AND => AND,
      PLUS => PLUS,
      MINUS => MINUS,
      PLUS_EACH => PLUS_EACH,
      MINUS_EACH => MINUS_EACH,
      PERCENT => PERCENT,
      PERMILLE => PERMILLE,
      LEFT_PAREN => LEFT_PAREN,
      RIGHT_PAREN => RIGHT_PAREN,
      LEFT_BRACKET => LEFT_BRACKET,
      RIGHT_BRACKET => RIGHT_BRACKET,
      LEFT_BRACE => LEFT_BRACE,
      RIGHT_BRACE => RIGHT_BRACE,
      SUCCESS_LESS_EACH => SUCCESS_LESS_EACH,
      SUCCESS_GREATER_EACH => SUCCESS_GREATER_EACH,
      SUCCESS_LESS => SUCCESS_LESS,
      SUCCESS_GREATER => SUCCESS_GREATER,
      EXPLODE_ONCE => EXPLODE_ONCE,
      EXPLODE_MANY => EXPLODE_MANY,
      EXPLODE_EACH_ONCE => EXPLODE_EACH_ONCE,
      EXPLODE_EACH_MANY => EXPLODE_EACH_MANY,
      HIGHEST_N => HIGHEST_N,
      LOWEST_N => LOWEST_N,
      MIDDLE_N => MIDDLE_N
    }.freeze
  end
end

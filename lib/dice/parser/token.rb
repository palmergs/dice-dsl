module Dice
  module Parser
    class Token

      INTEGER = 'integer'

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

      SCALAR_LESS_THAN = '<<'
      SCALAR_GREATER_THAN = '>>'

      LESS_THAN = '<'
      GREATER_THAN = '>'

      EXPLODE_ONCE = '!'
      EXPLODE_MANY = '!!'

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

          SCALAR_LESS_THAN => SCALAR_LESS_THAN,
          SCALAR_GREATER_THAN => SCALAR_GREATER_THAN,

          LESS_THAN => LESS_THAN,
          GREATER_THAN => GREATER_THAN,

          EXPLODE_ONCE => EXPLODE_ONCE,
          EXPLODE_MANY => EXPLODE_MANY,

          HIGHEST_N => HIGHEST_N,
          LOWEST_N => LOWEST_N,
          MIDDLE_N => MIDDLE_N
      }.freeze
    end
  end
end


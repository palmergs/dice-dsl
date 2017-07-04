module Dice
  module HasValues
    def scalar
      vector.inject(&:+)
    end

    def vector
      results.map(&:modified_value)
    end
  end
end
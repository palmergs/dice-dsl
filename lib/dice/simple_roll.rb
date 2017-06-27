module Dice
  class SimpleRoll
    attr_accessor :count, :range
    
    def scalar
      vector.inject(&:+)
    end

    def vector
      actual_count = count || 1
      (1..actual_count).map do |_|
        rand(range) + 1  
      end
    end
  end
end

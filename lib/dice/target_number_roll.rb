module Dice
  TargetNumberRoll = Struct.new(:roll, :target, :invert) do
    def scalar
      vector.inject(&:+)
    end

    def vector
      roll.vector.map do |v|
        if invert
          v <= actual_target ? 1 : 0
        else
          v >= actual_target ? 1 : 0    
        end
      end
    end

    def vector_with_range
      roll.vector_with_range
    end

    def actual_target
      target || 0
    end
  end
end

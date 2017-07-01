module Dice
  Roll = Struct.new(:value, :range, :modifier) do
    def modifier?
      return false unless modifier
      modifier != 0
    end

    def bonus?
      modifier? && modifier > 0
    end

    def penalty?
      modifier? && modifier < 0
    end

    def modifier_to_s
      return '' unless modifier?
      return "+#{ modifier }" if bonus?
      return "#{ modifier }" if penalty?
    end

    def to_s
      "#{ value } (1d#{ range }#{ modifier_to_s })"
    end

    def to_a
      [ value, range, modifier ? 0 : modifier ]
    end
  end
end

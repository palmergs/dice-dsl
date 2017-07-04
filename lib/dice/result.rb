module Dice
  Result = Struct.new(:value, :range, :modifier) do
    def to_s
      if modifier
        if modifier > 0
          "#{ value } (1d#{ range }+#{ modifier })"
        else
          "#{ value } (1d#{ range }#{ modifier })"
        end
      else
        "#{ value } (1d#{ range })"
      end
    end

    def max?
      value == range
    end

    def to_a
      [ modified_value, range ]
    end

    def modified_value
      value + actual_modifier
    end

    def actual_modifier
      modifier || 0
    end
  end
end

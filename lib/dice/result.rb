module Dice
  Result = Struct.new(:value, :range) do
    def to_s
      "#{ value } (1d#{ range })"
    end

    def to_a
      [ value, range ]
    end
  end
end

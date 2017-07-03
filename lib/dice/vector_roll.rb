module Dice

  # parser for rolls that can be interpreted as a vector
  class VectorRoll
    def self.parse scanner
      if modified_each = Dice::ModifyEachRoll.parse(scanner)
        modified_each
      elsif modified = Dice::ModifiedRoll.parse(scanner)
        modified
      elsif exploding_each = Dice::ExplodeEachRoll.parse(scanner)
        exploding_each
      elsif exploding = Dice::ExplodingRoll.parse(scanner)
        exploding
      elsif taken = Dice::TakeN.parse(scanner)
        taken
      elsif simple = Dice::SimpleRoll.parse(scanner)
        simple
      else
        nil
      end
    end
  end
end
module Dice

  # parser for rolls that can be interpreted as a vector
  class VectorRoll
    def self.parse scanner
      if simple = Dice::SimpleRoll.parse(scanner)
        simple
      elsif modified = Dice::ModifiedRoll.parse(scanner)
        modified
      elsif modified_each = Dice::ModifiedEachRoll.parse(scanner)
        modified_each
      elsif exploding = Dice::ExplodingRoll.parse(scanner)
        exploding
      elsif exploding_each = Dice::ExplodingEachRoll.parse(scanner)
        exploding_each
      else
        nil
      end
    end
  end
end
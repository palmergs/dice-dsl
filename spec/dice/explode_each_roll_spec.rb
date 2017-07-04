require 'spec_helper'
require 'dice'

RSpec.describe Dice::ExplodeEachRoll do
  it 'can generate a vector' do
    eer = Dice::ExplodeEachRoll.new(Dice::SimpleRoll.new(4, 6))
    expect(eer.vector.size).to eq(6)
    expect(eer.scalar).to be >= 6
    expect(eer.to_s).to eq('6d4!!')
  end

  describe '#parse' do
    it 'can parse a exploding die roll' do
      scanner = Dice::Parser::Scanner.new("4d4!!")
      eer = Dice::ExplodeEachRoll.parse(scanner)
      expect(eer.to_s).to eq('4d4!!')
    end
  end
end

require 'spec_helper'
require 'dice/simple_roll'
require 'dice/modified_roll'

RSpec.describe Dice::ModifiedRoll do
  it 'can generate a scalar' do
    mr = Dice::ModifiedRoll.new(Dice::SimpleRoll.new(8))
    expect(mr.vector.size).to eq(1)
    expect(mr.scalar).to be >= 1
    expect(mr.scalar).to be <= 8
    expect(mr.to_s).to eq('1d8')
  end

  it 'can generate a modified scalar' do
    mr = Dice::ModifiedRoll.new(Dice::SimpleRoll.new(8, 2), 4)
    expect(mr.vector.size).to eq(3)
    expect(mr.scalar).to be >= 6
    expect(mr.scalar).to be <= 20
    expect(mr.to_s).to eq('2d8+4')
  end

  it 'can generate a modified scalar' do
    mr = Dice::ModifiedRoll.new(Dice::SimpleRoll.new(8, 2), -2)
    expect(mr.vector.size).to eq(3)
    expect(mr.scalar).to be >= 0
    expect(mr.scalar).to be <= 14
    expect(mr.to_s).to eq('2d8-2')
  end

  describe '#parse' do
    it 'can parse a modified roll' do
      scanner = Dice::Parser::Scanner.new("3d6+2")
      mr = Dice::ModifiedRoll.parse(scanner)
      expect(mr.to_s).to eq('3d6+2')
    end
  end
end

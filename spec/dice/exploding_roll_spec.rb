require 'spec_helper'
require 'dice/exploding_roll'

RSpec.describe Dice::ExplodingRoll do
  it 'can generate a scalar' do
    er = Dice::ExplodingRoll.new(Dice::SimpleRoll.new(3,2))
    expect(er.vector.size).to be >= 2
    expect(er.scalar).to be >= 2
    expect(er.to_s).to eq('2d3!')
  end

  it 'can generate scalar of a single value' do
    er = Dice::ExplodingRoll.new(Dice::SimpleRoll.new(4))
    expect(er.results.size).to be >= 1
    expect(er.results.all? {|r| r.is_a?(Dice::Result) } ).to eq(true)
    expect(er.vector.size).to be >= 1
    expect(er.to_s).to eq('1d4!')
  end

  describe '#parse' do
    it 'can parse exploding roll' do
      scanner = Dice::Parser::Scanner.new("2d4!")
      er = Dice::ExplodingRoll.parse(scanner)
      expect(er.to_s).to eq('2d4!')
    end
  end
end

require 'spec_helper'
require 'dice'

RSpec.describe Dice::TakeN do
  it 'can generate a vector of the top N values' do
    tn = Dice::TakeN.new(Dice::SimpleRoll.new(6, 4), 3)
    expect(tn.vector.size).to eq(3)
    expect(tn.scalar).to be >= 3
    expect(tn.scalar).to be <= 18
    expect(tn.to_s).to eq("4d6^3")
  end

  it 'can generate a vector of the bottom N values' do
    tn = Dice::TakeN.new(Dice::SimpleRoll.new(6, 4), 3, Dice::LOWEST_N)
    expect(tn.vector.size).to eq(3)
    expect(tn.scalar).to be >= 3
    expect(tn.scalar).to be <= 18
    expect(tn.to_s).to eq("4d6`3")
  end

  it 'can generate a vector of the middle N values' do
    (1..10).each do |take|
      (1...10).each do |n|
        tn = Dice::TakeN.new(Dice::SimpleRoll.new(6, n), take, Dice::MIDDLE_N)
        expect(tn.vector.size).to eq([take, n].min)
      end
    end
  end

  describe '#parse' do
    it 'can parse a take n' do
      scanner = Dice::Parser::Scanner.new("4d6^3")
      tn = Dice::TakeN.parse(scanner)
      expect(tn.to_s).to eq('4d6^3')
      expect(tn.vector.length).to eq(3)
      expect(tn.scalar).to be >= 3
      expect(tn.scalar).to be <= 18
    end

    it 'can parse a take n (lower)' do
      scanner = Dice::Parser::Scanner.new("4d6`3")
      tn = Dice::TakeN.parse(scanner)
      expect(tn.to_s).to eq('4d6`3')
      expect(tn.vector.length).to eq(3)
      expect(tn.scalar).to be >= 3
      expect(tn.scalar).to be <= 18
    end

    it 'can parse a middle n' do
      scanner = Dice::Parser::Scanner.new("5d6~3")
      tn = Dice::TakeN.parse(scanner)
      expect(tn.to_s).to eq('5d6~3')
      expect(tn.vector.length).to eq(3)
      expect(tn.scalar).to be >= 3
      expect(tn.scalar).to be <= 18
    end
  end
end

require 'spec_helper'
require 'dice/simple_roll'
require 'dice/parser'

RSpec.describe Dice::SimpleRoll do

  it 'persists until rerolls' do
    sr = Dice::SimpleRoll.new(10, 100)
    sr.roll!
    value = sr.scalar
    expect(value).to eq(sr.scalar)

    sr.roll!
    expect(value).to_not eq(sr.scalar)
  end

  it 'has a default count of 1' do
    sr = Dice::SimpleRoll.new
    sr.range = 10
    expect(sr.vector.size).to eq(1)
    expect(sr.scalar).to be >= 1
    expect(sr.scalar).to be <= 10
    expect(sr.to_s).to eq('1d10')
  end

  it 'can generate a long vector' do
    sr = Dice::SimpleRoll.new
    sr.range = 6
    sr.count = 3
    expect(sr.vector.size).to eq(3)
    expect(sr.scalar).to be >= 3
    expect(sr.scalar).to be <= 18
    expect(sr.to_s).to eq('3d6')
  end

  describe '#parse' do

    it 'can parse a simple roll' do
      scanner = Dice::Parser::Scanner.new('d20')
      sr = Dice::SimpleRoll.parse(scanner)
      expect(sr.range).to eq(20)
      expect(sr.vector.length).to eq(1)
      expect(sr.to_s).to eq('1d20')
    end

    it 'can parse a simple roll with %' do
      scanner = Dice::Parser::Scanner.new('d%')
      sr = Dice::SimpleRoll.parse(scanner)
      expect(sr.range).to eq(100)
      expect(sr.to_s).to eq('1d100')
    end

    it 'can parse a simple roll with %%' do
      scanner = Dice::Parser::Scanner.new('d%%')
      sr = Dice::SimpleRoll.parse(scanner)
      expect(sr.range).to eq(1000)
      expect(sr.to_s).to eq('1d1000')
    end

    it 'can parse a simple roll with count' do
      scanner = Dice::Parser::Scanner.new('3d8')
      sr = Dice::SimpleRoll.parse(scanner)
      expect(sr.range).to eq(8)
      expect(sr.vector.length).to eq(3)
      expect(sr.to_s).to eq('3d8')
    end

  end
end

require 'spec_helper'
require 'dice/simple_roll'
require 'dice/modified_roll'

RSpec.describe Dice::ModifiedRoll do
  it 'can generate a scalar' do
    mr = Dice::ModifiedRoll.new(Dice::SimpleRoll.new(8))
    expect(mr.vector.size).to eq(1)
    expect(mr.scalar).to be >= 1
    expect(mr.scalar).to be <= 8
  end

  it 'can generate a modified scalar' do
    mr = Dice::ModifiedRoll.new(Dice::SimpleRoll.new(8, 2), 4)
    expect(mr.vector.size).to eq(2)
    expect(mr.scalar).to be >= 6
    expect(mr.scalar).to be <= 20
  end

  it 'can generate a modified scalar' do
    mr = Dice::ModifiedRoll.new(Dice::SimpleRoll.new(8, 2), -2)
    expect(mr.vector.size).to eq(2)
    expect(mr.scalar).to be >= 0
    expect(mr.scalar).to be <= 14
  end
end

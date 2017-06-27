require 'spec_helper'
require 'dice/simple_roll'

RSpec.describe Dice::SimpleRoll do
  it 'has a default count of 1' do
    sr = Dice::SimpleRoll.new
    sr.range = 10
    expect(sr.vector.size).to eq(1)
    expect(sr.scalar).to be >= 1
    expect(sr.scalar).to be <= 10
  end

  it 'can generate a long vector' do
    sr = Dice::SimpleRoll.new
    sr.range = 6
    sr.count = 3
    expect(sr.vector.size).to eq(3)
    expect(sr.scalar).to be >= 3
    expect(sr.scalar).to be <= 18
  end
end

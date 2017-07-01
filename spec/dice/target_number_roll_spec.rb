require 'spec_helper'
require 'dice'

RSpec.describe Dice::TargetNumberRoll do
  it 'can generate a vector of successes' do
    tnr = Dice::TargetNumberRoll.new(Dice::SimpleRoll.new(6, 6), 4)
    expect(tnr.vector.all? {|n| n == 1 || n == 0 }).to be_truthy
    expect(tnr.scalar).to be >= 0
    expect(tnr.scalar).to be <= 6
    expect(tnr.to_s).to eq("6d6[4]")
  end

  it 'can invert a vector of successes' do
    tnr = Dice::TargetNumberRoll.new(Dice::SimpleRoll.new(6, 6), 4, true)
    expect(tnr.vector.all? {|n| n == 1 || n == 0 }).to be_truthy
    expect(tnr.scalar).to be >= 0
    expect(tnr.scalar).to be <= 6
    expect(tnr.to_s).to eq("6d6<4>")
  end
end

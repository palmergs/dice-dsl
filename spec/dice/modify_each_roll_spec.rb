require 'spec_helper'
require 'dice'

RSpec.describe Dice::ModifyEachRoll do
  it 'can generate a scalar for a positive modifier' do
    mer = Dice::ModifyEachRoll.new(Dice::SimpleRoll.new(8, 4), 3)
    expect(mer.vector.length).to eq(4)
    expect(mer.scalar).to be >= 7
    expect(mer.scalar).to be <= 44
    expect(mer.to_s).to eq('4d8++3')
  end

  it 'can generate a scalar for a negative modifier' do
    mer = Dice::ModifyEachRoll.new(Dice::SimpleRoll.new(6, 4), -2)
    expect(mer.vector.length).to eq(4)
    expect(mer.scalar).to be <= 16
    expect(mer.scalar).to be >= -4
    expect(mer.to_s).to eq('4d6--2')
  end
end

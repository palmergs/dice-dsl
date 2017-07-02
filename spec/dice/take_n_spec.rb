require 'spec_helper'
require 'dice'

RSpec.describe Dice::TakeN do
  it 'can generate a vector of the top N values' do
    tn = Dice::TakeN.new(Dice::SimpleRoll.new(6, 4), 3)
    expect(tn.vector_with_range.size).to eq(4)
    expect(tn.vector.size).to eq(3)
    expect(tn.scalar).to be >= 3
    expect(tn.scalar).to be <= 18
    expect(tn.to_s).to eq("4d6^3")
  end

  it 'can generate a vector of the bottom N values' do
    tn = Dice::TakeN.new(Dice::SimpleRoll.new(6, 4), 3, true)
    expect(tn.vector_with_range.size).to eq(4)
    expect(tn.vector.size).to eq(3)
    expect(tn.scalar).to be >= 3
    expect(tn.scalar).to be <= 18
    expect(tn.to_s).to eq("4d6`3")
  end
end
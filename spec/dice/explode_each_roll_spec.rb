require 'spec_helper'
require 'dice'

RSpec.describe Dice::ExplodeEachRoll do
  it 'can generate a vector' do
    eer = Dice::ExplodeEachRoll.new(Dice::SimpleRoll.new(4, 6))
    expect(eer.vector_with_range.size).to eq(6)
    expect(eer.scalar).to be >= 6
    expect(eer.to_s).to eq('6d4!!')
  end
end

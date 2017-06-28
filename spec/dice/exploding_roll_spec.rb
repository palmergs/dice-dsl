require 'spec_helper'
require 'dice/exploding_roll'

RSpec.describe Dice::ExplodingRoll do
  it 'can generate a scalar' do
    er = Dice::ExplodingRoll.new(Dice::SimpleRoll.new(4,2))
    expect(er.vector.size).to be >= 2
    expect(er.scalar).to be >= 2
    expect(er.to_s).to eq('2d4!')
  end
end

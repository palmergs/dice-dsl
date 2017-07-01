require 'spec_helper'
require 'dice'

RSpec.describe Dice::Roll do
  describe '.to_s' do
    it 'renders to string' do
      r = Dice::Roll.new(4, 6)
      expect(r.to_s).to eq("4 (1d6)")
    end

    it 'can be converted to an array' do
      r = Dice::Roll.new(3, 4)
      expect(r.to_a).to eq([3, 4])
    end
  end
end

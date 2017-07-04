require 'spec_helper'
require 'dice'

RSpec.describe Dice::Result do
  describe '.to_s' do
    it 'renders to string' do
      r = Dice::Result.new(value: 4, range: 6)
      expect(r.to_s).to eq("4 (1d6)")
    end

    it 'can be converted to an array' do
      r = Dice::Result.new(value: 3, range: 4)
      expect(r.to_a).to eq([3, 4])
    end

    it 'can render exploded values' do
      r = Dice::Result.new(value: 4, range: 6)
      r.exploded << Dice::Result.new(value: 6, range: 6)
      r.exploded << Dice::Result.new(value: 4, range: 6)
      expect(r.to_s).to eq('14 (1d6 + 6 (1d6) + 4 (1d6))')
    end
  end
end

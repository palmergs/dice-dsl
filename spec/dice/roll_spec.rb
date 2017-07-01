require 'spec_helper'
require 'dice'

RSpec.describe Dice::Roll do
  describe '.to_s' do
    it 'renders without modifier' do
      r = Dice::Roll.new(4, 6)
      expect(r.to_s).to eq("4 (1d6)")
    end

    it 'renders with positive modifier' do
      r = Dice::Roll.new(4, 6, 2)
      expect(r.to_s).to eq("4 (1d6+2)")
    end

    it 'renders with negative modifier' do
      r = Dice::Roll.new(7, 20, -2)
      expect(r.to_s).to eq("7 (1d20-2)")
    end
  end
end

require 'spec_helper'

RSpec.describe Dice::Histogram do
  it 'can generate an array of values' do
    h = Dice::Histogram.new("4d6^3")
    h.iterations = 10_000
    arr = h.generate
    puts h.linear_chart
    expect(arr.length).to eq(19)
    expect(arr[0]).to be_nil
    expect(arr[1]).to be_nil
    expect(arr[2]).to be_nil
  end
end

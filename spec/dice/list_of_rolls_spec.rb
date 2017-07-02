require 'spec_helper'
require 'pp'
require 'dice'

RSpec.describe Dice::ListOfRolls do

  it 'can store multiple rolls' do
    lor = Dice::ListOfRolls.new
    lor << Dice::SimpleRoll.new(4, 2)
    lor << Dice::SimpleRoll.new(6, 3)
    lor << Dice::SimpleRoll.new(8, 2)
    lor << Dice::SimpleRoll.new(10)
    lor << Dice::SimpleRoll.new(12)
    expect(lor.vector.length).to eq(9)
    expect(lor.scalar).to be >= 9
    expect(lor.to_s).to eq("2d4, 3d6, 2d8, 1d10, 1d12")
  end

  it 'can store multiple advanced rolls' do
    lor = Dice::ListOfRolls.new
    lor << Dice::ModifyEachRoll.new(Dice::SimpleRoll.new(4, 2), 2)
    lor << Dice::ExplodeEachRoll.new(Dice::SimpleRoll.new(6, 6))
    expect(lor.to_s).to eq("2d4++2, 6d6!!")
  end

  it 'can be converted to a target number roll' do
    lor = Dice::ListOfRolls.new
    lor << Dice::ExplodeEachRoll.new(Dice::SimpleRoll.new(4, 2))
    lor << Dice::ExplodeEachRoll.new(Dice::SimpleRoll.new(6, 2))
    lor << Dice::ExplodeEachRoll.new(Dice::SimpleRoll.new(8, 2))
    lor << Dice::ExplodeEachRoll.new(Dice::SimpleRoll.new(10, 2))
    expect(lor.to_s).to eq("2d4!!, 2d6!!, 2d8!!, 2d10!!")
    expect(lor.vector.length).to eq(8)

    tnr = Dice::TargetNumberRoll.new(lor, 4)
    vector = tnr.vector
    expect(vector.length).to eq(8)
    expect(vector).to eq(tnr.vector)
    expect(tnr.scalar).to be >= 0
    expect(tnr.scalar).to be <= 8
    expect(tnr.vector.all? {|n| n == 0 || n == 1 }).to be_truthy
  end
end
require 'spec_helper'

RSpec.describe Dice::VectorRoll do

  it 'should parse different roll types' do
    scanner = Dice::Parser::Scanner.new("2d4")
    expect(Dice::VectorRoll.parse(scanner)).to be_a Dice::SimpleRoll

    scanner = Dice::Parser::Scanner.new("2d6++3")
    expect(Dice::VectorRoll.parse(scanner)).to be_a Dice::ModifyEachRoll

    scanner = Dice::Parser::Scanner.new("2d4!!")
    expect(Dice::VectorRoll.parse(scanner)).to be_a Dice::ExplodeEachRoll
  end
end
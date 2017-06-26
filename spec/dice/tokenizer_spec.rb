require 'spec_helper'

RSpec.describe Dice::Tokenizer do

  let(:tokenizer) { Dice::Tokenizer.new }

  it 'can parse a dice string into tokens' do
    expect(tokenizer.tokenize("2d12")).to eq [2, Dice::Token::ROLL, 12]
  end
  it 'can parse percentile' do
    expect(tokenizer.tokenize("d00")).to eq [Dice::Token::ROLL, 100]
    expect(tokenizer.tokenize("d100")).to eq [Dice::Token::ROLL, 100]
    expect(tokenizer.tokenize("d%")).to eq [Dice::Token::ROLL, 100]

    expect(tokenizer.tokenize("d000")).to eq [Dice::Token::ROLL, 1000]
    expect(tokenizer.tokenize("d1000")).to eq [Dice::Token::ROLL, 1000]
    expect(tokenizer.tokenize("d%%")).to eq [Dice::Token::ROLL, 1000]
  end
  it 'can parse arithmetic' do
    expect(tokenizer.tokenize('3d4+4')).to eq [3, Dice::Token::ROLL, 4, Dice::Token::PLUS, 4]
    expect(tokenizer.tokenize('10d12-2')).to eq [10, Dice::Token::ROLL, 12, Dice::Token::MINUS, 2]
  end
end

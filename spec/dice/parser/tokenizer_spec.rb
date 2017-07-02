require 'spec_helper'
require 'dice/parser'

RSpec.describe Dice::Parser::Tokenizer do

  let(:tokenizer) { Dice::Parser::Tokenizer.new }

  it 'raises an exception on an unrecognized token' do
    expect { tokenizer.tokenize("2d6 = 4") }.to raise_error "Unrecognized token '='"
  end

  it 'can parse a dice string into tokens' do
    expect(tokenizer.tokenize("2d12")).to eq [2, Dice::Parser::Token::ROLL, 12]
    expect(tokenizer.tokenize("2 d 12")).to eq [2, Dice::Parser::Token::ROLL, 12]
  end

  it 'can parse percentile' do
    expect(tokenizer.tokenize("d00")).to eq [Dice::Parser::Token::ROLL, 100]
    expect(tokenizer.tokenize("d100")).to eq [Dice::Parser::Token::ROLL, 100]
    expect(tokenizer.tokenize("d%")).to eq [Dice::Parser::Token::ROLL, 100]

    expect(tokenizer.tokenize("d000")).to eq [Dice::Parser::Token::ROLL, 1000]
    expect(tokenizer.tokenize("d1000")).to eq [Dice::Parser::Token::ROLL, 1000]
    expect(tokenizer.tokenize("d%%")).to eq [Dice::Parser::Token::ROLL, 1000]
  end

  it 'can parse arithmetic' do
    expect(tokenizer.tokenize('3d4+4')).to eq [3, Dice::Parser::Token::ROLL, 4, Dice::Parser::Token::PLUS, 4]
    expect(tokenizer.tokenize('10d12-2')).to eq [10,
        Dice::Parser::Token::ROLL,
        12,
        Dice::Parser::Token::MINUS,
        2 ]
  end

  it 'can parse lists' do
    expect(tokenizer.tokenize('1d4,2d4')).to eq [1,
        Dice::Parser::Token::ROLL,
        4,
        Dice::Parser::Token::COMMA,
        2,
        Dice::Parser::Token::ROLL,
        4 ]
  end

  it 'can tokenize modifiers' do
    expect(tokenizer.tokenize('1d3+2')).to eq [1, Dice::Parser::Token::ROLL, 3, Dice::Parser::Token::PLUS, 2]
    expect(tokenizer.tokenize('d8 - 2')).to eq [Dice::Parser::Token::ROLL, 8, Dice::Parser::Token::MINUS, 2]
  end
end

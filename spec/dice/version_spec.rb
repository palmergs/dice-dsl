require "spec_helper"

RSpec.describe Dice::VERSION do
  it "has a version number" do
    expect(Dice::VERSION).to match /^\d+\.\d+\.\d+(?:-[a-z0-9_]+)?$/i
  end
end

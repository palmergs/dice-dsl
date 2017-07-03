class Dice::Histogram
  def initialize str
    scanner = Dice::Parser::Scanner.new(str)
    @roll = Dice::RolList.parse(scanner)
  end

  def roll_one

  end
end
class Dice::Histogram

  attr_accessor :iterations

  def initialize str
    scanner = Dice::Parser::Scanner.new(str)
    @roll = Dice::RollList.parse(scanner)
    @iterations = 10_000
  end

  def generate
    values = []
    iterations.times do
      @roll.roll!
      value = @roll.scalar
      value = 0 if value < 0
      values[value] = 0 if values[value].nil?
      values[value] += 1
    end
    values
  end

  def linear_chart max_length = 20
    arr = generate
    max = arr.compact.max
    incr = max / max_length
    incr = 1 if incr < 1
    base_chart(arr) do |value, index|
      len = (value && value > 0) ? (value / incr) : 0
      "#{ index }\t#{ '*' * len }"
    end
  end

  def log_chart
    base_chart(generate) do |value, index|
      log = (value && value > 0) ? (Math.log(value) + 1) : 0
      "#{ index }\t#{ '*' * log }"
    end
  end

  def base_chart arr
    start = false
    arr.each_with_index.map do |n, idx|
      if start || !n.nil?
        start = true
        yield(n, idx)
      end
    end.compact.join("\n")
  end
end

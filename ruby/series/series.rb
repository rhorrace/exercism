=begin
Write your code for the 'Series' exercise in this file. Make the tests in
`series_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/series` directory.
=end

class Series
  attr_reader :str, :length

  def initialize(str)
    @str = str
    @length = str.size
  end

  def slices(n)
    raise ArgumentError if n > @length

    (0..(@length - n)).to_a.map do |i|
      @str[i..(i + n - 1)]
    end
  end
end
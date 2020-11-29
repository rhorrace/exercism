=begin
Write your code for the 'Series' exercise in this file. Make the tests in
`series_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/series` directory.
=end

class Series
  private

  attr_reader :str

  public

  def initialize(str)
    @str = str
  end

  def slices(lim)
    raise ArgumentError if n > @str.size

    (0..(@str.size - lim)).map do |i|
      @str[i..(i + lim - 1)]
    end
  end
end

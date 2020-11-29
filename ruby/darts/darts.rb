=begin
Write your code for the 'Darts' exercise in this file. Make the tests in
`darts_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/darts` directory.
=end

class Darts
  def initialize(pos_x, pos_y)
    @distance = Math.sqrt(pos_x**2 + pos_y**2)
  end

  def score
    if @distance <= 1
      10
    elsif @distance <= 5
      5
    elsif @distance <= 10
      1
    else
      0
    end
  end

  private

  attr_writer :distance
end
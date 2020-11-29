=begin
Write your code for the 'Triangle' exercise in this file. Make the tests in
`triangle_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/triangle` directory.
=end

class Triangle
  def initialize(sides)
    @sides = sides.sort
    @valid = valid?
  end

  def equilateral?
    @valid && @sides.all? { |s| s == @sides[0] }
  end

  def isosceles?
    @valid && (@sides[0] == @sides[1] || @sides[0] == @sides[2] || @sides[1] == @sides[2])
  end

  def scalene?
    @valid && !equilateral? && !isosceles?
  end

  private

  def valid?
    @sides.none?(:zero?) &&
      @sides[0] + @sides[1] > @sides[2] &&
      @sides[0] + @sides[2] > @sides[1] &&
      @sides[1] + @sides[2] > @sides[0]
  end

  attr_reader :sides, :valid
end

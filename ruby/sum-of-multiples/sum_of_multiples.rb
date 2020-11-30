=begin
Write your code for the 'Sum Of Multiples' exercise in this file. Make the tests in
`sum_of_multiples_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/sum-of-multiples` directory.
=end

class SumOfMultiples
  def initialize(*args)
    @factors = args.to_a
  end

  def to(lim)
    return 0 if @factors.empty? || @factors.any?(0)

    (@factors.min...lim).select { |i| @factors.any? { |j| (i % j).zero? } }.sum
  end

  private

  attr_reader :factors
end

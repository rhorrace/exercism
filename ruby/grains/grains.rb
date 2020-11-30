=begin
Write your code for the 'Grains' exercise in this file. Make the tests in
`grains_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/grains` directory.
=end

class Grains

  def self.square(num)
    raise ArgumentError if num < 1 || num > 64

    2**(num - 1)
  end

  def self.total
    (1..64).map { |i| square(i) }.sum
  end
end
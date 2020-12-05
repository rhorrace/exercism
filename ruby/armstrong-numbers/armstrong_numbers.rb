=begin
Write your code for the 'Armstrong Numbers' exercise in this file. Make the tests in
`armstrong_numbers_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/armstrong-numbers` directory.
=end

class ArmstrongNumbers
  def self.include?(num)
    size = num.to_s.length

    num.to_s
       .chars
       .map { |d| d.to_i ** size }
       .sum == num
  end
end

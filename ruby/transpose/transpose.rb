=begin
Write your code for the 'Transpose' exercise in this file. Make the tests in
`transpose_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/transpose` directory.
=end

class Transpose
  def self.transpose(input)
    strings = input.gsub(' ', '_').split("\n").map(&:chars)
    max_length = strings.map(&:length).max

    strings.map { |line| line + ([' '] * (max_length - line.length)) }
           .transpose
           .map { |line| line.join.rstrip.gsub('_', ' ') }
           .join("\n")
  end
end

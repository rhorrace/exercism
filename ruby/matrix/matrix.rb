=begin
Write your code for the 'Matrix' exercise in this file. Make the tests in
`matrix_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/matrix` directory.
=end

class Matrix

    attr_reader :rows

    def initialize(input)
        @rows = input.split("\n").map do |row|
            row.split(' ').map(&:to_i)
        end
    end

    def columns
        @rows.transpose
    end

    def saddle_points
      # code here
    end
end


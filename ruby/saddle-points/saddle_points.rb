=begin
Write your code for the 'Saddle Points' exercise in this file. Make the tests in
`saddle_points_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/saddle-points` directory.
=end

class Matrix

  attr_accessor :rows

  def initialize(numbers)
    @rows = numbers.lines.map { |line| line.split(' ').map(&:to_i) }
  end

  def columns
    @rows.transpose
  end

  def saddle_points
    result = []
    rows.each_with_index do |row, row_i|
      columns.each_with_index do |column, column_i|
        result << [row_i, column_i] if row[column_i] == row.max && rows[row_i][column_i] == column.min
      end
    end
    result
  end
end

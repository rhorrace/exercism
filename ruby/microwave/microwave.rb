=begin
Write your code for the 'Microwave' exercise in this file. Make the tests in
`microwave_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/microwave` directory.
=end

class Microwave
  def initialize(input)
    @minute = input >= 100 ? input / 100 : 0
    input %= 100
    @second = input

    return unless @second >= 60

    @minute += 1
    @second -= 60
  end

  def timer
    minute_str = @minute >= 10 ? @minute.to_s : "0#{@minute}"
    second_str = @second >= 10 ? @second.to_s : "0#{@second}"
    "#{minute_str}:#{second_str}"
  end

  private

  attr_accessor :minute, :second
end

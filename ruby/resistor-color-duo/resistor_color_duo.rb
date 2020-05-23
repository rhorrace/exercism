=begin
Write your code for the 'Resistor Color Duo' exercise in this file. Make the tests in
`resistor_color_duo_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/resistor-color-duo` directory.
=end

class ResistorColorDuo
  def self.value(colors)
    codes = colors.map {|color| convert(color)}
    return codes[0] * 10 + codes[1]
  end

  private_class_method def self.convert(color)
    case color
    when "black"
      return 0
    when "brown"
      return 1
    when "red"
      return 2
    when "orange"
      return 3
    when "yellow"
      return 4
    when "green"
      return 5
    when "blue"
      return 6
    when "violet"
      return 7
    when "grey"
      return 8
    when "white"
      return 9
    else
      raise "Error: not valid"
    end
  end

end

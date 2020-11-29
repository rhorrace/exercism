=begin
Write your code for the 'Space Age' exercise in this file. Make the tests in
`space_age_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/space-age` directory.
=end

class SpaceAge
  def initialize(seconds)
    @earth_years = seconds / 31_557_600.0
  end

  def on_earth
    @earth_years
  end

  def on_mercury
    @earth_years / 0.2408467
  end

  def on_venus
    @earth_years / 0.61519726
  end

  def on_mars
    @earth_years / 1.8808158
  end

  def on_jupiter
    @earth_years / 11.862615
  end

  def on_saturn
    @earth_years / 29.447498
  end

  def on_uranus
    @earth_years / 84.016846
  end

  def on_neptune
    @earth_years / 164.79132
  end

  private

  attr_reader :earth_years
end

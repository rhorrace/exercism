=begin
Write your code for the 'Phone Number' exercise in this file. Make the tests in
`phone_number_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/phone-number` directory.
=end

class PhoneNumber
  def self.clean(number)
    trimmed = number.gsub(/[^0-9a-zA-Z]/, '')
    return trimmed if trimmed =~ /^[2-9]\d{2}[2-9]\d{6}$/
    return trimmed[1..-1] if trimmed =~ /^1[2-9]\d{2}[2-9]\d{6}$/

    nil
  end
end

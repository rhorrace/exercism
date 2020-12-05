=begin
Write your code for the 'Phone Number' exercise in this file. Make the tests in
`phone_number_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/phone-number` directory.
=end

class PhoneNumber
  def self.clean(number)
    trimmed = number.gsub(/[^0-9a-zA-Z]/, '')
    return nil unless trimmed =~ /\d/
    return trimmed if trimmed.length == 10 && !trimmed.start_with?(/[0-1]/) && trimmed[3] =~ /[2-9]/
    return trimmed[1..-1] if trimmed.length == 11 && trimmed.start_with?(/1[2-9]/) && trimmed[4] =~ /[2-9]/

    nil
  end
end

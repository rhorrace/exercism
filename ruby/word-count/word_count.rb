=begin
Write your code for the 'Word Count' exercise in this file. Make the tests in
`word_count_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/word-count` directory.
=end

class Phrase
  attr_accessor :word_count

  def initialize(words)
    @word_count = words.downcase
                       .scan(/\b[\w']+\b/)
                       .tally
  end
end

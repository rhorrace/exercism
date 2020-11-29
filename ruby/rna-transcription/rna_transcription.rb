# frozen_string_literal: true
# Write your code for the 'Rna Transcription' exercise in this file. Make the tests in
# `rna_transcription_test.rb` pass.
#
# To get started with TDD, see the `README.md` file in your
# `ruby/rna-transcription` directory.

class Complement
  def self.of_dna(strand)
    rna = ''
    strand.each_char do |c|
      rna += @complements[c]
    end
    rna
  end

  private

  @complements = {'A' => 'U', 'T' => 'A', 'C' => 'G', 'G' => 'C'}
end

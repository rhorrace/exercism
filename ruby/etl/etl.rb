=begin
Write your code for the 'ETL' exercise in this file. Make the tests in
`etl_test.rb` pass.

To get started with TDD, see the `README.md` file in your
`ruby/etl` directory.
=end

class ETL
  def self.transform(old)
    new = {}
    old.each_key do |k|
      old[k].map { |v| new[v.downcase] = k }
    end
    new
  end
end

# Dice::Dsl

Welcome to your new gem! In this directory, you'll find the files you need to be able to package up your Ruby library into a gem. Put your Ruby code in the file `lib/dice/dsl`. To experiment with that code, run `bin/console` for an interactive prompt.

TODO: Delete this and the text above, and describe your gem

## Installation

Add this line to your application's Gemfile:

```ruby
gem 'dice-dsl'
```

And then execute:

    $ bundle

Or install it yourself as:

    $ gem install dice-dsl

## Usage

This is a dice roller with options for more complex rules. For each roll type, a vector of the values rolled or a scalar value which is the sum can be returned.

* `Dice::SimpleRoll.parse('4d6')` allows for a roll of any number of dice
* `Dice::ModifiedRoll.parse('3d6+3')` allows for a roll of any number of dice modified by a fixed amount
* `Dice::ModifyEachRoll.parse('3d6++2')` allows for a roll of any number of dice where each is modified by a fixed amount
* `Dice::ExplodingRoll.parse('2d6!')` allows for a roll of dice; if all the dice rolled are the maximum value then the die is rerolled. This continues until a non-maximum die is rolled
* `Dice::ExplodeEachRoll.parse('3d6!!')` allows for a roll of dice; for each die that is the maximum value reroll until a non maximum value is rolled
* `Dice::TakeN.parse('4d6^3')` or ``Dice::TakeN.parse('4d6`3')`` or `Dice::TakeN.parse('4d6~3')` take the top, bottom or middle N of a series of rolled dice
* `Dice::RollList.parse('3d6, 2d8--1, 1d10, 2d12++1')` builds a vector of all the individial dice rolls separated by commas

There is a histogram class that can generate a chart of the given results:

    h = Dice::Histogram.new('2d8!!')
    puts h.linear_chart
    
    2   **
    3   *****
    4   *********
    5   ***********
    6   **************
    7   *****************
    8   ********************
    9   ******************
    10  ***************
    11  ************
    12  ***********
    13  ********
    14  ******
    15  ****
    16  *****
    17  ****
    18  ****
    19  ***
    20  **
    21  *
    22  *
    23  
    24  *
    25  *


## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake spec` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and tags, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/palmergs/dice-dsl.

## License

The gem is available as open source under the terms of the [MIT License](http://opensource.org/licenses/MIT).

# Dice::DSL

Dice::DSL was an attempt to build a dice parser that I could use for random games. The alternate goal was to define a library in a language I was somewhat familiar with (Ruby) and then folllow up with a project in a language I wasn't familiar with (Rust using the Helix bindings). I never got around to using Helix, but this library now includes both the ruby and, three years later, a rust version. 

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

## Development (2)

There is also a top-level `Cargo.toml` and `src` directory to define a rust based library and command line application. It works similarly to the ruby version:

    > cargo build --release
    > target/release/roll --chart 2d8\!\!
     2. 100.0: ********
     3.  98.4: ***************
     4.  95.2: ***********************
     5.  90.3: ******************************
     6.  84.0: *************************************
     7.  76.3: ********************************************
     8.  66.9: ***************************************************
     9.  56.1: ********************************************
    10.  46.8: ***************************************
    11.  38.6: *********************************
    12.  31.6: ****************************
    13.  25.7: **********************
    14.  21.1: *****************
    15.  17.6: ***********
    16.  15.3: *************
    17.  12.6: ***********
    18.  10.2: **********
    19.   8.1: ********
    20.   6.5: *******
    21.   5.1: ******
    22.   3.9: ****
    23.   3.2: ***
    24.   2.8: ***
    25.   2.2: **
    26.   1.8: **
    27.   1.4: **
    28.   1.1: **
    29.   0.8: *
    30.   0.7: *
    31.   0.5: *
    32.   0.5: *
    33.   0.4: *
    34.   0.3: *
    35.   0.2: *
    36.   0.2: *
    37.   0.1: *
    38.   0.1: *
    39.   0.1: *
    40.   0.1: *
    41.   0.1: *
    42.   0.0: *

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/palmergs/dice-dsl.

## License

The gem is available as open source under the terms of the [MIT License](http://opensource.org/licenses/MIT).

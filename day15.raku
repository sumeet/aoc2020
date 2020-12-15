#!/usr/bin/env perl6

# actual input
my $input = "2,0,1,9,5,19";
# test input
#$input = "0,3,6";

module state {
    my @nums = $input.split(',');
    my %last_spoken_at;

    our sub say_word($word, $at_index) {
        if has_been_spoken_before($word) {
            %last_spoken_at{$word}.push($at_index);
        } else {
            %last_spoken_at{$word} = [$at_index];
        }

        @nums.push($word);
    }

    our sub has_been_spoken_before($word) { %last_spoken_at{$word} }


    our sub has_been_spoken_twice($word) { %last_spoken_at{$word}.elems > 1 }

    our sub spoken_at($index) { @nums[$index] }

    our sub occurences_for($n) { %last_spoken_at{$n} }

    our sub current_turn() { @nums.elems }

    for @nums.kv -> $index, $num { %last_spoken_at{$num}.push($index) };
}

# start at 1
for state::current_turn()..* -> $cur_index {
    my $last_spoken_num = state::spoken_at($cur_index - 1);
    my $word_to_speak = state::has_been_spoken_twice($last_spoken_num) ?? do {
        my ($prev2, $prev) = state::occurences_for($last_spoken_num).tail(2);
        $prev - $prev2
    } !! do {
        0
    };

    state::say_word($word_to_speak, $cur_index);

    #if $cur_index == 4 {
    #    say $word_to_speak.raku;
    #    exit;
    #};

    if $cur_index == 2019 {
        say $word_to_speak;
        last;
    }

}


#print @input

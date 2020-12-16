#!/usr/bin/env perl6

# actual input
my $input = "2,0,1,9,5,19";
# test input
$input = "0,3,6";

module state {
    my @nums = $input.split(',').map: *.Numeric;
    my %last_spoken_at;

    our sub all_nums() { @nums }

    our sub last_spoken_at() { %last_spoken_at }

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
for state::current_turn()..300001 -> $cur_index {
    my $last_spoken_num = state::spoken_at($cur_index - 1);
    my $word_to_speak = state::has_been_spoken_twice($last_spoken_num) ?? do {
        my ($prev2, $prev) = state::occurences_for($last_spoken_num).tail(2);
        $prev - $prev2
    } !! do {
        0
    };

    state::say_word($word_to_speak, $cur_index);

    my $c = $cur_index.Str;
    my $first_num = substr($c, 0, 1);
    my $rest = substr($c, 1);
    #say $rest.unique.raku;
    #if $first_num == "3" {#}&& $rest.unique() == ("0") {
        if $rest.unique().elems == 1 && $rest.unique()[0] == "0" {
            say "$cur_index, $word_to_speak";

        }
    #}

    #if $word_to_speak == 175594 {
    #    say $cur_index;
    #    exit;
    #}
}

#my @acc;
# for state::all_nums().kv -> $i, $num {
#     if $num == 0 {
#         print "{@acc.raku}\t(len {@acc.elems})\ni{$i}\t\t{@acc.tail()} (prev)\t";
#         @acc = ();
#     }
#     @acc.push($num);
# }

#my @all = state::last_spoken_at().kv.map: { $^a.Numeric, $^b };
#@all = @all.sort();
#for @all -> ($k, $v) {
#   say "$k | {$v.head() - $k}: {$v}";
#}

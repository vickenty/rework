#!/usr/bin/env perl
use strict;
use warnings;
use Time::HiRes qw/time/;

my $last_update = 0;
my $max_offset = 0;
my $max_pos = "1:1";
my $total;
my @stack;
my $max_depth = 0;
my %heatmap;
my %backtrack;
my @warnings;

while(<>) {
    chomp;
    $total++;

    if (/(?:Entering|Leaving) level|Cached (?:match|fail)/) {
        next;
    }

    my ($action, $rule, $pos, $ofs) = /(Attempting to match|Matched|Failed to match) rule (\S+) at (\d+:\d+) \(pos (\d+)\)/a;
    if ($ofs > $max_offset) {
        $max_offset = $ofs;
        $max_pos = $pos;
    }

    if ($action eq "Attempting to match") {
        $stack[-1]{matches}++ if @stack;
        push @stack, { rule => $rule, pos => $pos, offset => $ofs, matches => 0 };

        $heatmap{rule}{$rule}{all}++;
        $heatmap{pos}{$pos}{rule}{$rule}++;
        $heatmap{pos}{$pos}{all}++;
    }
    else {
        my $top = pop @stack;
        $stack[-1]{matches} += $top->{matches};

        if (!defined $top) {
            die "empty stack";
        }
        if ($top->{rule} ne $rule) {
            die "wrong rule on the stack: parsed $rule, stack has $top->{rule}: $_";
        }

        if ($action eq "Failed to match" && $top->{matches} > 1000) {
            push @warnings, "rule '$rule' failed after $top->{matches} inner matches at $pos";
        }
    }
    if (@stack > $max_depth) {
        $max_depth = @stack;
    }

    if (time - $last_update > 1) {
        show();
        $last_update = time;
    }
}

show();

sub show {
    my $cols = `tput cols`;
    my $NL = "\033[K\n";

    print "\033[H";
    print "Total lines: $total${NL}";
    print "Max position: $max_pos ($max_offset)   Max depth: $max_depth${NL}";

    print "Top rules:${NL}";
    my @top_rules = reverse sort { $heatmap{rule}{$a}{all} <=> $heatmap{rule}{$b}{all} || $a cmp $b } keys %{$heatmap{rule}};
    foreach my $rule (splice @top_rules, 0, 10) {
        my $data = $heatmap{rule}{$rule};
        printf "%20s % 8d${NL}", $rule, $data->{all};
    }

    my $hm_pos = $heatmap{pos};
    my @top_pos = reverse sort { $hm_pos->{$a}{all} <=> $hm_pos->{$b}{all} || $a cmp $b } keys $hm_pos->%*;
    my $pos_pop = scalar @top_pos;
    print "${NL}Top 10 positions (total $pos_pop):${NL}";
    foreach my $pos (splice @top_pos, 0, 10) {
        my $rules = $hm_pos->{$pos}{rule};
        my @top_rules = reverse sort { $rules->{$a} <=> $rules->{$b} } keys %$rules;
        my $chunk = sprintf "%10s % 8d   ", $pos, $hm_pos->{$pos}{all};
        print $chunk;
        my $width = length $chunk;

        foreach my $rule (@top_rules) {
            my $chunk = sprintf "%s (%d)  ", $rule, $rules->{$rule};
            $width += length $chunk;
            last if $width >= $cols;
            print $chunk;
        }
        print "${NL}";
    }
    
    print "${NL}Warnings:${NL}";
    my @warns = reverse @warnings;
    for my $warn (splice @warns, 0, 10) {
        print " - $warn${NL}";
    }
    print "\033[J";
    
    STDOUT->flush();
}

#!/usr/bin/env ruby
# frozen_string_literal: true

require "find"
require "fast_find"
require "monotime"

def avg(&job)
  total = Array.new(5) do
    Monotime::Duration.measure(&job).tap { |d| print "#{d.to_s(2)} " }
  end.sort.take(4)

  total.inject(:+) / total.size
end

def bench(what, &job)
  print "#{what}: "
  puts "[ avg best 4: #{avg(&job).to_s(2)} ]"
end

def test(dir)
  bench("Find") { Find.find(dir) { |x| File.stat(x).mtime } }
  bench("FastFind") { FastFind.find(dir) { |_, stat| stat.mtime } }
end

test(ARGV.first || abort("Usage: #{$0} [directory]"))

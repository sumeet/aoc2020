def apply_mask(n)
  num = n.to_bin(NUM_BITS)
  floating_positions = []

  MASK.chars.zip(num.chars)
      .each_with_index do |(mask_char, _), i|
    if mask_char == "0"
      # do nothing
    elsif mask_char == "1"
      num[i] = "1"
    elsif mask_char == "X"
      floating_positions << i
    else
      raise "unknown mask char: #{mask_char}"
    end
  end

  addresses = []
  ["0", "1"].repeated_permutation(floating_positions.size).map do |new_values|
    new_num = num.dup
    new_values.zip(floating_positions).each do |value, pos|
      new_num[pos] = value
    end
    addresses << new_num.to_i(2)
  end
  addresses
end

class Integer
  def to_bin(width)
    '%0*b' % [width, self]
  end
end

#MASK = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
MASK = ""
NUM_BITS = 36
mem = {}

def mem.[]=(position, val)
  apply_mask(position).each do |each_pos|
    store(each_pos, val)
  end
end

open('day14.txt').each_line do |line|

  line = line.strip
  if line.start_with?("mask")
    MASK[0..-1] = line.split(" = ")[1]
  elsif line.start_with?("mem[")
    eval(line)
  else
    raise "didn't recognize #{line}"
  end
end

p MASK
p mem.values.sum

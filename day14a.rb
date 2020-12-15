def apply_mask(n)
  num = n.to_bin(NUM_BITS)
  MASK.chars.zip(num.chars)
      .each_with_index do |(mask_char, _), i|
    num[i] = mask_char if mask_char != "X"
  end
  num.to_i(2)
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
  store(position, apply_mask(val))
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

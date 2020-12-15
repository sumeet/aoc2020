input = "19,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,599,x,29,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17,x,x,x,x,x,23,x,x,x,x,x,x,x,761,x,x,x,x,x,x,x,x,x,41,x,x,13"

pairs = (input.split(",").each_with_index.map do |bus, i|
  if (bus = bus.to_i) > 0
    [bus, i]
  end
end.compact.to_a)


a = pairs.map(&:first)
b = pairs.map {|p| p[1]}

# take these values from above and feed them into python like the following
In [14]: from sympy.ntheory.modular import crt

In [15]: a, b = crt([7, 13, 59, 31, 19], [0, 1, 4, 6, 7]) ; b - a
Out[15]: mpz(1068781)

#
# the following is chinese remainder theorem stuff i didn't bother to get working
#
## from https://rosettacode.org/wiki/Chinese_remainder_theorem#Ruby
#def extended_gcd(a, b)
#  last_remainder, remainder = a.abs, b.abs
#  x, last_x, y, last_y = 0, 1, 1, 0
#  while remainder != 0
#    last_remainder, (quotient, remainder) = remainder, last_remainder.divmod(remainder)
#    x, last_x = last_x - quotient*x, x
#    y, last_y = last_y - quotient*y, y
#  end
#  return last_remainder, last_x * (a < 0 ? -1 : 1)
#end
# 
#def invmod(e, et)
#  g, x = extended_gcd(e, et)
#  if g != 1
#    raise 'Multiplicative inverse modulo does not exist!'
#  end
#  x % et
#end
# 
#def chinese_remainder(mods, remainders)
#  max = mods.inject( :* )  # product of all moduli
#  series = remainders.zip(mods).map{ |r,m| (r * max * invmod(max/m, m) / m) }
#  series.inject( :+ ) % max 
#end
#
#
#p chinese_remainder(a, b)


#n = 1
#c = 0
#
#[[5, 4]]


# from solve (t mod 19 = 0) and (t mod 37 = 13) and (t mod 599 = 19) and (t mod 29 = 21) 
# (missing a bunch off the end)
#def go(n)
#  19.to_r * ((642727.to_r * n.to_r) + 306090.to_r)
#end
#
##p (0..1000).map {|n| go(n)}
#p go(1_000_000_0) > 100000000000000

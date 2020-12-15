input = '1009310
19,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,599,x,29,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17,x,x,x,x,x,23,x,x,x,x,x,x,x,761,x,x,x,x,x,x,x,x,x,41,x,x,13'


earliest_time, bus_start_times = input.strip.split("\n")
earliest_time = Integer(earliest_time)

bus_start_times = bus_start_times.split(",").map { |i| Integer(i) rescue nil }.compact
#bus_start_times_by_id = bus_start_times.map { |start_time| [start_time, start_time] }.to_h

possible_start_times = bus_start_times.map do |start_time|
  div = earliest_time.to_r / start_time.to_r
  if div == 0
    [start_time, div]
  else
    [start_time, (div.floor + 1) * start_time]
  end
end.to_h

bus_id, start_time = possible_start_times.sort_by do |bus_id, value|
  value
end.to_a.first


p (start_time - earliest_time) * bus_id

lines = File.read("part1.txt").strip.split("\n").map(&:strip)

elves = []
current_elf = []

lines.each do |line|
  if line.empty?
    elves << current_elf
    current_elf = []
    next
  end

  current_elf << line.to_i
end

if !current_elf.empty?
  elves << current_elf
end

result = elves.map { |elf| elf.sum }.max
puts "result: #{result}"

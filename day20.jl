INPUT = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
"
import StatsBase.countmap
import Base.product
import Base.Iterators.flatten

# julia weirdness, a small price to pay for the built in matrix operations
import Base.transpose
transpose(c::Char) = c

photo_parts = []

const Pixels = Array{Char, 2}

struct EdgeValues
    top::Int
    bottom::Int
    left::Int
    right::Int
    function EdgeValues(pixels::Pixels)
        top = pixels[1, :]
        bottom = pixels[10, :]
        left = pixels[:, 1]
        right = pixels[:, 10]
        new(num_from_binary_rep(top), num_from_binary_rep(bottom),
            num_from_binary_rep(left), num_from_binary_rep(right))
    end
end

function num_from_binary_rep(a::Vector{Char}) :: Int
    parse(Int, join(c == '#' ? 1 : 0 for c in a), base = 2)
end

struct PhotoPart
    tile_id::Int
    pixels::Pixels
end

function all_edge_value_permutations(pixels::Pixels)
    Channel() do channel
        for flip in all_flips(pixels)
            put!(channel, [num_from_binary_rep(edge) for edge in all_edges(flip)])
        end
    end
end

function all_edges(pixels::Pixels)
    Channel() do channel
        # top
        put!(channel, pixels[1, :])
        # bottom
        put!(channel, pixels[10, :])
        # left
        put!(channel, pixels[:, 1])
        # right
        put!(channel, pixels[:, 10])
    end
end

global current_tile_id = nothing
current_tile_lines = Vector{Char}[]
for line in split(INPUT, "\n")
    if startswith(line, "Tile")
        global current_tile_id = parse(Int, strip(line, collect("Tile :")))
    elseif line == ""
        pixels = mapreduce(transpose, vcat, current_tile_lines)
        push!(photo_parts, PhotoPart(current_tile_id, pixels))
        empty!(current_tile_lines)
    else
        push!(current_tile_lines, collect(line) :: Vector{Char})
    end
end

function all_flips(pixels_orig)
    Channel() do channel
        put!(channel, pixels_orig)
        put!(channel, reverse(pixels_orig, dims=1))
        put!(channel, reverse(pixels_orig, dims=2))
    end
end

struct Tile
    tile_id
    edge_permutation
end

all_edges_from_all_parts = [
    [Tile(photo_part.tile_id, permutation) for permutation in
        all_edge_value_permutations(photo_part.pixels)]
        for photo_part in photo_parts]

all_permutations_of_tile_edges = [product(all_edges_from_all_parts...)...] ::Vector{NTuple{9,Tile}}
permutations_with_counts = [(tiles, edge_count_by_value=countmap(flatten(tile.edge_permutation for tile in tiles))) for tiles in all_permutations_of_tile_edges]
num_edges, i = findmin(map(pwc -> length(pwc.edge_count_by_value), permutations_with_counts))
permutation_with_counts = permutations_with_counts[i]

function score(tile::Tile, edge_count_by_value)
    sum(map(edge -> edge_count_by_value[edge], tile.edge_permutation))
end

sorted_tiles = sort(collect(permutation_with_counts.tiles), by=(tile -> score(tile, permutation_with_counts.edge_count_by_value)))
prod(map(t -> t.tile_id, sorted_tiles[1:4]))

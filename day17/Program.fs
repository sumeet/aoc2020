type CellState =
    | Active
    | Inactive

let parseCell c =
    match c with
    | '#' -> Active
    | '.' -> Inactive
    | _ -> failwith (sprintf "couldn't parse %A" c)

type Point = int * int * int

let input = "..#..##.
#.....##
##.#.#.#
..#...#.
.###....
######..
.###..#.
..#..##."

let mutable pocket = Map.empty

let parsedInput =
    for (y, line) in input.Split("\n") |> Array.indexed do
        for (x, char) in line.ToCharArray() |> Array.indexed do
            pocket <- pocket.Add((x, y, 0), parseCell (char))

let neighbors (x, y, z) =
    seq {
        for dx in [ -1; 0; 1 ] do
            for dy in [ -1; 0; 1 ] do
                for dz in [ -1; 0; 1 ] do
                    if not ((dx, dy, dz) = (0, 0, 0)) then yield (x + dx, y + dy, z + dz)
    }



let getPoint (pocket: Map<_, _>, point) =
    match pocket.TryFind(point) with
    | Some (state) -> state
    | None -> Inactive


let transition (beforePocket: Map<Point, CellState>) =
    let mutable afterPocket = beforePocket

    let mutable newNeighbors = Set.empty

    for (point, state) in Map.toSeq (beforePocket) do
        let allNeighbors = neighbors (point)

        let numActiveNeighbors =
            allNeighbors
            |> Seq.map (fun point ->
                // a little sneaky here,adding in neighbors while iterating
                // for a different reason
                (newNeighbors <- newNeighbors.Add(point)
                 getPoint (beforePocket, point)))
            |> Seq.filter (fun state -> state = Active)
            |> Seq.length

        match state with
        | Active ->
            do (if numActiveNeighbors <> 2 && numActiveNeighbors <> 3
                then afterPocket <- afterPocket.Add(point, Inactive))
        | Inactive ->
            if numActiveNeighbors = 3
            then afterPocket <- afterPocket.Add(point, Active)

    let prevPoints =
        Map.toSeq beforePocket |> Seq.map fst |> Set.ofSeq

    newNeighbors <- newNeighbors - prevPoints

    for inactivePoint in newNeighbors do
        let allNeighbors = neighbors (inactivePoint)

        let numActiveNeighbors =
            allNeighbors
            |> Seq.map (fun point -> getPoint (beforePocket, point))
            |> Seq.filter (fun state -> state = Active)
            |> Seq.length

        if numActiveNeighbors = 3
        then afterPocket <- afterPocket.Add(inactivePoint, Active)

    afterPocket

let printActives () =
    (let actives =
        (Map.toSeq pocket
         |> Seq.filter (fun (k, v) -> v = Active))

     printf "%d actives\n" (Seq.length actives)

    //     for active in actives do
//         printf "%A" active
//
//     printf "\n")
    )

for _ in 0 .. 5 do
    pocket <- transition (pocket)

printActives ()

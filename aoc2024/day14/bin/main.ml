let file = "input.txt"
let rows = 103
let cols = 101
let mid_row = 51
let mid_col = 50

let parse_pair str =
    match str with
    | [x; y] -> [x; y]
    | _ -> assert false;;

let positive_mod x n =
    let rem = x mod n in
    if rem < 0 then rem + n else rem;;

let inc_hash (robots, quad) =
    if (Hashtbl.mem !robots quad) then
        Hashtbl.replace !robots quad ((Hashtbl.find !robots quad) + 1)
    else
        Hashtbl.add !robots quad 1;;


let move_robot_n_seconds_to_quad secs (pos_x, pos_y, vel_x, vel_y, robots) =
    let new_pos_x = positive_mod (pos_x + vel_x * secs) cols in
    let new_pos_y = positive_mod (pos_y + vel_y * secs) rows in
    if (new_pos_x != mid_col && new_pos_y != mid_row) then
        match (new_pos_x < mid_col, new_pos_y < mid_row) with
        | (true, true) -> inc_hash (robots, 1)
        | (true, false) -> inc_hash (robots, 2)
        | (false, true) -> inc_hash (robots, 3)
        | (false, false) -> inc_hash (robots, 4)

let move_robot_n_seconds secs (pos_x, pos_y, vel_x, vel_y, robots) =
    let new_pos_x = positive_mod (pos_x + vel_x * secs) cols in
    let new_pos_y = positive_mod (pos_y + vel_y * secs) rows in
    if not (Hashtbl.mem !robots [new_pos_x; new_pos_y]) then
        Hashtbl.add !robots [new_pos_x; new_pos_y] 1;;

[@@@ocaml.warning "-8"]
let parse_line (str, res, f) =
    let [pos_str; velocity_str] = parse_pair (String.split_on_char ' ' (String.trim(str))) in
    let [_; pos_str] = parse_pair (String.split_on_char '=' pos_str) in
    let [pos_x; pos_y] = parse_pair (List.map int_of_string (String.split_on_char ',' pos_str)) in
    let [_; vel_str] = parse_pair (String.split_on_char '=' velocity_str) in
    let [vel_x; vel_y] = parse_pair (List.map int_of_string (String.split_on_char ',' vel_str)) in
    f (pos_x, pos_y, vel_x, vel_y, res);;

let move_robots (res, f) =
    let ic = open_in file in
    let rec loop res =
        let line = input_line ic in
            parse_line (line, res, f);
            loop res;
    in
        try loop res with
            End_of_file -> close_in ic;;

let display res =
    for row = 0 to rows-1 do
        let () = for col = 0 to cols-1 do
            if (Hashtbl.mem !res [col; row]) then
            print_string "X"
            else print_string "."
        done in
        print_string "\n";
    done;
    print_string "\n";
    print_string "\n";;

let rec dfs res visited [x; y]=
    let ns = [
        (if (Hashtbl.mem !res [x-1; y-1]) then [x-1; y-1] else []);
        (if (Hashtbl.mem !res [x-1; y]) then [x-1; y] else []);
        (if (Hashtbl.mem !res [x; y-1]) then [x; y-1] else []);
        (if (Hashtbl.mem !res [x+1; y-1]) then [x+1; y-1] else []);
        (if (Hashtbl.mem !res [x+1; y]) then [x+1; y] else []);
        (if (Hashtbl.mem !res [x+1; y+1]) then [x+1; y+1] else []);
        (if (Hashtbl.mem !res [x-1; y+1]) then [x-1; y+1] else []);
        (if (Hashtbl.mem !res [x; y+1]) then [x; y+1] else [])
    ] in
    let non_empty l = match l with
        | [] -> false
        | h   -> not (Hashtbl.mem !visited h)
    in
    let stack = List.filter non_empty ns in
    let f init s = dfs res visited s + init in
    Hashtbl.add !visited [x; y] 1;
    (List.fold_left f 0 stack) + 1;;


let large_cc (res, large) =
    for row = 0 to rows-1 do
        let () = for col = 0 to cols-1 do
            let visited = ref (Hashtbl.create 1000) in
            if (Hashtbl.mem !res [col; row]) && not !large then
                large := ((dfs res visited [col; row]) > 100)
        done in
        ();
    done;;

let () =
    let res = ref (Hashtbl.create 5) in
    move_robots (res, move_robot_n_seconds_to_quad 100);
    let final = (Hashtbl.find !res 1) * (Hashtbl.find !res 2) * (Hashtbl.find !res 3) * (Hashtbl.find !res 4) in
    print_string "Part 1: ";
    print_int final;
    print_string "\n";
    print_string "\n";;

let () =
    for secs = 0 to 8087 do
        let res = ref (Hashtbl.create 1000) in
        let large = ref false in
        move_robots (res, move_robot_n_seconds secs);
        flush stdout;
        large_cc (res, large);
        let pr res = display res; flush stdout; in
        if !large then (
            pr res;
            Printf.printf "Part 2: %d\n" secs;
        )
    done;;




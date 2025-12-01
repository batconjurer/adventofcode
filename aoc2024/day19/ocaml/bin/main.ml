let file = "../input.txt"

(* Parse the input file into two lists.  *)
let parse_file towels patterns =
    let ic = open_in file in
    let first_line = input_line ic in
    let rec loop parsed_lines =
        let line = input_line ic in
        parsed_lines := !parsed_lines @ [line];
        loop parsed_lines;
    in
        towels := List.map String.trim (String.split_on_char ',' (String.trim(first_line)));
        try loop patterns with
            End_of_file -> close_in ic

(* Initialize the towel and pattern lists and populated them by parsing the input file *)
let get_towels_and_patterns =
    let towels = ref [] in
    let patterns = ref [] in
    parse_file towels patterns;
    (!towels, !patterns)

(*
   If `pattern` begins with `towel`, return the remaining pattern after
   stripping `towel` from the beginning
*)
let strip_prefix (towel, pattern) =
    let towel_len = String.length towel in
    let pattern_len = String.length pattern in
    flush stdout;
    if String.starts_with ~prefix: towel pattern then
        Some (String.sub pattern towel_len (pattern_len - towel_len))
    else
        None;;

(*
    Counts the number of ways to fill a given pattern using towels
    from the provided lists. The cache argument is for memoization.
 *)
let rec fill_pattern (towels, remaining_pattern, cache) =
    let total = ref 0 in
    let inner towel =
        match strip_prefix (towel, remaining_pattern) with
        | None -> ()
        (* pattern has been filled *)
        | Some "" -> incr total;
        (* check for cache hit. Otherwise, recurse *)
        | Some (rem) -> if (Hashtbl.mem !cache rem) then (
            total := !total + (Hashtbl.find !cache rem);
        ) else (
            let amt = fill_pattern (towels, rem, cache) in
            Hashtbl.add !cache rem amt;
            total := !total + amt;
        )
        in
    if String.length remaining_pattern == 0 then 1
    else (
        List.iter inner towels;
        !total
    );;


let () =
    let (towels, patterns) = get_towels_and_patterns in
    let cache = ref (Hashtbl.create 100000) in
    let inc counter pattern =
        (* In part 1, we only care if filling the pattern is possible. *)
        match fill_pattern (towels, pattern, cache) with
        | 0 -> counter
        | _-> counter + 1
    in
    let total = List.fold_left inc 0 patterns in
    Printf.printf "Part 1: %d\n" total;;

let () =
    let (towels, patterns) = get_towels_and_patterns in
    let cache = ref (Hashtbl.create 100000) in
    let inc counter pattern =
        counter + fill_pattern (towels, pattern, cache)
    in
    let total = List.fold_left inc 0 patterns in
    Printf.printf "Part 2: %d\n" total;;
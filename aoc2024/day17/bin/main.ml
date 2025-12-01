let program = [2; 4; 1; 1; 7; 5; 1; 5; 4; 1; 5; 5; 0; 3; 3; 0]
let rev_program = [0; 3; 3; 0; 5; 5; 1; 4; 5; 1; 5; 7; 1; 1; 4; 2]

(* Compute the power of two ints using squaring *)
let rec pow a = function
  | 0 -> 1
  | 1 -> a
  | n ->
    let b = pow a (n / 2) in
    b * b * (if n mod 2 = 0 then 1 else a);;


let combo_op (regA, regB, regC, operand) =
    match operand with
    | 0 -> 0
    | 1 -> 1
    | 2 -> 2
    | 3 -> 3
    | 4 -> !regA
    | 5 -> !regB
    | 6 -> !regC
    | _ -> assert false;;

let adv (regA, regB, regC, operand) =
    let op = combo_op (regA, regB, regC, operand) in
    regA := !regA / (pow 2 op);;

let bxl (regB, operand) =
    regB := !regB lxor operand;;

let bst (regA, regB, regC, operand) =
    let op = combo_op (regA, regB, regC, operand) in
    regB := op mod 8;;

(* remove the first `inst_no` cmds from the input program *)
let rec jnz cmds instr_no =
    if instr_no == 0 then
     cmds
    else match cmds with
        | [] -> []
        | _ :: rest -> jnz rest (instr_no - 1)

let bxc (regB, regC) =
    regB := !regB lxor !regC;;

(* Append the output to the output list *)
let out (regA, regB, regC, output, operand) =
    let op = combo_op (regA, regB, regC, operand) in
    output := !output @ [op mod 8];;

let bdv (regA, regB, regC, operand) =
    let op = combo_op (regA, regB, regC, operand) in
    regB := !regA / (pow 2 op);;

let cdv (regA, regB, regC, operand) =
    let op = combo_op (regA, regB, regC, operand) in
    regC := !regA / (pow 2 op);;

(* perform the next operation of the program and return the remaining sub-program *)
let rec run_program (regA, regB, regC, output, cmds) =
    match cmds with
    | [] -> []
    | 0 :: op :: rest ->
        adv (regA, regB, regC, op);
        run_program (regA, regB, regC, output, rest)
    | 1 :: op :: rest ->
        bxl (regB, op);
        run_program (regA, regB, regC, output, rest)
    | 2 :: op :: rest ->
        bst (regA, regB, regC, op);
        run_program (regA, regB, regC, output, rest)
    | 3 :: instr_ptr :: rest -> if !regA != 0 then (
            (* jump to the part of the program given by the instruction pointer *)
            run_program (regA, regB, regC, output, jnz program instr_ptr)
         ) else (
            run_program (regA, regB, regC, output, rest)
        )
    | 4 :: _ :: rest ->
        bxc (regB, regC);
        run_program (regA, regB, regC, output, rest)
    | 5 :: op :: rest ->
        out (regA, regB, regC, output, op);
        run_program (regA, regB, regC, output, rest)
    | 6 :: op :: rest ->
        bdv (regA, regB, regC, op);
        run_program (regA, regB, regC, output, rest)
    | 7 ::  op :: rest ->
        cdv (regA, regB, regC, op);
        run_program (regA, regB, regC, output, rest)
    | _ -> assert false;;

(* Print out a list *)
let rec display ls =
    match ls with
    | [] -> print_string "\n"
    | h :: rest -> print_int h;
        print_string ",";
        display rest;;

(* Compute the program on this input *)
let () =
    let regA = ref 17323786 in
    let regB = ref 0 in
    let regC = ref 0 in
    let output = ref [] in
    let _ = run_program (regA, regB, regC, output, program) in
    print_string "Part 1: ";
    display !output;;

(* Check if ls1 is equal to ls2 after truncating ls2 to the length of ls1 *)
let rec match_pref ls1 ls2 =
    match (ls1, ls2) with
    | ([], _) -> true
    | (_, []) -> false
    | (h1 :: t1, h2 :: t2) -> if (h1 == h2) then (match_pref t1 t2) else false;;

let find_next_octal (min, max) =
    let min_of a b =
        if !a > b then a := b in
     let max_of a b =
        if !a < b then a := b in
    let new_min = ref (max*8 + 8) in
    let new_max = ref 0 in
    for a = min*8 to (max*8 + 7) do
        let regA = ref a in
        let regB = ref 0 in
        let regC = ref 0 in
        let output = ref [] in
        let _ = run_program (regA, regB, regC, output, program) in
        if ( match_pref (List.rev !output) rev_program) then (
            min_of new_min a;
            max_of new_max a;
        )
    done;
    (!new_min, !new_max);;

let () =
    let min = ref 0 in
    let max = ref 0 in
    while !min < pow 2 27 do
        let (new_min, new_max) = find_next_octal (!min, !max) in
        min := new_min;
        max := new_max;
    done;
    (* try to narrow the search space a bit and see if we get lucky *)
    max := !min;
    while !min < pow 2 47 do
        let (new_min, new_max) = find_next_octal (!min, !max) in
        min := new_min;
        max := new_max;
    done;
    Printf.printf "Part 2: %d\n" !min;;
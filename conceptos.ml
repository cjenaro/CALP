(* LISTAS *)

(* 1 *)

let rec last ls = match ls with
                | [] -> None
                | [x] -> Some x
                | x::xs -> last xs;;

(* 2 *)

let rec last_two ls = match ls with
                        | [] -> None
                        | [x] -> None
                        | x::xs -> begin
                                match xs with
                                | [] -> None
                                | [y] -> Some (x, y)
                                | y::ys -> last_two xs
                                end;;

(* 3 *)

let rec fold_left f a ls = match ls with
                            | [] -> a
                            | x::xs -> fold_left f (f a x) xs;;


(* 4 *)

let mapi = let rec mapip i f l = match l with
                        | [] -> []
                        | x::xs -> (f i x)::mapip (i+1) f xs in
                        mapip 0;;

(* 5 *)

let rec filter f l = match l with
                        | [] -> []
                        | x::xs -> begin
                                    match f x with
                                    | true -> x::filter f xs
                                    | false -> filter f xs
                                    end;;

(* 6 *)

let mapi_fold = let rec mapip_fold i f l = match l with
                        | [] -> []
                        | x::xs -> fold_left f (i+1) xs
                        | x::xs -> (f i x)::mapip (i+1) f xs in
                        mapip_fold 0;;

(* 7 *)

let sinceros f ls = map f (filter (fun y -> y <> 0) ls);;

(* tipo es (int -> 'a) -> int list -> 'a list = <fun> *)

(* 8 *)

let rec compress ls = match ls with
                        | [] -> []
                        | [x] -> [x]
                        | x::xs -> begin
                                    match xs with
                                    | [] -> [x]
                                    | y::ys -> if x<>y then
                                                    x::(compress (y::ys))
                                                else
                                                    compress (y::ys)
                                    end;;

(* 9 *)

let rec sacar_option l = match l with
                            | [] -> []
                            | x::xs -> begin
                                        match x with
                                            | None -> sacar_option xs
                                            | Some y -> y::(sacar_option xs)
                                        end;;

let none_list l = let x = sacar_option l in
                            if List.length x < List.length l then
                                None
                            else
                                Some x;;

(* TIPOS *)

(* 1 *)

type roman_numeral = I | V | X | L | C | D | M;;
type number = roman_numeral list;;

let int_de_numeral x = match x with
                        | I -> 1
                        | V -> 5
                        | X -> 10
                        | L -> 50
                        | C -> 100
                        | D -> 500
                        | M -> 1000;;


let int_de_numero = List.map int_de_numeral;;

let rec suma_de_lista ls = match ls with
                        | [] -> 0
                        | x::xs -> begin
                                    match xs with
                                    | [] -> x
                                    | y::ys -> if x>y then
                                                (x-y) + suma_de_lista ys
                                            else
                                                (x+y) + suma_de_lista ys
                                    end;;

let convertir_romano_int x = suma_de_lista(List.rev(int_de_numero x));;

(* 2 *)

type 'a abb = Hoja | Nodo of 'a abb * 'a * 'a abb;;

let rec map_a f abb = match abb with
                    | Hoja -> Hoja
                    | Nodo(l,n,r) -> Nodo(map_a f l, f n, map_a f r);;

let rec fold f a abb = match abb with
    | Hoja -> a
    | Nodo(l,n,r) -> fold f (fold f (f a n) l) r;;



(* 3 *)

type tipo = Gato | Perro | Pajaro;;

type animal =
    {
    tipo : tipo;
    vuela : bool;
    patas : int;
    };;

let amputar_pata an = {an with patas = an.patas-1};;

let helicopterizar an = {an with vuela = true};;

(* REFERENCIAS *)

(* 1 *)

let max_for ls =
        let m = ref (List.nth ls 0) in
            for i = 1 to ((List.length ls) - 1) do
                let k = List.nth ls i in
                    if !m < k then
                        m := k
            done;
            !m;;

let max_fun ls = List.fold_left max min_int ls;;

(* 2 *)

type materia =
{
    nombre : string;
    nota : float
};;

type estudiante =
{
    libreta : int;
    nombre : string;
    materias : (materia list) ref;
    score : (unit -> float) ref
};;

(* 3 *)

let mate = {nombre = "Matematica" ; nota = 5.0};;
let leng = {nombre = "Lengua" ; nota = 5.0};;

let sumar_materia m n = m +. n.nota;;

let new_s l n = let m = ref [] in
    {libreta = l; nombre = n; materias = m; score = ref (fun avg -> (List.fold_left (sumar_materia) 0.0 !m) /. (float (List.length !m)))} ;;

let beta = new_s 5 "Beta";;
beta.materias := materiass;;
(* MODULOS *)

(* 1 *)

module type COUNTER = sig
    type t
    val init : unit -> t
    val inc : t -> t
    val lec : t -> int
end;;

module CounterF : COUNTER = struct
    type t = int
    let init () = 0
    let inc n = n + 1
    let lec n = n
end;;

module CounterR = struct
    type t = int ref
    let init () = ref 0
    let inc n = n := !n + 1; n
    let lec n = !n
end;;

(* 3 *)

module type NOMBRES = sig
    type t
    val init : string -> t
    val print_fresh : t -> t
end;;

module Names (Counter : COUNTER) : NOMBRES = struct
    type t = { nombre : string; count : Counter.t}
    let init s = {nombre = s; count = Counter.init()}
    let print_fresh x = Printf.printf "%s %d\n" x.nombre (Counter.lec x.count); { x with count = Counter.inc x.count }

end;;

module Name = Names(CounterF);;



type 'a stream = Nil | Cons of ('a * (unit -> 'a stream));;

let headST st =  match st
            | Cons (a, _) -> a;;


let tailST st = match st with
            | Cons (_, tl) -> tl ();;

let rec take i st =
  match i with
  | 0 -> []
  | i -> headST st :: take (i - 1) (tailST st);;

let rec mapST f st = match st with
                    | Nil -> Nil
                    | Cons (a, tl) -> Cons (f a, fun _ -> mapST f (tl ()));;


let rec filter f st = match st with
                    | Nil -> Nil
                    | Cons (a, tl) -> if f a then
                                        Cons (a, fun _ -> filter f (tl ()))
                                      else
                                         filter f (tl ());;


let rec unzip st = match st with
              | Nil -> (Nil, Nil)
              | Cons (a, tl) ->
              (Cons(fst a, fun _ -> fst(unzip (tl ()))) ,
               Cons(snd a, fun _ -> snd(unzip (tl ()))));;


let rec zip st st'= match st, st' with
                | Nil, Nil -> Nil
                | Nil, a -> Nil
                | a, Nil -> Nil
                | Cons (a, tl), Cons (b, tl') -> Cons((a,b), fun _ -> zip (tl ()) (tl' ()));;


let nats = let rec nats' x = Cons(x, fun _ -> nats' (x+1)) in nats' 0;;

let is_pair n = match n mod 2 with
              | 0 -> true
              | _ -> false;;


let pares = filter is_pair nats;;

let rec dup (st : 'a stream) : ('a stream) = match st with
            | Nil -> Nil
            | Cons (a, tl) -> Cons(a,(fun _ -> Cons(a, (fun _ -> dup (tl ())))));;

let dups = dup nats;;

let st_strings = mapST string_of_int nats;;

module type THUNKS = sig
    type 'a t
    val from_fun : (unit -> 'a) -> 'a t
    val force : 'a t -> 'a
end;;

module Thunk : THUNKS = struct
  type 'a t = (Either(unit -> 'a, 'a)) ref
  let from_fun f = ref Left f
  let force (a : (unit -> 'a) ref) = match !a with
                                      | Left f -> let x = f() in
                                                    a := Right (f ()); x
                                      | Right b -> b
end;;

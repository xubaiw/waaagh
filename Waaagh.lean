import Lean.Data.Parsec

open Lean Parsec

inductive Expr
| atom : String → Expr
| list : Array Expr → Expr
deriving Repr

-- TODO
partial def atom : Parsec Expr := 
   Expr.atom <$> many1Chars (satisfy λ c => c ≠ ' ' ∧ c ≠ ',' ∧ c ≠ '(' ∧ c ≠ ')')

def manySep (p : Parsec α) (s : Parsec β) : Parsec $ Array α := do
  manyCore (attempt (s *> p)) #[←p]

def sep : Parsec Unit := 
  ws *>
  optional (pchar ',') *>
  ws

mutual 

partial def expr : Parsec Expr :=
  atom <|> list
 
partial def list : Parsec Expr := do
  skipString "("
  let r ← manySep expr sep
  skipString ")"
  return Expr.list r

end

def parse (s : String) : Except String Expr :=
  match expr s.mkIterator with
  | Parsec.ParseResult.success _ res => Except.ok res
  | Parsec.ParseResult.error it err  => Except.error s!"offset {repr it.i.byteIdx}: {err}"


#eval parse "(displayln (+ 1 2))"
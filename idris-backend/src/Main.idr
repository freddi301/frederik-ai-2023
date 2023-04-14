module Main

main : IO ()
main = putStrLn "Hello world"

data Term : Type -> Type where
    Variable : variable -> Term variable
    Not : Term variable -> Term variable
    And : Term variable -> Term variable -> Term variable
    Or : Term variable -> Term variable -> Term variable

interface EvaluateIn term context where
    evaluate : term -> context -> Bool

EvaluateIn variable context => EvaluateIn (Term variable) context where
    evaluate (Variable variable) context = evaluate variable context
    evaluate (Not x) context = not (evaluate x context)
    evaluate (And x y) context = (evaluate x context) && (evaluate y context)
    evaluate (Or x y) context = (evaluate x context) || (evaluate y context)

data CharacterInWindow : Type where
    CharacterAtNegativeOffset : Int -> Char -> CharacterInWindow

evaluateAccuracy : (Foldable contexts, EvaluateIn (Term variable) context) => Term variable -> contexts context -> Double
evaluateAccuracy term contexts = toNumber (foldr reducer (0, 0) contexts) where    
    reducer context (trues, falses) = case evaluate term context of
        True => (trues + 1, falses)
        False => (trues, falses + 1)
    toNumber (trues, falses) = let all = trues + falses in (cast trues) / (cast all)

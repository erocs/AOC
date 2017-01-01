import Debug.Trace

import Data.Bits
import Data.List.Split
import qualified Data.Map as Map
import Data.Word
import Text.Read

type Value = Word16
type Wire = String

data Operand = Constant Value
             | Connection Wire
    deriving (Show, Eq)

data Operator = OpNothing
              | OpId Operand
              | OpAnd Operand Operand
              | OpOr Operand Operand
              | OpLshift Operand Operand
              | OpRshift Operand Operand
              | OpNot Operand
    deriving (Show, Eq)

type OperatorMap = Map.Map Wire Operator

parseOperand :: String -> Operand
parseOperand s = case (readMaybe s :: Maybe Word16) of
    Just n -> Constant n
    _ -> Connection s

parseOp' :: [String] -> (Wire, Operator)
parseOp' (x:"AND":y:"->":w:[]) = (w, OpAnd (parseOperand x) (parseOperand y))
parseOp' (x:"OR":y:"->":w:[]) = (w, OpOr (parseOperand x) (parseOperand y))
parseOp' (x:"LSHIFT":y:"->":w:[]) = (w, OpLshift (parseOperand x) (parseOperand y))
parseOp' (x:"RSHIFT":y:"->":w:[]) = (w, OpRshift (parseOperand x) (parseOperand y))
parseOp' ("NOT":x:"->":w:[]) = (w, OpNot $ parseOperand x)
parseOp' (x:"->":w:[]) = (w, OpId $ parseOperand x)
parseOp' _ = ("", OpNothing)

parseOperator :: String -> (Wire, Operator)
parseOperator s = parseOp' $ splitOn " " s

parseComputer :: String -> OperatorMap
parseComputer s = Map.fromList $ map parseOperator $ lines s

runOperand :: OperatorMap -> Operand -> Maybe Word16
runOperand m (Constant c) = Just c
runOperand m (Connection w) = runComputer w m

run2Op :: (Word16 -> Word16 -> Word16) -> OperatorMap -> Operand -> Operand -> Maybe Word16
run2Op p m a b = let
    oa = runOperand m a
    ob = runOperand m b
    op' (Just ra) (Just rb) = Just $ p ra rb
    op' _ _ = Nothing
  in
    op' oa ob

runAnd = run2Op $ \a b -> a .&. b
runOr = run2Op $ \a b -> a .|. b
runLshift = run2Op $ \a b -> shift a $ fromIntegral b
runRshift = run2Op $ \a b -> shift a $ negate $ fromIntegral b

runNot :: OperatorMap -> Operand -> Maybe Word16
runNot m o = case (runOperand m o) of
    Just n -> Just $ 0xFFFF - n
    _ -> Nothing

runOperator :: OperatorMap -> Operator -> Maybe Word16
runOperator m (OpId o) = runOperand m o
runOperator m (OpAnd a b) = runAnd m a b
runOperator m (OpOr a b) = runOr m a b
runOperator m (OpLshift a b) = runLshift m a b
runOperator m (OpRshift a b) = runRshift m a b
runOperator m (OpNot o) = runNot m o
runOperator _ _ = Nothing

-- type MemoMap Map.Map Wire Value
-- runComputer' :: String -> OperatorMap -> MemoMap -> Maybe Word16

runComputer :: String -> OperatorMap -> Maybe Word16
runComputer w m = case (Map.lookup w m) of
    Just op -> trace ("" ++ w ++ " " ++ show op) (runOperator m op)
    Nothing -> Nothing

processor :: Wire -> String -> String
processor w s = case (runComputer w $ parseComputer s) of
    Just r -> show r
    _ -> "Unknown"

main = interact $ processor "a"

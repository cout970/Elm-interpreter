module TypeCheck exposing (..)

-- Default Imports
import Basics exposing (..)
import List exposing ((::))
import Maybe exposing (Maybe(..))
import Result exposing (Result(..))
import String exposing (String)
import Char exposing (Char)
import Tuple

import Debug

--import Platform exposing ( Program )
--import Platform.Cmd as Cmd exposing ( Cmd )
--import Platform.Sub as Sub exposing ( Sub )
--

func0: List Int
func0 = [1,2,3]

func1: Int
func1 = 0

func2: Int -> Int
func2 x = x

func3 a b = a * b

func4 a = func2 a

func5 a b = (func2 a) + b

func6 x = if x then 1 else 2

func7 x = if x then 1.5 else 2

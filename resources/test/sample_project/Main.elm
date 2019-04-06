module Main exposing (..)

import SubModule1 exposing (hello)
import Mod.SubModule2 exposing (world)

sayHello : Int -> String
sayHello a = hello ++ " " ++ world

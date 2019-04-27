module Main exposing (..)

import SubModule1 exposing (hello)
import Mod.SubModule2 as S exposing (world)
import Mod.SubModule2

sayHello = hello ++ " " ++ world
module Main exposing (..)

import SubModule1 exposing (hello)
import Mod.SubModule2 exposing (world)

sayHello = hello ++ " " ++ world

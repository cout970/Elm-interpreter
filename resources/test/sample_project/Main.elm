module Main exposing (..)

import SubModule1 exposing (hello)
import Mod.SubModule2 as S exposing (world)
import Mod.SubModule2

sayHello = hello ++ " " ++ world

sayHello2 = hello
sayHello3 = SubModule1.hello
sayHello4 = S.world
sayHello5 = Mod.SubModule2.world
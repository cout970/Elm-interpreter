module Main exposing (..)

import Browser
import Html exposing (Html, button, div, text)
import Html.Events exposing (onClick)

update : Msg -> Model -> Model
update msg model =
  case msg of
    Increment ->
        model + 1
    Decrement ->
        model - 1

main =
  Browser.sandbox { init = 0, update = update, view = view }

type alias Model a b c = Int (List a)

type Msg
    = Increment
    | Decrement

view model =
  div []
    [ button [ onClick Decrement ] [ text "-" ]
    , div [] [ text (String.fromInt model) ]
    , button [ onClick Increment ] [ text "+" ]
    ]

view : Model -> Int
view model =
    1

example { x, y } = x + y
example [ x, y ] = x + y
example x y = x + y

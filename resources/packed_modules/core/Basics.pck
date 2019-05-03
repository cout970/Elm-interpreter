{
  "header": {
    "name": "Basics",
    "exposing": {
      "Just": [
        {
          "Type": "Int"
        },
        {
          "Type": "Float"
        },
        {
          "BinaryOperator": "+"
        },
        {
          "BinaryOperator": "-"
        },
        {
          "BinaryOperator": "*"
        },
        {
          "BinaryOperator": "/"
        },
        {
          "BinaryOperator": "//"
        },
        {
          "BinaryOperator": "^"
        },
        {
          "Definition": "toFloat"
        },
        {
          "Definition": "round"
        },
        {
          "Definition": "floor"
        },
        {
          "Definition": "ceiling"
        },
        {
          "Definition": "truncate"
        },
        {
          "BinaryOperator": "=="
        },
        {
          "BinaryOperator": "/="
        },
        {
          "BinaryOperator": "<"
        },
        {
          "BinaryOperator": ">"
        },
        {
          "BinaryOperator": "<="
        },
        {
          "BinaryOperator": ">="
        },
        {
          "Definition": "max"
        },
        {
          "Definition": "min"
        },
        {
          "Definition": "compare"
        },
        {
          "Adt": [
            "Order",
            "All"
          ]
        },
        {
          "Adt": [
            "Bool",
            "All"
          ]
        },
        {
          "Definition": "not"
        },
        {
          "BinaryOperator": "&&"
        },
        {
          "BinaryOperator": "||"
        },
        {
          "Definition": "xor"
        },
        {
          "BinaryOperator": "++"
        },
        {
          "Definition": "modBy"
        },
        {
          "Definition": "remainderBy"
        },
        {
          "Definition": "negate"
        },
        {
          "Definition": "abs"
        },
        {
          "Definition": "clamp"
        },
        {
          "Definition": "sqrt"
        },
        {
          "Definition": "logBase"
        },
        {
          "Definition": "e"
        },
        {
          "Definition": "pi"
        },
        {
          "Definition": "cos"
        },
        {
          "Definition": "sin"
        },
        {
          "Definition": "tan"
        },
        {
          "Definition": "acos"
        },
        {
          "Definition": "asin"
        },
        {
          "Definition": "atan"
        },
        {
          "Definition": "atan2"
        },
        {
          "Definition": "degrees"
        },
        {
          "Definition": "radians"
        },
        {
          "Definition": "turns"
        },
        {
          "Definition": "toPolar"
        },
        {
          "Definition": "fromPolar"
        },
        {
          "Definition": "isNaN"
        },
        {
          "Definition": "isInfinite"
        },
        {
          "Definition": "identity"
        },
        {
          "Definition": "always"
        },
        {
          "BinaryOperator": "<|"
        },
        {
          "BinaryOperator": "|>"
        },
        {
          "BinaryOperator": "<<"
        },
        {
          "BinaryOperator": ">>"
        },
        {
          "Type": "Never"
        },
        {
          "Definition": "never"
        }
      ]
    }
  },
  "imports": [
    {
      "path": [
        "Elm",
        "Kernel",
        "Basics"
      ],
      "alias": null,
      "exposing": null
    },
    {
      "path": [
        "Elm",
        "Kernel",
        "Utils"
      ],
      "alias": null,
      "exposing": null
    }
  ],
  "statements": [
    {
      "Infix": [
        "right",
        0,
        "<|",
        "apL"
      ]
    },
    {
      "Infix": [
        "left",
        0,
        "|>",
        "apR"
      ]
    },
    {
      "Infix": [
        "right",
        2,
        "||",
        "or"
      ]
    },
    {
      "Infix": [
        "right",
        3,
        "&&",
        "and"
      ]
    },
    {
      "Infix": [
        "non",
        4,
        "==",
        "eq"
      ]
    },
    {
      "Infix": [
        "non",
        4,
        "/=",
        "neq"
      ]
    },
    {
      "Infix": [
        "non",
        4,
        "<",
        "lt"
      ]
    },
    {
      "Infix": [
        "non",
        4,
        ">",
        "gt"
      ]
    },
    {
      "Infix": [
        "non",
        4,
        "<=",
        "le"
      ]
    },
    {
      "Infix": [
        "non",
        4,
        ">=",
        "ge"
      ]
    },
    {
      "Infix": [
        "right",
        5,
        "++",
        "append"
      ]
    },
    {
      "Infix": [
        "left",
        6,
        "+",
        "add"
      ]
    },
    {
      "Infix": [
        "left",
        6,
        "-",
        "sub"
      ]
    },
    {
      "Infix": [
        "left",
        7,
        "*",
        "mul"
      ]
    },
    {
      "Infix": [
        "left",
        7,
        "/",
        "fdiv"
      ]
    },
    {
      "Infix": [
        "left",
        7,
        "//",
        "idiv"
      ]
    },
    {
      "Infix": [
        "right",
        8,
        "^",
        "pow"
      ]
    },
    {
      "Infix": [
        "left",
        9,
        "<<",
        "composeL"
      ]
    },
    {
      "Infix": [
        "right",
        9,
        ">>",
        "composeR"
      ]
    },
    {
      "Adt": [
        "Int",
        [],
        [
          [
            [
              2986,
              2989
            ],
            "Int",
            []
          ]
        ]
      ]
    },
    {
      "Adt": [
        "Float",
        [],
        [
          [
            [
              3781,
              3786
            ],
            "Float",
            []
          ]
        ]
      ]
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "number"
            },
            {
              "Fun": [
                {
                  "Var": "number"
                },
                {
                  "Var": "number"
                }
              ]
            }
          ]
        },
        "name": "add",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              4855,
              4876
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "add"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "number"
            },
            {
              "Fun": [
                {
                  "Var": "number"
                },
                {
                  "Var": "number"
                }
              ]
            }
          ]
        },
        "name": "sub",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              5020,
              5041
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "sub"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "number"
            },
            {
              "Fun": [
                {
                  "Var": "number"
                },
                {
                  "Var": "number"
                }
              ]
            }
          ]
        },
        "name": "mul",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              5185,
              5206
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "mul"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Float",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Float",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "fdiv",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              5304,
              5326
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "fdiv"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Int",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Int",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "idiv",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              5446,
              5468
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "idiv"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "number"
            },
            {
              "Fun": [
                {
                  "Var": "number"
                },
                {
                  "Var": "number"
                }
              ]
            }
          ]
        },
        "name": "pow",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              5562,
              5583
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "pow"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Tag": [
                "Float",
                []
              ]
            }
          ]
        },
        "name": "toFloat",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              5823,
              5848
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "toFloat"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Tag": [
                "Int",
                []
              ]
            }
          ]
        },
        "name": "round",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              6069,
              6092
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "round"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Tag": [
                "Int",
                []
              ]
            }
          ]
        },
        "name": "floor",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              6305,
              6328
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "floor"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Tag": [
                "Int",
                []
              ]
            }
          ]
        },
        "name": "ceiling",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              6559,
              6584
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "ceiling"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Tag": [
                "Int",
                []
              ]
            }
          ]
        },
        "name": "truncate",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              6835,
              6861
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "truncate"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "a"
            },
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "eq",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              7899,
              7921
            ],
            [
              "Elm",
              "Kernel",
              "Utils"
            ],
            "equal"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "a"
            },
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "neq",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              8056,
              8081
            ],
            [
              "Elm",
              "Kernel",
              "Utils"
            ],
            "notEqual"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "comparable"
            },
            {
              "Fun": [
                {
                  "Var": "comparable"
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "lt",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              8153,
              8172
            ],
            [
              "Elm",
              "Kernel",
              "Utils"
            ],
            "lt"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "comparable"
            },
            {
              "Fun": [
                {
                  "Var": "comparable"
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "gt",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              8226,
              8245
            ],
            [
              "Elm",
              "Kernel",
              "Utils"
            ],
            "gt"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "comparable"
            },
            {
              "Fun": [
                {
                  "Var": "comparable"
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "le",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              8299,
              8318
            ],
            [
              "Elm",
              "Kernel",
              "Utils"
            ],
            "le"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "comparable"
            },
            {
              "Fun": [
                {
                  "Var": "comparable"
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "ge",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              8372,
              8391
            ],
            [
              "Elm",
              "Kernel",
              "Utils"
            ],
            "ge"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "comparable"
            },
            {
              "Fun": [
                {
                  "Var": "comparable"
                },
                {
                  "Var": "comparable"
                }
              ]
            }
          ]
        },
        "name": "min",
        "patterns": [
          {
            "Var": [
              [
                8543,
                8544
              ],
              "x"
            ]
          },
          {
            "Var": [
              [
                8545,
                8546
              ],
              "y"
            ]
          }
        ],
        "expr": {
          "If": [
            [
              8551,
              8574
            ],
            {
              "Application": [
                [
                  8554,
                  8560
                ],
                {
                  "Application": [
                    [
                      8554,
                      8560
                    ],
                    {
                      "Ref": [
                        [
                          8554,
                          8556
                        ],
                        "lt"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          8557,
                          8558
                        ],
                        "x"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      8559,
                      8560
                    ],
                    "y"
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  8566,
                  8567
                ],
                "x"
              ]
            },
            {
              "Ref": [
                [
                  8573,
                  8574
                ],
                "y"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "comparable"
            },
            {
              "Fun": [
                {
                  "Var": "comparable"
                },
                {
                  "Var": "comparable"
                }
              ]
            }
          ]
        },
        "name": "max",
        "patterns": [
          {
            "Var": [
              [
                8731,
                8732
              ],
              "x"
            ]
          },
          {
            "Var": [
              [
                8733,
                8734
              ],
              "y"
            ]
          }
        ],
        "expr": {
          "If": [
            [
              8739,
              8762
            ],
            {
              "Application": [
                [
                  8742,
                  8748
                ],
                {
                  "Application": [
                    [
                      8742,
                      8748
                    ],
                    {
                      "Ref": [
                        [
                          8742,
                          8744
                        ],
                        "gt"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          8745,
                          8746
                        ],
                        "x"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      8747,
                      8748
                    ],
                    "y"
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  8754,
                  8755
                ],
                "x"
              ]
            },
            {
              "Ref": [
                [
                  8761,
                  8762
                ],
                "y"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "comparable"
            },
            {
              "Fun": [
                {
                  "Var": "comparable"
                },
                {
                  "Tag": [
                    "Order",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "compare",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              9113,
              9137
            ],
            [
              "Elm",
              "Kernel",
              "Utils"
            ],
            "compare"
          ]
        }
      }
    },
    {
      "Adt": [
        "Order",
        [],
        [
          [
            [
              9265,
              9267
            ],
            "LT",
            []
          ],
          [
            [
              9270,
              9272
            ],
            "EQ",
            []
          ],
          [
            [
              9275,
              9277
            ],
            "GT",
            []
          ]
        ]
      ]
    },
    {
      "Adt": [
        "Bool",
        [],
        [
          [
            [
              9771,
              9775
            ],
            "True",
            []
          ],
          [
            [
              9778,
              9783
            ],
            "False",
            []
          ]
        ]
      ]
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Bool",
                []
              ]
            },
            {
              "Tag": [
                "Bool",
                []
              ]
            }
          ]
        },
        "name": "not",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              9889,
              9910
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "not"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Bool",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "and",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              10325,
              10346
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "and"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Bool",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "or",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              10761,
              10781
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "or"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Bool",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "xor",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              11007,
              11028
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "xor"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "appendable"
            },
            {
              "Fun": [
                {
                  "Var": "appendable"
                },
                {
                  "Var": "appendable"
                }
              ]
            }
          ]
        },
        "name": "append",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              11266,
              11289
            ],
            [
              "Elm",
              "Kernel",
              "Utils"
            ],
            "append"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Int",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Int",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "modBy",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              12091,
              12114
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "modBy"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Int",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Int",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "remainderBy",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              12671,
              12700
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "remainderBy"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "number"
            },
            {
              "Var": "number"
            }
          ]
        },
        "name": "negate",
        "patterns": [
          {
            "Var": [
              [
                12821,
                12822
              ],
              "n"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              12827,
              12829
            ],
            {
              "Ref": [
                [
                  12827,
                  12828
                ],
                "__internal__minus"
              ]
            },
            {
              "Ref": [
                [
                  12828,
                  12829
                ],
                "n"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "number"
            },
            {
              "Var": "number"
            }
          ]
        },
        "name": "abs",
        "patterns": [
          {
            "Var": [
              [
                13041,
                13042
              ],
              "n"
            ]
          }
        ],
        "expr": {
          "If": [
            [
              13047,
              13071
            ],
            {
              "Application": [
                [
                  13050,
                  13056
                ],
                {
                  "Application": [
                    [
                      13050,
                      13056
                    ],
                    {
                      "Ref": [
                        [
                          13050,
                          13052
                        ],
                        "lt"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          13053,
                          13054
                        ],
                        "n"
                      ]
                    }
                  ]
                },
                {
                  "Literal": [
                    [
                      13055,
                      13056
                    ],
                    {
                      "Int": 0
                    }
                  ]
                }
              ]
            },
            {
              "Application": [
                [
                  13062,
                  13065
                ],
                {
                  "Ref": [
                    [
                      13062,
                      13063
                    ],
                    "__internal__minus"
                  ]
                },
                {
                  "Ref": [
                    [
                      13063,
                      13064
                    ],
                    "n"
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  13070,
                  13071
                ],
                "n"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "number"
            },
            {
              "Fun": [
                {
                  "Var": "number"
                },
                {
                  "Fun": [
                    {
                      "Var": "number"
                    },
                    {
                      "Var": "number"
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "clamp",
        "patterns": [
          {
            "Var": [
              [
                13314,
                13317
              ],
              "low"
            ]
          },
          {
            "Var": [
              [
                13318,
                13322
              ],
              "high"
            ]
          },
          {
            "Var": [
              [
                13323,
                13329
              ],
              "number"
            ]
          }
        ],
        "expr": {
          "If": [
            [
              13334,
              13420
            ],
            {
              "Application": [
                [
                  13337,
                  13350
                ],
                {
                  "Application": [
                    [
                      13337,
                      13350
                    ],
                    {
                      "Ref": [
                        [
                          13337,
                          13339
                        ],
                        "lt"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          13340,
                          13346
                        ],
                        "number"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      13347,
                      13350
                    ],
                    "low"
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  13360,
                  13363
                ],
                "low"
              ]
            },
            {
              "If": [
                [
                  13371,
                  13420
                ],
                {
                  "Application": [
                    [
                      13374,
                      13388
                    ],
                    {
                      "Application": [
                        [
                          13374,
                          13388
                        ],
                        {
                          "Ref": [
                            [
                              13374,
                              13376
                            ],
                            "gt"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              13377,
                              13383
                            ],
                            "number"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          13384,
                          13388
                        ],
                        "high"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      13398,
                      13402
                    ],
                    "high"
                  ]
                },
                {
                  "Ref": [
                    [
                      13414,
                      13420
                    ],
                    "number"
                  ]
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Tag": [
                "Float",
                []
              ]
            }
          ]
        },
        "name": "sqrt",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              13564,
              13586
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "sqrt"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Float",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Float",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "logBase",
        "patterns": [
          {
            "Var": [
              [
                13741,
                13745
              ],
              "base"
            ]
          },
          {
            "Var": [
              [
                13746,
                13752
              ],
              "number"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              13757,
              13829
            ],
            {
              "Application": [
                [
                  13757,
                  13829
                ],
                {
                  "Ref": [
                    [
                      13757,
                      13761
                    ],
                    "fdiv"
                  ]
                },
                {
                  "Application": [
                    [
                      13767,
                      13795
                    ],
                    {
                      "QualifiedRef": [
                        [
                          13767,
                          13789
                        ],
                        [
                          "Elm",
                          "Kernel",
                          "Basics"
                        ],
                        "log"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          13789,
                          13795
                        ],
                        "number"
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Application": [
                [
                  13802,
                  13828
                ],
                {
                  "QualifiedRef": [
                    [
                      13802,
                      13824
                    ],
                    [
                      "Elm",
                      "Kernel",
                      "Basics"
                    ],
                    "log"
                  ]
                },
                {
                  "Ref": [
                    [
                      13824,
                      13828
                    ],
                    "base"
                  ]
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Tag": [
            "Float",
            []
          ]
        },
        "name": "e",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              13878,
              13897
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "e"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Tag": [
                "Float",
                []
              ]
            }
          ]
        },
        "name": "radians",
        "patterns": [
          {
            "Var": [
              [
                14040,
                14054
              ],
              "angleInRadians"
            ]
          }
        ],
        "expr": {
          "Ref": [
            [
              14059,
              14073
            ],
            "angleInRadians"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Tag": [
                "Float",
                []
              ]
            }
          ]
        },
        "name": "degrees",
        "patterns": [
          {
            "Var": [
              [
                14204,
                14218
              ],
              "angleInDegrees"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              14223,
              14255
            ],
            {
              "Application": [
                [
                  14223,
                  14255
                ],
                {
                  "Ref": [
                    [
                      14223,
                      14227
                    ],
                    "fdiv"
                  ]
                },
                {
                  "Application": [
                    [
                      14229,
                      14250
                    ],
                    {
                      "Application": [
                        [
                          14229,
                          14250
                        ],
                        {
                          "Ref": [
                            [
                              14229,
                              14232
                            ],
                            "mul"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              14233,
                              14247
                            ],
                            "angleInDegrees"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          14248,
                          14250
                        ],
                        "pi"
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Literal": [
                [
                  14252,
                  14255
                ],
                {
                  "Int": 180
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Tag": [
                "Float",
                []
              ]
            }
          ]
        },
        "name": "turns",
        "patterns": [
          {
            "Var": [
              [
                14408,
                14420
              ],
              "angleInTurns"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              14425,
              14452
            ],
            {
              "Application": [
                [
                  14425,
                  14452
                ],
                {
                  "Ref": [
                    [
                      14425,
                      14428
                    ],
                    "mul"
                  ]
                },
                {
                  "Application": [
                    [
                      14430,
                      14438
                    ],
                    {
                      "Application": [
                        [
                          14430,
                          14438
                        ],
                        {
                          "Ref": [
                            [
                              14430,
                              14433
                            ],
                            "mul"
                          ]
                        },
                        {
                          "Literal": [
                            [
                              14434,
                              14435
                            ],
                            {
                              "Int": 2
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          14436,
                          14438
                        ],
                        "pi"
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  14440,
                  14452
                ],
                "angleInTurns"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Tag": [
            "Float",
            []
          ]
        },
        "name": "pi",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              14523,
              14543
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "pi"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Tag": [
                "Float",
                []
              ]
            }
          ]
        },
        "name": "cos",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              14821,
              14842
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "cos"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Tag": [
                "Float",
                []
              ]
            }
          ]
        },
        "name": "sin",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              15122,
              15143
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "sin"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Tag": [
                "Float",
                []
              ]
            }
          ]
        },
        "name": "tan",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              15421,
              15442
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "tan"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Tag": [
                "Float",
                []
              ]
            }
          ]
        },
        "name": "acos",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              15611,
              15633
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "acos"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Tag": [
                "Float",
                []
              ]
            }
          ]
        },
        "name": "asin",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              15800,
              15822
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "asin"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Tag": [
                "Float",
                []
              ]
            }
          ]
        },
        "name": "atan",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              16779,
              16801
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "atan"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Float",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Float",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "atan2",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              17283,
              17306
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "atan2"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tuple": [
                {
                  "Tag": [
                    "Float",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Float",
                    []
                  ]
                }
              ]
            },
            {
              "Tuple": [
                {
                  "Tag": [
                    "Float",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Float",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "fromPolar",
        "patterns": [
          {
            "Tuple": [
              [
                17509,
                17524
              ],
              [
                {
                  "Var": [
                    [
                      17510,
                      17516
                    ],
                    "radius"
                  ]
                },
                {
                  "Var": [
                    [
                      17518,
                      17523
                    ],
                    "theta"
                  ]
                }
              ]
            ]
          }
        ],
        "expr": {
          "Tuple": [
            [
              17529,
              17584
            ],
            [
              {
                "Application": [
                  [
                    17531,
                    17553
                  ],
                  {
                    "Application": [
                      [
                        17531,
                        17553
                      ],
                      {
                        "Ref": [
                          [
                            17531,
                            17534
                          ],
                          "mul"
                        ]
                      },
                      {
                        "Ref": [
                          [
                            17535,
                            17541
                          ],
                          "radius"
                        ]
                      }
                    ]
                  },
                  {
                    "Application": [
                      [
                        17543,
                        17552
                      ],
                      {
                        "Ref": [
                          [
                            17543,
                            17546
                          ],
                          "cos"
                        ]
                      },
                      {
                        "Ref": [
                          [
                            17547,
                            17552
                          ],
                          "theta"
                        ]
                      }
                    ]
                  }
                ]
              },
              {
                "Application": [
                  [
                    17558,
                    17580
                  ],
                  {
                    "Application": [
                      [
                        17558,
                        17580
                      ],
                      {
                        "Ref": [
                          [
                            17558,
                            17561
                          ],
                          "mul"
                        ]
                      },
                      {
                        "Ref": [
                          [
                            17562,
                            17568
                          ],
                          "radius"
                        ]
                      }
                    ]
                  },
                  {
                    "Application": [
                      [
                        17570,
                        17579
                      ],
                      {
                        "Ref": [
                          [
                            17570,
                            17573
                          ],
                          "sin"
                        ]
                      },
                      {
                        "Ref": [
                          [
                            17574,
                            17579
                          ],
                          "theta"
                        ]
                      }
                    ]
                  }
                ]
              }
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tuple": [
                {
                  "Tag": [
                    "Float",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Float",
                    []
                  ]
                }
              ]
            },
            {
              "Tuple": [
                {
                  "Tag": [
                    "Float",
                    []
                  ]
                },
                {
                  "Tag": [
                    "Float",
                    []
                  ]
                }
              ]
            }
          ]
        },
        "name": "toPolar",
        "patterns": [
          {
            "Tuple": [
              [
                17808,
                17816
              ],
              [
                {
                  "Var": [
                    [
                      17810,
                      17811
                    ],
                    "x"
                  ]
                },
                {
                  "Var": [
                    [
                      17813,
                      17814
                    ],
                    "y"
                  ]
                }
              ]
            ]
          }
        ],
        "expr": {
          "Tuple": [
            [
              17821,
              17871
            ],
            [
              {
                "Application": [
                  [
                    17823,
                    17853
                  ],
                  {
                    "Ref": [
                      [
                        17823,
                        17827
                      ],
                      "sqrt"
                    ]
                  },
                  {
                    "Application": [
                      [
                        17829,
                        17852
                      ],
                      {
                        "Application": [
                          [
                            17829,
                            17852
                          ],
                          {
                            "Ref": [
                              [
                                17829,
                                17832
                              ],
                              "add"
                            ]
                          },
                          {
                            "Application": [
                              [
                                17834,
                                17841
                              ],
                              {
                                "Application": [
                                  [
                                    17834,
                                    17841
                                  ],
                                  {
                                    "Ref": [
                                      [
                                        17834,
                                        17837
                                      ],
                                      "mul"
                                    ]
                                  },
                                  {
                                    "Ref": [
                                      [
                                        17838,
                                        17839
                                      ],
                                      "x"
                                    ]
                                  }
                                ]
                              },
                              {
                                "Ref": [
                                  [
                                    17840,
                                    17841
                                  ],
                                  "x"
                                ]
                              }
                            ]
                          }
                        ]
                      },
                      {
                        "Application": [
                          [
                            17844,
                            17851
                          ],
                          {
                            "Application": [
                              [
                                17844,
                                17851
                              ],
                              {
                                "Ref": [
                                  [
                                    17844,
                                    17847
                                  ],
                                  "mul"
                                ]
                              },
                              {
                                "Ref": [
                                  [
                                    17848,
                                    17849
                                  ],
                                  "y"
                                ]
                              }
                            ]
                          },
                          {
                            "Ref": [
                              [
                                17850,
                                17851
                              ],
                              "y"
                            ]
                          }
                        ]
                      }
                    ]
                  }
                ]
              },
              {
                "Application": [
                  [
                    17858,
                    17867
                  ],
                  {
                    "Application": [
                      [
                        17858,
                        17867
                      ],
                      {
                        "Ref": [
                          [
                            17858,
                            17863
                          ],
                          "atan2"
                        ]
                      },
                      {
                        "Ref": [
                          [
                            17864,
                            17865
                          ],
                          "y"
                        ]
                      }
                    ]
                  },
                  {
                    "Ref": [
                      [
                        17866,
                        17867
                      ],
                      "x"
                    ]
                  }
                ]
              }
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Tag": [
                "Bool",
                []
              ]
            }
          ]
        },
        "name": "isNaN",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              18265,
              18288
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "isNaN"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Float",
                []
              ]
            },
            {
              "Tag": [
                "Bool",
                []
              ]
            }
          ]
        },
        "name": "isInfinite",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              18663,
              18691
            ],
            [
              "Elm",
              "Kernel",
              "Basics"
            ],
            "isInfinite"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "b"
                },
                {
                  "Var": "c"
                }
              ]
            },
            {
              "Fun": [
                {
                  "Fun": [
                    {
                      "Var": "a"
                    },
                    {
                      "Var": "b"
                    }
                  ]
                },
                {
                  "Fun": [
                    {
                      "Var": "a"
                    },
                    {
                      "Var": "c"
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "composeL",
        "patterns": [
          {
            "Var": [
              [
                19138,
                19139
              ],
              "g"
            ]
          },
          {
            "Var": [
              [
                19140,
                19141
              ],
              "f"
            ]
          },
          {
            "Var": [
              [
                19142,
                19143
              ],
              "x"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              19148,
              19155
            ],
            {
              "Ref": [
                [
                  19148,
                  19149
                ],
                "g"
              ]
            },
            {
              "Application": [
                [
                  19151,
                  19154
                ],
                {
                  "Ref": [
                    [
                      19151,
                      19152
                    ],
                    "f"
                  ]
                },
                {
                  "Ref": [
                    [
                      19153,
                      19154
                    ],
                    "x"
                  ]
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Var": "b"
                }
              ]
            },
            {
              "Fun": [
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Var": "c"
                    }
                  ]
                },
                {
                  "Fun": [
                    {
                      "Var": "a"
                    },
                    {
                      "Var": "c"
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "composeR",
        "patterns": [
          {
            "Var": [
              [
                19396,
                19397
              ],
              "f"
            ]
          },
          {
            "Var": [
              [
                19398,
                19399
              ],
              "g"
            ]
          },
          {
            "Var": [
              [
                19400,
                19401
              ],
              "x"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              19406,
              19413
            ],
            {
              "Ref": [
                [
                  19406,
                  19407
                ],
                "g"
              ]
            },
            {
              "Application": [
                [
                  19409,
                  19412
                ],
                {
                  "Ref": [
                    [
                      19409,
                      19410
                    ],
                    "f"
                  ]
                },
                {
                  "Ref": [
                    [
                      19411,
                      19412
                    ],
                    "x"
                  ]
                }
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "a"
            },
            {
              "Fun": [
                {
                  "Fun": [
                    {
                      "Var": "a"
                    },
                    {
                      "Var": "b"
                    }
                  ]
                },
                {
                  "Var": "b"
                }
              ]
            }
          ]
        },
        "name": "apR",
        "patterns": [
          {
            "Var": [
              [
                20468,
                20469
              ],
              "x"
            ]
          },
          {
            "Var": [
              [
                20470,
                20471
              ],
              "f"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              20476,
              20479
            ],
            {
              "Ref": [
                [
                  20476,
                  20477
                ],
                "f"
              ]
            },
            {
              "Ref": [
                [
                  20478,
                  20479
                ],
                "x"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Var": "b"
                }
              ]
            },
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Var": "b"
                }
              ]
            }
          ]
        },
        "name": "apL",
        "patterns": [
          {
            "Var": [
              [
                20708,
                20709
              ],
              "f"
            ]
          },
          {
            "Var": [
              [
                20710,
                20711
              ],
              "x"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              20716,
              20719
            ],
            {
              "Ref": [
                [
                  20716,
                  20717
                ],
                "f"
              ]
            },
            {
              "Ref": [
                [
                  20718,
                  20719
                ],
                "x"
              ]
            }
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "a"
            },
            {
              "Var": "a"
            }
          ]
        },
        "name": "identity",
        "patterns": [
          {
            "Var": [
              [
                20892,
                20893
              ],
              "x"
            ]
          }
        ],
        "expr": {
          "Ref": [
            [
              20898,
              20899
            ],
            "x"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "a"
            },
            {
              "Fun": [
                {
                  "Var": "b"
                },
                {
                  "Var": "a"
                }
              ]
            }
          ]
        },
        "name": "always",
        "patterns": [
          {
            "Var": [
              [
                21161,
                21162
              ],
              "a"
            ]
          },
          {
            "Wildcard": [
              21163,
              21164
            ]
          }
        ],
        "expr": {
          "Ref": [
            [
              21169,
              21170
            ],
            "a"
          ]
        }
      }
    },
    {
      "Adt": [
        "Never",
        [],
        [
          [
            [
              22024,
              22041
            ],
            "JustOneMore",
            [
              {
                "Tag": [
                  "Never",
                  []
                ]
              }
            ]
          ]
        ]
      ]
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Never",
                []
              ]
            },
            {
              "Var": "a"
            }
          ]
        },
        "name": "never",
        "patterns": [
          {
            "Adt": [
              [
                22604,
                22619
              ],
              "JustOneMore",
              [
                {
                  "Var": [
                    [
                      22616,
                      22619
                    ],
                    "nvr"
                  ]
                }
              ]
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              22625,
              22634
            ],
            {
              "Ref": [
                [
                  22625,
                  22630
                ],
                "never"
              ]
            },
            {
              "Ref": [
                [
                  22631,
                  22634
                ],
                "nvr"
              ]
            }
          ]
        }
      }
    }
  ]
}
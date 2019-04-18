{
  "header": {
    "name": "Dict",
    "exposing": {
      "Just": [
        {
          "Type": "Dict"
        },
        {
          "Definition": "empty"
        },
        {
          "Definition": "singleton"
        },
        {
          "Definition": "insert"
        },
        {
          "Definition": "update"
        },
        {
          "Definition": "remove"
        },
        {
          "Definition": "isEmpty"
        },
        {
          "Definition": "member"
        },
        {
          "Definition": "get"
        },
        {
          "Definition": "size"
        },
        {
          "Definition": "keys"
        },
        {
          "Definition": "values"
        },
        {
          "Definition": "toList"
        },
        {
          "Definition": "fromList"
        },
        {
          "Definition": "map"
        },
        {
          "Definition": "foldl"
        },
        {
          "Definition": "foldr"
        },
        {
          "Definition": "filter"
        },
        {
          "Definition": "partition"
        },
        {
          "Definition": "union"
        },
        {
          "Definition": "intersect"
        },
        {
          "Definition": "diff"
        },
        {
          "Definition": "merge"
        }
      ]
    }
  },
  "imports": [
    {
      "path": [
        "Basics"
      ],
      "alias": null,
      "exposing": "All"
    },
    {
      "path": [
        "Maybe"
      ],
      "alias": null,
      "exposing": "All"
    },
    {
      "path": [
        "List"
      ],
      "alias": null,
      "exposing": "All"
    }
  ],
  "statements": [
    {
      "Adt": [
        "NColor",
        [],
        [
          [
            "Red",
            []
          ],
          [
            "Black",
            []
          ]
        ]
      ]
    },
    {
      "Adt": [
        "Dict",
        [
          "k",
          "v"
        ],
        [
          [
            "RBNode_elm_builtin",
            [
              {
                "Tag": [
                  "NColor",
                  [
                    {
                      "Var": "k"
                    },
                    {
                      "Var": "v"
                    },
                    {
                      "Tag": [
                        "Dict",
                        [
                          {
                            "Var": "k"
                          },
                          {
                            "Var": "v"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "Dict",
                        [
                          {
                            "Var": "k"
                          },
                          {
                            "Var": "v"
                          }
                        ]
                      ]
                    }
                  ]
                ]
              }
            ]
          ],
          [
            "RBEmpty_elm_builtin",
            []
          ]
        ]
      ]
    },
    {
      "Def": {
        "header": {
          "Tag": [
            "Dict",
            [
              {
                "Var": "k"
              },
              {
                "Var": "v"
              }
            ]
          ]
        },
        "name": "empty",
        "patterns": [],
        "expr": {
          "Ref": [
            [
              1591,
              1610
            ],
            "RBEmpty_elm_builtin"
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
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "v"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "get",
        "patterns": [
          {
            "Var": "targetKey"
          },
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Case": [
            [
              2015,
              2278
            ],
            {
              "Ref": [
                [
                  2020,
                  2024
                ],
                "dict"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "RBEmpty_elm_builtin",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      2061,
                      2068
                    ],
                    "Nothing"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      "Wildcard",
                      {
                        "Var": "key"
                      },
                      {
                        "Var": "value"
                      },
                      {
                        "Var": "left"
                      },
                      {
                        "Var": "right"
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      2125,
                      2278
                    ],
                    {
                      "Application": [
                        [
                          2130,
                          2151
                        ],
                        {
                          "Application": [
                            [
                              2130,
                              2151
                            ],
                            {
                              "Ref": [
                                [
                                  2130,
                                  2137
                                ],
                                "compare"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  2138,
                                  2147
                                ],
                                "targetKey"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              2148,
                              2151
                            ],
                            "key"
                          ]
                        }
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            "LT",
                            []
                          ]
                        },
                        {
                          "Application": [
                            [
                              2179,
                              2197
                            ],
                            {
                              "Application": [
                                [
                                  2179,
                                  2197
                                ],
                                {
                                  "Ref": [
                                    [
                                      2179,
                                      2182
                                    ],
                                    "get"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      2183,
                                      2192
                                    ],
                                    "targetKey"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  2193,
                                  2197
                                ],
                                "left"
                              ]
                            }
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            "EQ",
                            []
                          ]
                        },
                        {
                          "Application": [
                            [
                              2223,
                              2233
                            ],
                            {
                              "Ref": [
                                [
                                  2223,
                                  2228
                                ],
                                "Just"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  2228,
                                  2233
                                ],
                                "value"
                              ]
                            }
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            "GT",
                            []
                          ]
                        },
                        {
                          "Application": [
                            [
                              2259,
                              2278
                            ],
                            {
                              "Application": [
                                [
                                  2259,
                                  2278
                                ],
                                {
                                  "Ref": [
                                    [
                                      2259,
                                      2262
                                    ],
                                    "get"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      2263,
                                      2272
                                    ],
                                    "targetKey"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  2273,
                                  2278
                                ],
                                "right"
                              ]
                            }
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ]
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
              "Var": "comparable"
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
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
        "name": "member",
        "patterns": [
          {
            "Var": "key"
          },
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Case": [
            [
              2396,
              2469
            ],
            {
              "Application": [
                [
                  2401,
                  2413
                ],
                {
                  "Application": [
                    [
                      2401,
                      2413
                    ],
                    {
                      "Ref": [
                        [
                          2401,
                          2404
                        ],
                        "get"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          2405,
                          2408
                        ],
                        "key"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      2409,
                      2413
                    ],
                    "dict"
                  ]
                }
              ]
            },
            [
              [
                {
                  "Adt": [
                    "Just",
                    [
                      "Wildcard"
                    ]
                  ]
                },
                {
                  "Ref": [
                    [
                      2437,
                      2441
                    ],
                    "True"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      2464,
                      2469
                    ],
                    "False"
                  ]
                }
              ]
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
                "Dict",
                [
                  {
                    "Var": "k"
                  },
                  {
                    "Var": "v"
                  }
                ]
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
        "name": "size",
        "patterns": [
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Application": [
            [
              2575,
              2590
            ],
            {
              "Application": [
                [
                  2575,
                  2590
                ],
                {
                  "Ref": [
                    [
                      2575,
                      2583
                    ],
                    "sizeHelp"
                  ]
                },
                {
                  "Literal": [
                    [
                      2584,
                      2585
                    ],
                    {
                      "Int": 0
                    }
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  2586,
                  2590
                ],
                "dict"
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
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "k"
                      },
                      {
                        "Var": "v"
                      }
                    ]
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
        "name": "sizeHelp",
        "patterns": [
          {
            "Var": "n"
          },
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Case": [
            [
              2647,
              2781
            ],
            {
              "Ref": [
                [
                  2652,
                  2656
                ],
                "dict"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "RBEmpty_elm_builtin",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      2693,
                      2694
                    ],
                    "n"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      "Wildcard",
                      "Wildcard",
                      "Wildcard",
                      {
                        "Var": "left"
                      },
                      {
                        "Var": "right"
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      2745,
                      2781
                    ],
                    {
                      "Application": [
                        [
                          2745,
                          2781
                        ],
                        {
                          "Ref": [
                            [
                              2745,
                              2753
                            ],
                            "sizeHelp"
                          ]
                        },
                        {
                          "Application": [
                            [
                              2755,
                              2775
                            ],
                            {
                              "Application": [
                                [
                                  2755,
                                  2775
                                ],
                                {
                                  "Ref": [
                                    [
                                      2755,
                                      2763
                                    ],
                                    "sizeHelp"
                                  ]
                                },
                                {
                                  "OpChain": [
                                    [
                                      2765,
                                      2768
                                    ],
                                    [
                                      {
                                        "Ref": [
                                          [
                                            2765,
                                            2766
                                          ],
                                          "n"
                                        ]
                                      },
                                      {
                                        "Literal": [
                                          [
                                            2767,
                                            2768
                                          ],
                                          {
                                            "Int": 1
                                          }
                                        ]
                                      }
                                    ],
                                    [
                                      "+"
                                    ]
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  2770,
                                  2775
                                ],
                                "right"
                              ]
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          2777,
                          2781
                        ],
                        "left"
                      ]
                    }
                  ]
                }
              ]
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
                "Dict",
                [
                  {
                    "Var": "k"
                  },
                  {
                    "Var": "v"
                  }
                ]
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
        "name": "isEmpty",
        "patterns": [
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Case": [
            [
              2898,
              2997
            ],
            {
              "Ref": [
                [
                  2903,
                  2907
                ],
                "dict"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "RBEmpty_elm_builtin",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      2944,
                      2948
                    ],
                    "True"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      "Wildcard",
                      "Wildcard",
                      "Wildcard",
                      "Wildcard",
                      "Wildcard"
                    ]
                  ]
                },
                {
                  "Ref": [
                    [
                      2992,
                      2997
                    ],
                    "False"
                  ]
                }
              ]
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
              "Var": "comparable"
            },
            {
              "Fun": [
                {
                  "Var": "v"
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Dict",
                        [
                          {
                            "Var": "comparable"
                          },
                          {
                            "Var": "v"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "Dict",
                        [
                          {
                            "Var": "comparable"
                          },
                          {
                            "Var": "v"
                          }
                        ]
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "insert",
        "patterns": [
          {
            "Var": "key"
          },
          {
            "Var": "value"
          },
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Case": [
            [
              3216,
              3344
            ],
            {
              "Application": [
                [
                  3221,
                  3246
                ],
                {
                  "Application": [
                    [
                      3221,
                      3246
                    ],
                    {
                      "Application": [
                        [
                          3221,
                          3246
                        ],
                        {
                          "Ref": [
                            [
                              3221,
                              3231
                            ],
                            "insertHelp"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              3232,
                              3235
                            ],
                            "key"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          3236,
                          3241
                        ],
                        "value"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      3242,
                      3246
                    ],
                    "dict"
                  ]
                }
              ]
            },
            [
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      {
                        "Adt": [
                          "Red",
                          [
                            {
                              "Var": "k"
                            },
                            {
                              "Var": "v"
                            },
                            {
                              "Var": "l"
                            },
                            {
                              "Var": "r"
                            }
                          ]
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      3294,
                      3326
                    ],
                    {
                      "Application": [
                        [
                          3294,
                          3326
                        ],
                        {
                          "Application": [
                            [
                              3294,
                              3326
                            ],
                            {
                              "Application": [
                                [
                                  3294,
                                  3326
                                ],
                                {
                                  "Application": [
                                    [
                                      3294,
                                      3326
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          3294,
                                          3313
                                        ],
                                        "RBNode_elm_builtin"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          3313,
                                          3319
                                        ],
                                        "Black"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      3319,
                                      3320
                                    ],
                                    "k"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  3321,
                                  3322
                                ],
                                "v"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              3323,
                              3324
                            ],
                            "l"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          3325,
                          3326
                        ],
                        "r"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Var": "x"
                },
                {
                  "Ref": [
                    [
                      3343,
                      3344
                    ],
                    "x"
                  ]
                }
              ]
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
              "Var": "comparable"
            },
            {
              "Fun": [
                {
                  "Var": "v"
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Dict",
                        [
                          {
                            "Var": "comparable"
                          },
                          {
                            "Var": "v"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "Dict",
                        [
                          {
                            "Var": "comparable"
                          },
                          {
                            "Var": "v"
                          }
                        ]
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "insertHelp",
        "patterns": [
          {
            "Var": "key"
          },
          {
            "Var": "value"
          },
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Case": [
            [
              3448,
              4009
            ],
            {
              "Ref": [
                [
                  3453,
                  3457
                ],
                "dict"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "RBEmpty_elm_builtin",
                    []
                  ]
                },
                {
                  "Application": [
                    [
                      3597,
                      3669
                    ],
                    {
                      "Application": [
                        [
                          3597,
                          3669
                        ],
                        {
                          "Application": [
                            [
                              3597,
                              3669
                            ],
                            {
                              "Application": [
                                [
                                  3597,
                                  3669
                                ],
                                {
                                  "Application": [
                                    [
                                      3597,
                                      3669
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          3597,
                                          3616
                                        ],
                                        "RBNode_elm_builtin"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          3616,
                                          3620
                                        ],
                                        "Red"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      3620,
                                      3623
                                    ],
                                    "key"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  3624,
                                  3629
                                ],
                                "value"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              3630,
                              3650
                            ],
                            "RBEmpty_elm_builtin"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          3650,
                          3669
                        ],
                        "RBEmpty_elm_builtin"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      {
                        "Var": "nColor"
                      },
                      {
                        "Var": "nKey"
                      },
                      {
                        "Var": "nValue"
                      },
                      {
                        "Var": "nLeft"
                      },
                      {
                        "Var": "nRight"
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      3735,
                      4009
                    ],
                    {
                      "Application": [
                        [
                          3740,
                          3756
                        ],
                        {
                          "Application": [
                            [
                              3740,
                              3756
                            ],
                            {
                              "Ref": [
                                [
                                  3740,
                                  3747
                                ],
                                "compare"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  3748,
                                  3751
                                ],
                                "key"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              3752,
                              3756
                            ],
                            "nKey"
                          ]
                        }
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            "LT",
                            []
                          ]
                        },
                        {
                          "Application": [
                            [
                              3784,
                              3846
                            ],
                            {
                              "Application": [
                                [
                                  3784,
                                  3846
                                ],
                                {
                                  "Application": [
                                    [
                                      3784,
                                      3846
                                    ],
                                    {
                                      "Application": [
                                        [
                                          3784,
                                          3846
                                        ],
                                        {
                                          "Application": [
                                            [
                                              3784,
                                              3846
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  3784,
                                                  3791
                                                ],
                                                "balance"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  3792,
                                                  3798
                                                ],
                                                "nColor"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              3799,
                                              3803
                                            ],
                                            "nKey"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          3804,
                                          3810
                                        ],
                                        "nValue"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      3812,
                                      3838
                                    ],
                                    {
                                      "Application": [
                                        [
                                          3812,
                                          3838
                                        ],
                                        {
                                          "Application": [
                                            [
                                              3812,
                                              3838
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  3812,
                                                  3822
                                                ],
                                                "insertHelp"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  3823,
                                                  3826
                                                ],
                                                "key"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              3827,
                                              3832
                                            ],
                                            "value"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          3833,
                                          3838
                                        ],
                                        "nLeft"
                                      ]
                                    }
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  3840,
                                  3846
                                ],
                                "nRight"
                              ]
                            }
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            "EQ",
                            []
                          ]
                        },
                        {
                          "Application": [
                            [
                              3872,
                              3921
                            ],
                            {
                              "Application": [
                                [
                                  3872,
                                  3921
                                ],
                                {
                                  "Application": [
                                    [
                                      3872,
                                      3921
                                    ],
                                    {
                                      "Application": [
                                        [
                                          3872,
                                          3921
                                        ],
                                        {
                                          "Application": [
                                            [
                                              3872,
                                              3921
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  3872,
                                                  3891
                                                ],
                                                "RBNode_elm_builtin"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  3891,
                                                  3897
                                                ],
                                                "nColor"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              3898,
                                              3902
                                            ],
                                            "nKey"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          3903,
                                          3908
                                        ],
                                        "value"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      3909,
                                      3914
                                    ],
                                    "nLeft"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  3915,
                                  3921
                                ],
                                "nRight"
                              ]
                            }
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            "GT",
                            []
                          ]
                        },
                        {
                          "Application": [
                            [
                              3947,
                              4009
                            ],
                            {
                              "Application": [
                                [
                                  3947,
                                  4009
                                ],
                                {
                                  "Application": [
                                    [
                                      3947,
                                      4009
                                    ],
                                    {
                                      "Application": [
                                        [
                                          3947,
                                          4009
                                        ],
                                        {
                                          "Application": [
                                            [
                                              3947,
                                              4009
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  3947,
                                                  3954
                                                ],
                                                "balance"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  3955,
                                                  3961
                                                ],
                                                "nColor"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              3962,
                                              3966
                                            ],
                                            "nKey"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          3967,
                                          3973
                                        ],
                                        "nValue"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      3974,
                                      3979
                                    ],
                                    "nLeft"
                                  ]
                                }
                              ]
                            },
                            {
                              "Application": [
                                [
                                  3981,
                                  4008
                                ],
                                {
                                  "Application": [
                                    [
                                      3981,
                                      4008
                                    ],
                                    {
                                      "Application": [
                                        [
                                          3981,
                                          4008
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              3981,
                                              3991
                                            ],
                                            "insertHelp"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              3992,
                                              3995
                                            ],
                                            "key"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          3996,
                                          4001
                                        ],
                                        "value"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      4002,
                                      4008
                                    ],
                                    "nRight"
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ]
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
                "NColor",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Var": "k"
                },
                {
                  "Fun": [
                    {
                      "Var": "v"
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "Dict",
                            [
                              {
                                "Var": "k"
                              },
                              {
                                "Var": "v"
                              }
                            ]
                          ]
                        },
                        {
                          "Fun": [
                            {
                              "Tag": [
                                "Dict",
                                [
                                  {
                                    "Var": "k"
                                  },
                                  {
                                    "Var": "v"
                                  }
                                ]
                              ]
                            },
                            {
                              "Tag": [
                                "Dict",
                                [
                                  {
                                    "Var": "k"
                                  },
                                  {
                                    "Var": "v"
                                  }
                                ]
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "balance",
        "patterns": [
          {
            "Var": "color"
          },
          {
            "Var": "key"
          },
          {
            "Var": "value"
          },
          {
            "Var": "left"
          },
          {
            "Var": "right"
          }
        ],
        "expr": {
          "Case": [
            [
              4114,
              4944
            ],
            {
              "Ref": [
                [
                  4119,
                  4124
                ],
                "right"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      {
                        "Adt": [
                          "Red",
                          [
                            {
                              "Var": "rK"
                            },
                            {
                              "Var": "rV"
                            },
                            {
                              "Var": "rLeft"
                            },
                            {
                              "Var": "rRight"
                            }
                          ]
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      4183,
                      4551
                    ],
                    {
                      "Ref": [
                        [
                          4188,
                          4192
                        ],
                        "left"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            "RBNode_elm_builtin",
                            [
                              {
                                "Adt": [
                                  "Red",
                                  [
                                    {
                                      "Var": "lK"
                                    },
                                    {
                                      "Var": "lV"
                                    },
                                    {
                                      "Var": "lLeft"
                                    },
                                    {
                                      "Var": "lRight"
                                    }
                                  ]
                                ]
                              }
                            ]
                          ]
                        },
                        {
                          "Application": [
                            [
                              4259,
                              4443
                            ],
                            {
                              "Application": [
                                [
                                  4259,
                                  4443
                                ],
                                {
                                  "Application": [
                                    [
                                      4259,
                                      4443
                                    ],
                                    {
                                      "Application": [
                                        [
                                          4259,
                                          4443
                                        ],
                                        {
                                          "Application": [
                                            [
                                              4259,
                                              4443
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  4259,
                                                  4290
                                                ],
                                                "RBNode_elm_builtin"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  4290,
                                                  4306
                                                ],
                                                "Red"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              4306,
                                              4309
                                            ],
                                            "key"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          4322,
                                          4327
                                        ],
                                        "value"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      4341,
                                      4384
                                    ],
                                    {
                                      "Application": [
                                        [
                                          4341,
                                          4384
                                        ],
                                        {
                                          "Application": [
                                            [
                                              4341,
                                              4384
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  4341,
                                                  4384
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      4341,
                                                      4384
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          4341,
                                                          4360
                                                        ],
                                                        "RBNode_elm_builtin"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          4360,
                                                          4366
                                                        ],
                                                        "Black"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      4366,
                                                      4368
                                                    ],
                                                    "lK"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  4369,
                                                  4371
                                                ],
                                                "lV"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              4372,
                                              4377
                                            ],
                                            "lLeft"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          4378,
                                          4384
                                        ],
                                        "lRight"
                                      ]
                                    }
                                  ]
                                }
                              ]
                            },
                            {
                              "Application": [
                                [
                                  4399,
                                  4442
                                ],
                                {
                                  "Application": [
                                    [
                                      4399,
                                      4442
                                    ],
                                    {
                                      "Application": [
                                        [
                                          4399,
                                          4442
                                        ],
                                        {
                                          "Application": [
                                            [
                                              4399,
                                              4442
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  4399,
                                                  4442
                                                ],
                                                {
                                                  "Ref": [
                                                    [
                                                      4399,
                                                      4418
                                                    ],
                                                    "RBNode_elm_builtin"
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      4418,
                                                      4424
                                                    ],
                                                    "Black"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  4424,
                                                  4426
                                                ],
                                                "rK"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              4427,
                                              4429
                                            ],
                                            "rV"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          4430,
                                          4435
                                        ],
                                        "rLeft"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      4436,
                                      4442
                                    ],
                                    "rRight"
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ],
                      [
                        "Wildcard",
                        {
                          "Application": [
                            [
                              4468,
                              4551
                            ],
                            {
                              "Application": [
                                [
                                  4468,
                                  4551
                                ],
                                {
                                  "Application": [
                                    [
                                      4468,
                                      4551
                                    ],
                                    {
                                      "Application": [
                                        [
                                          4468,
                                          4551
                                        ],
                                        {
                                          "Application": [
                                            [
                                              4468,
                                              4551
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  4468,
                                                  4487
                                                ],
                                                "RBNode_elm_builtin"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  4487,
                                                  4492
                                                ],
                                                "color"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              4493,
                                              4495
                                            ],
                                            "rK"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          4496,
                                          4498
                                        ],
                                        "rV"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      4500,
                                      4543
                                    ],
                                    {
                                      "Application": [
                                        [
                                          4500,
                                          4543
                                        ],
                                        {
                                          "Application": [
                                            [
                                              4500,
                                              4543
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  4500,
                                                  4543
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      4500,
                                                      4543
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          4500,
                                                          4519
                                                        ],
                                                        "RBNode_elm_builtin"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          4519,
                                                          4523
                                                        ],
                                                        "Red"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      4523,
                                                      4526
                                                    ],
                                                    "key"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  4527,
                                                  4532
                                                ],
                                                "value"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              4533,
                                              4537
                                            ],
                                            "left"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          4538,
                                          4543
                                        ],
                                        "rLeft"
                                      ]
                                    }
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  4545,
                                  4551
                                ],
                                "rRight"
                              ]
                            }
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ],
              [
                "Wildcard",
                {
                  "Case": [
                    [
                      4568,
                      4944
                    ],
                    {
                      "Ref": [
                        [
                          4573,
                          4577
                        ],
                        "left"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            "RBNode_elm_builtin",
                            [
                              {
                                "Adt": [
                                  "Red",
                                  [
                                    {
                                      "Var": "lK"
                                    },
                                    {
                                      "Var": "lV"
                                    },
                                    {
                                      "Adt": [
                                        "RBNode_elm_builtin",
                                        [
                                          {
                                            "Adt": [
                                              "Red",
                                              [
                                                {
                                                  "Var": "llK"
                                                },
                                                {
                                                  "Var": "llV"
                                                },
                                                {
                                                  "Var": "llLeft"
                                                },
                                                {
                                                  "Var": "llRight"
                                                }
                                              ]
                                            ]
                                          }
                                        ]
                                      ]
                                    },
                                    {
                                      "Var": "lRight"
                                    }
                                  ]
                                ]
                              }
                            ]
                          ]
                        },
                        {
                          "Application": [
                            [
                              4686,
                              4874
                            ],
                            {
                              "Application": [
                                [
                                  4686,
                                  4874
                                ],
                                {
                                  "Application": [
                                    [
                                      4686,
                                      4874
                                    ],
                                    {
                                      "Application": [
                                        [
                                          4686,
                                          4874
                                        ],
                                        {
                                          "Application": [
                                            [
                                              4686,
                                              4874
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  4686,
                                                  4717
                                                ],
                                                "RBNode_elm_builtin"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  4717,
                                                  4733
                                                ],
                                                "Red"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              4733,
                                              4735
                                            ],
                                            "lK"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          4748,
                                          4750
                                        ],
                                        "lV"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      4764,
                                      4811
                                    ],
                                    {
                                      "Application": [
                                        [
                                          4764,
                                          4811
                                        ],
                                        {
                                          "Application": [
                                            [
                                              4764,
                                              4811
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  4764,
                                                  4811
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      4764,
                                                      4811
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          4764,
                                                          4783
                                                        ],
                                                        "RBNode_elm_builtin"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          4783,
                                                          4789
                                                        ],
                                                        "Black"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      4789,
                                                      4792
                                                    ],
                                                    "llK"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  4793,
                                                  4796
                                                ],
                                                "llV"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              4797,
                                              4803
                                            ],
                                            "llLeft"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          4804,
                                          4811
                                        ],
                                        "llRight"
                                      ]
                                    }
                                  ]
                                }
                              ]
                            },
                            {
                              "Application": [
                                [
                                  4826,
                                  4873
                                ],
                                {
                                  "Application": [
                                    [
                                      4826,
                                      4873
                                    ],
                                    {
                                      "Application": [
                                        [
                                          4826,
                                          4873
                                        ],
                                        {
                                          "Application": [
                                            [
                                              4826,
                                              4873
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  4826,
                                                  4873
                                                ],
                                                {
                                                  "Ref": [
                                                    [
                                                      4826,
                                                      4845
                                                    ],
                                                    "RBNode_elm_builtin"
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      4845,
                                                      4851
                                                    ],
                                                    "Black"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  4851,
                                                  4854
                                                ],
                                                "key"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              4855,
                                              4860
                                            ],
                                            "value"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          4861,
                                          4867
                                        ],
                                        "lRight"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      4868,
                                      4873
                                    ],
                                    "right"
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ],
                      [
                        "Wildcard",
                        {
                          "Application": [
                            [
                              4899,
                              4944
                            ],
                            {
                              "Application": [
                                [
                                  4899,
                                  4944
                                ],
                                {
                                  "Application": [
                                    [
                                      4899,
                                      4944
                                    ],
                                    {
                                      "Application": [
                                        [
                                          4899,
                                          4944
                                        ],
                                        {
                                          "Application": [
                                            [
                                              4899,
                                              4944
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  4899,
                                                  4918
                                                ],
                                                "RBNode_elm_builtin"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  4918,
                                                  4923
                                                ],
                                                "color"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              4924,
                                              4927
                                            ],
                                            "key"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          4928,
                                          4933
                                        ],
                                        "value"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      4934,
                                      4938
                                    ],
                                    "left"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  4939,
                                  4944
                                ],
                                "right"
                              ]
                            }
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ]
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
              "Var": "comparable"
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "remove",
        "patterns": [
          {
            "Var": "key"
          },
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Case": [
            [
              5156,
              5278
            ],
            {
              "Application": [
                [
                  5161,
                  5180
                ],
                {
                  "Application": [
                    [
                      5161,
                      5180
                    ],
                    {
                      "Ref": [
                        [
                          5161,
                          5171
                        ],
                        "removeHelp"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          5172,
                          5175
                        ],
                        "key"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      5176,
                      5180
                    ],
                    "dict"
                  ]
                }
              ]
            },
            [
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      {
                        "Adt": [
                          "Red",
                          [
                            {
                              "Var": "k"
                            },
                            {
                              "Var": "v"
                            },
                            {
                              "Var": "l"
                            },
                            {
                              "Var": "r"
                            }
                          ]
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      5228,
                      5260
                    ],
                    {
                      "Application": [
                        [
                          5228,
                          5260
                        ],
                        {
                          "Application": [
                            [
                              5228,
                              5260
                            ],
                            {
                              "Application": [
                                [
                                  5228,
                                  5260
                                ],
                                {
                                  "Application": [
                                    [
                                      5228,
                                      5260
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          5228,
                                          5247
                                        ],
                                        "RBNode_elm_builtin"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          5247,
                                          5253
                                        ],
                                        "Black"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      5253,
                                      5254
                                    ],
                                    "k"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  5255,
                                  5256
                                ],
                                "v"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              5257,
                              5258
                            ],
                            "l"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          5259,
                          5260
                        ],
                        "r"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Var": "x"
                },
                {
                  "Ref": [
                    [
                      5277,
                      5278
                    ],
                    "x"
                  ]
                }
              ]
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
              "Var": "comparable"
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "removeHelp",
        "patterns": [
          {
            "Var": "targetKey"
          },
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Case": [
            [
              5763,
              6644
            ],
            {
              "Ref": [
                [
                  5768,
                  5772
                ],
                "dict"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "RBEmpty_elm_builtin",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      5809,
                      5828
                    ],
                    "RBEmpty_elm_builtin"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      {
                        "Var": "color"
                      },
                      {
                        "Var": "key"
                      },
                      {
                        "Var": "value"
                      },
                      {
                        "Var": "left"
                      },
                      {
                        "Var": "right"
                      }
                    ]
                  ]
                },
                {
                  "If": [
                    [
                      5889,
                      6644
                    ],
                    {
                      "OpChain": [
                        [
                          5892,
                          5907
                        ],
                        [
                          {
                            "Ref": [
                              [
                                5892,
                                5901
                              ],
                              "targetKey"
                            ]
                          },
                          {
                            "Ref": [
                              [
                                5904,
                                5907
                              ],
                              "key"
                            ]
                          }
                        ],
                        [
                          "<"
                        ]
                      ]
                    },
                    {
                      "Case": [
                        [
                          5921,
                          6544
                        ],
                        {
                          "Ref": [
                            [
                              5926,
                              5930
                            ],
                            "left"
                          ]
                        },
                        [
                          [
                            {
                              "Adt": [
                                "RBNode_elm_builtin",
                                [
                                  {
                                    "Adt": [
                                      "Black",
                                      [
                                        "Wildcard",
                                        "Wildcard",
                                        {
                                          "Var": "lLeft"
                                        },
                                        "Wildcard"
                                      ]
                                    ]
                                  }
                                ]
                              ]
                            },
                            {
                              "Case": [
                                [
                                  5996,
                                  6440
                                ],
                                {
                                  "Ref": [
                                    [
                                      6001,
                                      6006
                                    ],
                                    "lLeft"
                                  ]
                                },
                                [
                                  [
                                    {
                                      "Adt": [
                                        "RBNode_elm_builtin",
                                        [
                                          {
                                            "Adt": [
                                              "Red",
                                              [
                                                "Wildcard",
                                                "Wildcard",
                                                "Wildcard",
                                                "Wildcard"
                                              ]
                                            ]
                                          }
                                        ]
                                      ]
                                    },
                                    {
                                      "Application": [
                                        [
                                          6074,
                                          6142
                                        ],
                                        {
                                          "Application": [
                                            [
                                              6074,
                                              6142
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  6074,
                                                  6142
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      6074,
                                                      6142
                                                    ],
                                                    {
                                                      "Application": [
                                                        [
                                                          6074,
                                                          6142
                                                        ],
                                                        {
                                                          "Ref": [
                                                            [
                                                              6074,
                                                              6093
                                                            ],
                                                            "RBNode_elm_builtin"
                                                          ]
                                                        },
                                                        {
                                                          "Ref": [
                                                            [
                                                              6093,
                                                              6098
                                                            ],
                                                            "color"
                                                          ]
                                                        }
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          6099,
                                                          6102
                                                        ],
                                                        "key"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      6103,
                                                      6108
                                                    ],
                                                    "value"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Application": [
                                                [
                                                  6110,
                                                  6135
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      6110,
                                                      6135
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          6110,
                                                          6120
                                                        ],
                                                        "removeHelp"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          6121,
                                                          6130
                                                        ],
                                                        "targetKey"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      6131,
                                                      6135
                                                    ],
                                                    "left"
                                                  ]
                                                }
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              6137,
                                              6142
                                            ],
                                            "right"
                                          ]
                                        }
                                      ]
                                    }
                                  ],
                                  [
                                    "Wildcard",
                                    {
                                      "Case": [
                                        [
                                          6179,
                                          6440
                                        ],
                                        {
                                          "Application": [
                                            [
                                              6184,
                                              6200
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  6184,
                                                  6195
                                                ],
                                                "moveRedLeft"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  6196,
                                                  6200
                                                ],
                                                "dict"
                                              ]
                                            }
                                          ]
                                        },
                                        [
                                          [
                                            {
                                              "Adt": [
                                                "RBNode_elm_builtin",
                                                [
                                                  {
                                                    "Var": "nColor"
                                                  },
                                                  {
                                                    "Var": "nKey"
                                                  },
                                                  {
                                                    "Var": "nValue"
                                                  },
                                                  {
                                                    "Var": "nLeft"
                                                  },
                                                  {
                                                    "Var": "nRight"
                                                  }
                                                ]
                                              ]
                                            },
                                            {
                                              "Application": [
                                                [
                                                  6296,
                                                  6358
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      6296,
                                                      6358
                                                    ],
                                                    {
                                                      "Application": [
                                                        [
                                                          6296,
                                                          6358
                                                        ],
                                                        {
                                                          "Application": [
                                                            [
                                                              6296,
                                                              6358
                                                            ],
                                                            {
                                                              "Application": [
                                                                [
                                                                  6296,
                                                                  6358
                                                                ],
                                                                {
                                                                  "Ref": [
                                                                    [
                                                                      6296,
                                                                      6303
                                                                    ],
                                                                    "balance"
                                                                  ]
                                                                },
                                                                {
                                                                  "Ref": [
                                                                    [
                                                                      6304,
                                                                      6310
                                                                    ],
                                                                    "nColor"
                                                                  ]
                                                                }
                                                              ]
                                                            },
                                                            {
                                                              "Ref": [
                                                                [
                                                                  6311,
                                                                  6315
                                                                ],
                                                                "nKey"
                                                              ]
                                                            }
                                                          ]
                                                        },
                                                        {
                                                          "Ref": [
                                                            [
                                                              6316,
                                                              6322
                                                            ],
                                                            "nValue"
                                                          ]
                                                        }
                                                      ]
                                                    },
                                                    {
                                                      "Application": [
                                                        [
                                                          6324,
                                                          6350
                                                        ],
                                                        {
                                                          "Application": [
                                                            [
                                                              6324,
                                                              6350
                                                            ],
                                                            {
                                                              "Ref": [
                                                                [
                                                                  6324,
                                                                  6334
                                                                ],
                                                                "removeHelp"
                                                              ]
                                                            },
                                                            {
                                                              "Ref": [
                                                                [
                                                                  6335,
                                                                  6344
                                                                ],
                                                                "targetKey"
                                                              ]
                                                            }
                                                          ]
                                                        },
                                                        {
                                                          "Ref": [
                                                            [
                                                              6345,
                                                              6350
                                                            ],
                                                            "nLeft"
                                                          ]
                                                        }
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      6352,
                                                      6358
                                                    ],
                                                    "nRight"
                                                  ]
                                                }
                                              ]
                                            }
                                          ],
                                          [
                                            {
                                              "Adt": [
                                                "RBEmpty_elm_builtin",
                                                []
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  6421,
                                                  6440
                                                ],
                                                "RBEmpty_elm_builtin"
                                              ]
                                            }
                                          ]
                                        ]
                                      ]
                                    }
                                  ]
                                ]
                              ]
                            }
                          ],
                          [
                            "Wildcard",
                            {
                              "Application": [
                                [
                                  6469,
                                  6537
                                ],
                                {
                                  "Application": [
                                    [
                                      6469,
                                      6537
                                    ],
                                    {
                                      "Application": [
                                        [
                                          6469,
                                          6537
                                        ],
                                        {
                                          "Application": [
                                            [
                                              6469,
                                              6537
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  6469,
                                                  6537
                                                ],
                                                {
                                                  "Ref": [
                                                    [
                                                      6469,
                                                      6488
                                                    ],
                                                    "RBNode_elm_builtin"
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      6488,
                                                      6493
                                                    ],
                                                    "color"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  6494,
                                                  6497
                                                ],
                                                "key"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              6498,
                                              6503
                                            ],
                                            "value"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Application": [
                                        [
                                          6505,
                                          6530
                                        ],
                                        {
                                          "Application": [
                                            [
                                              6505,
                                              6530
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  6505,
                                                  6515
                                                ],
                                                "removeHelp"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  6516,
                                                  6525
                                                ],
                                                "targetKey"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              6526,
                                              6530
                                            ],
                                            "left"
                                          ]
                                        }
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      6532,
                                      6537
                                    ],
                                    "right"
                                  ]
                                }
                              ]
                            }
                          ]
                        ]
                      ]
                    },
                    {
                      "Application": [
                        [
                          6557,
                          6644
                        ],
                        {
                          "Application": [
                            [
                              6557,
                              6644
                            ],
                            {
                              "Ref": [
                                [
                                  6557,
                                  6571
                                ],
                                "removeHelpEQGT"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  6572,
                                  6581
                                ],
                                "targetKey"
                              ]
                            }
                          ]
                        },
                        {
                          "Application": [
                            [
                              6583,
                              6643
                            ],
                            {
                              "Application": [
                                [
                                  6583,
                                  6643
                                ],
                                {
                                  "Application": [
                                    [
                                      6583,
                                      6643
                                    ],
                                    {
                                      "Application": [
                                        [
                                          6583,
                                          6643
                                        ],
                                        {
                                          "Application": [
                                            [
                                              6583,
                                              6643
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  6583,
                                                  6643
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      6583,
                                                      6643
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          6583,
                                                          6601
                                                        ],
                                                        "removeHelpPrepEQGT"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          6602,
                                                          6611
                                                        ],
                                                        "targetKey"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      6612,
                                                      6616
                                                    ],
                                                    "dict"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  6617,
                                                  6622
                                                ],
                                                "color"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              6623,
                                              6626
                                            ],
                                            "key"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          6627,
                                          6632
                                        ],
                                        "value"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      6633,
                                      6637
                                    ],
                                    "left"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  6638,
                                  6643
                                ],
                                "right"
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
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
              "Var": "comparable"
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "NColor",
                        []
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Var": "comparable"
                        },
                        {
                          "Fun": [
                            {
                              "Var": "v"
                            },
                            {
                              "Fun": [
                                {
                                  "Tag": [
                                    "Dict",
                                    [
                                      {
                                        "Var": "comparable"
                                      },
                                      {
                                        "Var": "v"
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Fun": [
                                    {
                                      "Tag": [
                                        "Dict",
                                        [
                                          {
                                            "Var": "comparable"
                                          },
                                          {
                                            "Var": "v"
                                          }
                                        ]
                                      ]
                                    },
                                    {
                                      "Tag": [
                                        "Dict",
                                        [
                                          {
                                            "Var": "comparable"
                                          },
                                          {
                                            "Var": "v"
                                          }
                                        ]
                                      ]
                                    }
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "removeHelpPrepEQGT",
        "patterns": [
          {
            "Var": "targetKey"
          },
          {
            "Var": "dict"
          },
          {
            "Var": "color"
          },
          {
            "Var": "key"
          },
          {
            "Var": "value"
          },
          {
            "Var": "left"
          },
          {
            "Var": "right"
          }
        ],
        "expr": {
          "Case": [
            [
              6857,
              7304
            ],
            {
              "Ref": [
                [
                  6862,
                  6866
                ],
                "left"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      {
                        "Adt": [
                          "Red",
                          [
                            {
                              "Var": "lK"
                            },
                            {
                              "Var": "lV"
                            },
                            {
                              "Var": "lLeft"
                            },
                            {
                              "Var": "lRight"
                            }
                          ]
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      6925,
                      7049
                    ],
                    {
                      "Application": [
                        [
                          6925,
                          7049
                        ],
                        {
                          "Application": [
                            [
                              6925,
                              7049
                            ],
                            {
                              "Application": [
                                [
                                  6925,
                                  7049
                                ],
                                {
                                  "Application": [
                                    [
                                      6925,
                                      7049
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          6925,
                                          6952
                                        ],
                                        "RBNode_elm_builtin"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          6952,
                                          6957
                                        ],
                                        "color"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      6966,
                                      6968
                                    ],
                                    "lK"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  6977,
                                  6979
                                ],
                                "lV"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              6988,
                              6993
                            ],
                            "lLeft"
                          ]
                        }
                      ]
                    },
                    {
                      "Application": [
                        [
                          7003,
                          7048
                        ],
                        {
                          "Application": [
                            [
                              7003,
                              7048
                            ],
                            {
                              "Application": [
                                [
                                  7003,
                                  7048
                                ],
                                {
                                  "Application": [
                                    [
                                      7003,
                                      7048
                                    ],
                                    {
                                      "Application": [
                                        [
                                          7003,
                                          7048
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              7003,
                                              7022
                                            ],
                                            "RBNode_elm_builtin"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              7022,
                                              7026
                                            ],
                                            "Red"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          7026,
                                          7029
                                        ],
                                        "key"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      7030,
                                      7035
                                    ],
                                    "value"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  7036,
                                  7042
                                ],
                                "lRight"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              7043,
                              7048
                            ],
                            "right"
                          ]
                        }
                      ]
                    }
                  ]
                }
              ],
              [
                "Wildcard",
                {
                  "Case": [
                    [
                      7066,
                      7304
                    ],
                    {
                      "Ref": [
                        [
                          7071,
                          7076
                        ],
                        "right"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            "RBNode_elm_builtin",
                            [
                              {
                                "Adt": [
                                  "Black",
                                  [
                                    "Wildcard",
                                    "Wildcard",
                                    {
                                      "Adt": [
                                        "RBNode_elm_builtin",
                                        [
                                          {
                                            "Adt": [
                                              "Black",
                                              [
                                                "Wildcard",
                                                "Wildcard",
                                                "Wildcard",
                                                "Wildcard"
                                              ]
                                            ]
                                          }
                                        ]
                                      ]
                                    },
                                    "Wildcard"
                                  ]
                                ]
                              }
                            ]
                          ]
                        },
                        {
                          "Application": [
                            [
                              7167,
                              7184
                            ],
                            {
                              "Ref": [
                                [
                                  7167,
                                  7179
                                ],
                                "moveRedRight"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  7180,
                                  7184
                                ],
                                "dict"
                              ]
                            }
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            "RBNode_elm_builtin",
                            [
                              {
                                "Adt": [
                                  "Black",
                                  [
                                    "Wildcard",
                                    "Wildcard",
                                    {
                                      "Adt": [
                                        "RBEmpty_elm_builtin",
                                        [
                                          "Wildcard"
                                        ]
                                      ]
                                    }
                                  ]
                                ]
                              }
                            ]
                          ]
                        },
                        {
                          "Application": [
                            [
                              7258,
                              7275
                            ],
                            {
                              "Ref": [
                                [
                                  7258,
                                  7270
                                ],
                                "moveRedRight"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  7271,
                                  7275
                                ],
                                "dict"
                              ]
                            }
                          ]
                        }
                      ],
                      [
                        "Wildcard",
                        {
                          "Ref": [
                            [
                              7300,
                              7304
                            ],
                            "dict"
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ]
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
              "Var": "comparable"
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "removeHelpEQGT",
        "patterns": [
          {
            "Var": "targetKey"
          },
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Case": [
            [
              7590,
              8031
            ],
            {
              "Ref": [
                [
                  7595,
                  7599
                ],
                "dict"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      {
                        "Var": "color"
                      },
                      {
                        "Var": "key"
                      },
                      {
                        "Var": "value"
                      },
                      {
                        "Var": "left"
                      },
                      {
                        "Var": "right"
                      }
                    ]
                  ]
                },
                {
                  "If": [
                    [
                      7662,
                      7977
                    ],
                    {
                      "OpChain": [
                        [
                          7665,
                          7681
                        ],
                        [
                          {
                            "Ref": [
                              [
                                7665,
                                7674
                              ],
                              "targetKey"
                            ]
                          },
                          {
                            "Ref": [
                              [
                                7678,
                                7681
                              ],
                              "key"
                            ]
                          }
                        ],
                        [
                          "=="
                        ]
                      ]
                    },
                    {
                      "Case": [
                        [
                          7695,
                          7907
                        ],
                        {
                          "Application": [
                            [
                              7700,
                              7712
                            ],
                            {
                              "Ref": [
                                [
                                  7700,
                                  7706
                                ],
                                "getMin"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  7707,
                                  7712
                                ],
                                "right"
                              ]
                            }
                          ]
                        },
                        [
                          [
                            {
                              "Adt": [
                                "RBNode_elm_builtin",
                                [
                                  "Wildcard",
                                  {
                                    "Var": "minKey"
                                  },
                                  {
                                    "Var": "minValue"
                                  },
                                  "Wildcard",
                                  "Wildcard"
                                ]
                              ]
                            },
                            {
                              "Application": [
                                [
                                  7782,
                                  7834
                                ],
                                {
                                  "Application": [
                                    [
                                      7782,
                                      7834
                                    ],
                                    {
                                      "Application": [
                                        [
                                          7782,
                                          7834
                                        ],
                                        {
                                          "Application": [
                                            [
                                              7782,
                                              7834
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  7782,
                                                  7834
                                                ],
                                                {
                                                  "Ref": [
                                                    [
                                                      7782,
                                                      7789
                                                    ],
                                                    "balance"
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      7790,
                                                      7795
                                                    ],
                                                    "color"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  7796,
                                                  7802
                                                ],
                                                "minKey"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              7803,
                                              7811
                                            ],
                                            "minValue"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          7812,
                                          7816
                                        ],
                                        "left"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      7818,
                                      7833
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          7818,
                                          7827
                                        ],
                                        "removeMin"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          7828,
                                          7833
                                        ],
                                        "right"
                                      ]
                                    }
                                  ]
                                }
                              ]
                            }
                          ],
                          [
                            {
                              "Adt": [
                                "RBEmpty_elm_builtin",
                                []
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  7881,
                                  7907
                                ],
                                "RBEmpty_elm_builtin"
                              ]
                            }
                          ]
                        ]
                      ]
                    },
                    {
                      "Application": [
                        [
                          7920,
                          7977
                        ],
                        {
                          "Application": [
                            [
                              7920,
                              7977
                            ],
                            {
                              "Application": [
                                [
                                  7920,
                                  7977
                                ],
                                {
                                  "Application": [
                                    [
                                      7920,
                                      7977
                                    ],
                                    {
                                      "Application": [
                                        [
                                          7920,
                                          7977
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              7920,
                                              7927
                                            ],
                                            "balance"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              7928,
                                              7933
                                            ],
                                            "color"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          7934,
                                          7937
                                        ],
                                        "key"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      7938,
                                      7943
                                    ],
                                    "value"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  7944,
                                  7948
                                ],
                                "left"
                              ]
                            }
                          ]
                        },
                        {
                          "Application": [
                            [
                              7950,
                              7976
                            ],
                            {
                              "Application": [
                                [
                                  7950,
                                  7976
                                ],
                                {
                                  "Ref": [
                                    [
                                      7950,
                                      7960
                                    ],
                                    "removeHelp"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      7961,
                                      7970
                                    ],
                                    "targetKey"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  7971,
                                  7976
                                ],
                                "right"
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "RBEmpty_elm_builtin",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      8012,
                      8031
                    ],
                    "RBEmpty_elm_builtin"
                  ]
                }
              ]
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
                "Dict",
                [
                  {
                    "Var": "k"
                  },
                  {
                    "Var": "v"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Dict",
                [
                  {
                    "Var": "k"
                  },
                  {
                    "Var": "v"
                  }
                ]
              ]
            }
          ]
        },
        "name": "getMin",
        "patterns": [
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Case": [
            [
              8080,
              8206
            ],
            {
              "Ref": [
                [
                  8085,
                  8089
                ],
                "dict"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      "Wildcard",
                      "Wildcard",
                      "Wildcard",
                      {
                        "Alias": [
                          {
                            "Adt": [
                              "RBNode_elm_builtin",
                              [
                                "Wildcard",
                                "Wildcard",
                                "Wildcard",
                                "Wildcard",
                                "Wildcard"
                              ]
                            ]
                          },
                          "left"
                        ]
                      },
                      "Wildcard"
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      8174,
                      8185
                    ],
                    {
                      "Ref": [
                        [
                          8174,
                          8180
                        ],
                        "getMin"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          8181,
                          8185
                        ],
                        "left"
                      ]
                    }
                  ]
                }
              ],
              [
                "Wildcard",
                {
                  "Ref": [
                    [
                      8202,
                      8206
                    ],
                    "dict"
                  ]
                }
              ]
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
                "Dict",
                [
                  {
                    "Var": "k"
                  },
                  {
                    "Var": "v"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Dict",
                [
                  {
                    "Var": "k"
                  },
                  {
                    "Var": "v"
                  }
                ]
              ]
            }
          ]
        },
        "name": "removeMin",
        "patterns": [
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Case": [
            [
              8261,
              8944
            ],
            {
              "Ref": [
                [
                  8266,
                  8270
                ],
                "dict"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      {
                        "Var": "color"
                      },
                      {
                        "Var": "key"
                      },
                      {
                        "Var": "value"
                      },
                      {
                        "Alias": [
                          {
                            "Adt": [
                              "RBNode_elm_builtin",
                              [
                                {
                                  "Var": "lColor"
                                },
                                "Wildcard",
                                "Wildcard",
                                {
                                  "Var": "lLeft"
                                },
                                "Wildcard"
                              ]
                            ]
                          },
                          "left"
                        ]
                      },
                      {
                        "Var": "right"
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      8378,
                      8908
                    ],
                    {
                      "Ref": [
                        [
                          8383,
                          8389
                        ],
                        "lColor"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            "Black",
                            []
                          ]
                        },
                        {
                          "Case": [
                            [
                              8420,
                              8826
                            ],
                            {
                              "Ref": [
                                [
                                  8425,
                                  8430
                                ],
                                "lLeft"
                              ]
                            },
                            [
                              [
                                {
                                  "Adt": [
                                    "RBNode_elm_builtin",
                                    [
                                      {
                                        "Adt": [
                                          "Red",
                                          [
                                            "Wildcard",
                                            "Wildcard",
                                            "Wildcard",
                                            "Wildcard"
                                          ]
                                        ]
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      8494,
                                      8551
                                    ],
                                    {
                                      "Application": [
                                        [
                                          8494,
                                          8551
                                        ],
                                        {
                                          "Application": [
                                            [
                                              8494,
                                              8551
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  8494,
                                                  8551
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      8494,
                                                      8551
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          8494,
                                                          8513
                                                        ],
                                                        "RBNode_elm_builtin"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          8513,
                                                          8518
                                                        ],
                                                        "color"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      8519,
                                                      8522
                                                    ],
                                                    "key"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  8523,
                                                  8528
                                                ],
                                                "value"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Application": [
                                            [
                                              8530,
                                              8544
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  8530,
                                                  8539
                                                ],
                                                "removeMin"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  8540,
                                                  8544
                                                ],
                                                "left"
                                              ]
                                            }
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          8546,
                                          8551
                                        ],
                                        "right"
                                      ]
                                    }
                                  ]
                                }
                              ],
                              [
                                "Wildcard",
                                {
                                  "Case": [
                                    [
                                      8584,
                                      8826
                                    ],
                                    {
                                      "Application": [
                                        [
                                          8589,
                                          8605
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              8589,
                                              8600
                                            ],
                                            "moveRedLeft"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              8601,
                                              8605
                                            ],
                                            "dict"
                                          ]
                                        }
                                      ]
                                    },
                                    [
                                      [
                                        {
                                          "Adt": [
                                            "RBNode_elm_builtin",
                                            [
                                              {
                                                "Var": "nColor"
                                              },
                                              {
                                                "Var": "nKey"
                                              },
                                              {
                                                "Var": "nValue"
                                              },
                                              {
                                                "Var": "nLeft"
                                              },
                                              {
                                                "Var": "nRight"
                                              }
                                            ]
                                          ]
                                        },
                                        {
                                          "Application": [
                                            [
                                              8697,
                                              8748
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  8697,
                                                  8748
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      8697,
                                                      8748
                                                    ],
                                                    {
                                                      "Application": [
                                                        [
                                                          8697,
                                                          8748
                                                        ],
                                                        {
                                                          "Application": [
                                                            [
                                                              8697,
                                                              8748
                                                            ],
                                                            {
                                                              "Ref": [
                                                                [
                                                                  8697,
                                                                  8704
                                                                ],
                                                                "balance"
                                                              ]
                                                            },
                                                            {
                                                              "Ref": [
                                                                [
                                                                  8705,
                                                                  8711
                                                                ],
                                                                "nColor"
                                                              ]
                                                            }
                                                          ]
                                                        },
                                                        {
                                                          "Ref": [
                                                            [
                                                              8712,
                                                              8716
                                                            ],
                                                            "nKey"
                                                          ]
                                                        }
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          8717,
                                                          8723
                                                        ],
                                                        "nValue"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Application": [
                                                    [
                                                      8725,
                                                      8740
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          8725,
                                                          8734
                                                        ],
                                                        "removeMin"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          8735,
                                                          8740
                                                        ],
                                                        "nLeft"
                                                      ]
                                                    }
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  8742,
                                                  8748
                                                ],
                                                "nRight"
                                              ]
                                            }
                                          ]
                                        }
                                      ],
                                      [
                                        {
                                          "Adt": [
                                            "RBEmpty_elm_builtin",
                                            []
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              8807,
                                              8826
                                            ],
                                            "RBEmpty_elm_builtin"
                                          ]
                                        }
                                      ]
                                    ]
                                  ]
                                }
                              ]
                            ]
                          ]
                        }
                      ],
                      [
                        "Wildcard",
                        {
                          "Application": [
                            [
                              8851,
                              8908
                            ],
                            {
                              "Application": [
                                [
                                  8851,
                                  8908
                                ],
                                {
                                  "Application": [
                                    [
                                      8851,
                                      8908
                                    ],
                                    {
                                      "Application": [
                                        [
                                          8851,
                                          8908
                                        ],
                                        {
                                          "Application": [
                                            [
                                              8851,
                                              8908
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  8851,
                                                  8870
                                                ],
                                                "RBNode_elm_builtin"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  8870,
                                                  8875
                                                ],
                                                "color"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              8876,
                                              8879
                                            ],
                                            "key"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          8880,
                                          8885
                                        ],
                                        "value"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      8887,
                                      8901
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          8887,
                                          8896
                                        ],
                                        "removeMin"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          8897,
                                          8901
                                        ],
                                        "left"
                                      ]
                                    }
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  8903,
                                  8908
                                ],
                                "right"
                              ]
                            }
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ],
              [
                "Wildcard",
                {
                  "Ref": [
                    [
                      8925,
                      8944
                    ],
                    "RBEmpty_elm_builtin"
                  ]
                }
              ]
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
                "Dict",
                [
                  {
                    "Var": "k"
                  },
                  {
                    "Var": "v"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Dict",
                [
                  {
                    "Var": "k"
                  },
                  {
                    "Var": "v"
                  }
                ]
              ]
            }
          ]
        },
        "name": "moveRedLeft",
        "patterns": [
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Case": [
            [
              9003,
              9956
            ],
            {
              "Ref": [
                [
                  9008,
                  9012
                ],
                "dict"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      {
                        "Var": "clr"
                      },
                      {
                        "Var": "k"
                      },
                      {
                        "Var": "v"
                      },
                      {
                        "Adt": [
                          "RBNode_elm_builtin",
                          [
                            {
                              "Var": "lClr"
                            },
                            {
                              "Var": "lK"
                            },
                            {
                              "Var": "lV"
                            },
                            {
                              "Var": "lLeft"
                            },
                            {
                              "Var": "lRight"
                            }
                          ]
                        ]
                      },
                      {
                        "Adt": [
                          "RBNode_elm_builtin",
                          [
                            {
                              "Var": "rClr"
                            },
                            {
                              "Var": "rK"
                            },
                            {
                              "Var": "rV"
                            },
                            {
                              "Alias": [
                                {
                                  "Adt": [
                                    "RBNode_elm_builtin",
                                    [
                                      {
                                        "Adt": [
                                          "Red",
                                          [
                                            {
                                              "Var": "rlK"
                                            },
                                            {
                                              "Var": "rlV"
                                            },
                                            {
                                              "Var": "rlL"
                                            },
                                            {
                                              "Var": "rlR"
                                            }
                                          ]
                                        ]
                                      }
                                    ]
                                  ]
                                },
                                "rLeft"
                              ]
                            },
                            {
                              "Var": "rRight"
                            }
                          ]
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      9192,
                      9385
                    ],
                    {
                      "Application": [
                        [
                          9192,
                          9385
                        ],
                        {
                          "Application": [
                            [
                              9192,
                              9385
                            ],
                            {
                              "Application": [
                                [
                                  9192,
                                  9385
                                ],
                                {
                                  "Application": [
                                    [
                                      9192,
                                      9385
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          9192,
                                          9219
                                        ],
                                        "RBNode_elm_builtin"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          9219,
                                          9231
                                        ],
                                        "Red"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      9231,
                                      9234
                                    ],
                                    "rlK"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  9243,
                                  9246
                                ],
                                "rlV"
                              ]
                            }
                          ]
                        },
                        {
                          "Application": [
                            [
                              9256,
                              9332
                            ],
                            {
                              "Application": [
                                [
                                  9256,
                                  9332
                                ],
                                {
                                  "Application": [
                                    [
                                      9256,
                                      9332
                                    ],
                                    {
                                      "Application": [
                                        [
                                          9256,
                                          9332
                                        ],
                                        {
                                          "Application": [
                                            [
                                              9256,
                                              9332
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  9256,
                                                  9275
                                                ],
                                                "RBNode_elm_builtin"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  9275,
                                                  9281
                                                ],
                                                "Black"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              9281,
                                              9282
                                            ],
                                            "k"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          9283,
                                          9284
                                        ],
                                        "v"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      9286,
                                      9327
                                    ],
                                    {
                                      "Application": [
                                        [
                                          9286,
                                          9327
                                        ],
                                        {
                                          "Application": [
                                            [
                                              9286,
                                              9327
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  9286,
                                                  9327
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      9286,
                                                      9327
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          9286,
                                                          9305
                                                        ],
                                                        "RBNode_elm_builtin"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          9305,
                                                          9309
                                                        ],
                                                        "Red"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      9309,
                                                      9311
                                                    ],
                                                    "lK"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  9312,
                                                  9314
                                                ],
                                                "lV"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              9315,
                                              9320
                                            ],
                                            "lLeft"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          9321,
                                          9327
                                        ],
                                        "lRight"
                                      ]
                                    }
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  9329,
                                  9332
                                ],
                                "rlL"
                              ]
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "Application": [
                        [
                          9343,
                          9384
                        ],
                        {
                          "Application": [
                            [
                              9343,
                              9384
                            ],
                            {
                              "Application": [
                                [
                                  9343,
                                  9384
                                ],
                                {
                                  "Application": [
                                    [
                                      9343,
                                      9384
                                    ],
                                    {
                                      "Application": [
                                        [
                                          9343,
                                          9384
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              9343,
                                              9362
                                            ],
                                            "RBNode_elm_builtin"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              9362,
                                              9368
                                            ],
                                            "Black"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          9368,
                                          9370
                                        ],
                                        "rK"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      9371,
                                      9373
                                    ],
                                    "rV"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  9374,
                                  9377
                                ],
                                "rlR"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              9378,
                              9384
                            ],
                            "rRight"
                          ]
                        }
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      {
                        "Var": "clr"
                      },
                      {
                        "Var": "k"
                      },
                      {
                        "Var": "v"
                      },
                      {
                        "Adt": [
                          "RBNode_elm_builtin",
                          [
                            {
                              "Var": "lClr"
                            },
                            {
                              "Var": "lK"
                            },
                            {
                              "Var": "lV"
                            },
                            {
                              "Var": "lLeft"
                            },
                            {
                              "Var": "lRight"
                            }
                          ]
                        ]
                      },
                      {
                        "Adt": [
                          "RBNode_elm_builtin",
                          [
                            {
                              "Var": "rClr"
                            },
                            {
                              "Var": "rK"
                            },
                            {
                              "Var": "rV"
                            },
                            {
                              "Var": "rLeft"
                            },
                            {
                              "Var": "rRight"
                            }
                          ]
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      9517,
                      9935
                    ],
                    {
                      "Ref": [
                        [
                          9522,
                          9525
                        ],
                        "clr"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            "Black",
                            []
                          ]
                        },
                        {
                          "Application": [
                            [
                              9556,
                              9732
                            ],
                            {
                              "Application": [
                                [
                                  9556,
                                  9732
                                ],
                                {
                                  "Application": [
                                    [
                                      9556,
                                      9732
                                    ],
                                    {
                                      "Application": [
                                        [
                                          9556,
                                          9732
                                        ],
                                        {
                                          "Application": [
                                            [
                                              9556,
                                              9732
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  9556,
                                                  9587
                                                ],
                                                "RBNode_elm_builtin"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  9587,
                                                  9605
                                                ],
                                                "Black"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              9605,
                                              9606
                                            ],
                                            "k"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          9619,
                                          9620
                                        ],
                                        "v"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      9634,
                                      9675
                                    ],
                                    {
                                      "Application": [
                                        [
                                          9634,
                                          9675
                                        ],
                                        {
                                          "Application": [
                                            [
                                              9634,
                                              9675
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  9634,
                                                  9675
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      9634,
                                                      9675
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          9634,
                                                          9653
                                                        ],
                                                        "RBNode_elm_builtin"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          9653,
                                                          9657
                                                        ],
                                                        "Red"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      9657,
                                                      9659
                                                    ],
                                                    "lK"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  9660,
                                                  9662
                                                ],
                                                "lV"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              9663,
                                              9668
                                            ],
                                            "lLeft"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          9669,
                                          9675
                                        ],
                                        "lRight"
                                      ]
                                    }
                                  ]
                                }
                              ]
                            },
                            {
                              "Application": [
                                [
                                  9690,
                                  9731
                                ],
                                {
                                  "Application": [
                                    [
                                      9690,
                                      9731
                                    ],
                                    {
                                      "Application": [
                                        [
                                          9690,
                                          9731
                                        ],
                                        {
                                          "Application": [
                                            [
                                              9690,
                                              9731
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  9690,
                                                  9731
                                                ],
                                                {
                                                  "Ref": [
                                                    [
                                                      9690,
                                                      9709
                                                    ],
                                                    "RBNode_elm_builtin"
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      9709,
                                                      9713
                                                    ],
                                                    "Red"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  9713,
                                                  9715
                                                ],
                                                "rK"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              9716,
                                              9718
                                            ],
                                            "rV"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          9719,
                                          9724
                                        ],
                                        "rLeft"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      9725,
                                      9731
                                    ],
                                    "rRight"
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            "Red",
                            []
                          ]
                        },
                        {
                          "Application": [
                            [
                              9759,
                              9935
                            ],
                            {
                              "Application": [
                                [
                                  9759,
                                  9935
                                ],
                                {
                                  "Application": [
                                    [
                                      9759,
                                      9935
                                    ],
                                    {
                                      "Application": [
                                        [
                                          9759,
                                          9935
                                        ],
                                        {
                                          "Application": [
                                            [
                                              9759,
                                              9935
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  9759,
                                                  9790
                                                ],
                                                "RBNode_elm_builtin"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  9790,
                                                  9808
                                                ],
                                                "Black"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              9808,
                                              9809
                                            ],
                                            "k"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          9822,
                                          9823
                                        ],
                                        "v"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      9837,
                                      9878
                                    ],
                                    {
                                      "Application": [
                                        [
                                          9837,
                                          9878
                                        ],
                                        {
                                          "Application": [
                                            [
                                              9837,
                                              9878
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  9837,
                                                  9878
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      9837,
                                                      9878
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          9837,
                                                          9856
                                                        ],
                                                        "RBNode_elm_builtin"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          9856,
                                                          9860
                                                        ],
                                                        "Red"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      9860,
                                                      9862
                                                    ],
                                                    "lK"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  9863,
                                                  9865
                                                ],
                                                "lV"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              9866,
                                              9871
                                            ],
                                            "lLeft"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          9872,
                                          9878
                                        ],
                                        "lRight"
                                      ]
                                    }
                                  ]
                                }
                              ]
                            },
                            {
                              "Application": [
                                [
                                  9893,
                                  9934
                                ],
                                {
                                  "Application": [
                                    [
                                      9893,
                                      9934
                                    ],
                                    {
                                      "Application": [
                                        [
                                          9893,
                                          9934
                                        ],
                                        {
                                          "Application": [
                                            [
                                              9893,
                                              9934
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  9893,
                                                  9934
                                                ],
                                                {
                                                  "Ref": [
                                                    [
                                                      9893,
                                                      9912
                                                    ],
                                                    "RBNode_elm_builtin"
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      9912,
                                                      9916
                                                    ],
                                                    "Red"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  9916,
                                                  9918
                                                ],
                                                "rK"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              9919,
                                              9921
                                            ],
                                            "rV"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          9922,
                                          9927
                                        ],
                                        "rLeft"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      9928,
                                      9934
                                    ],
                                    "rRight"
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ],
              [
                "Wildcard",
                {
                  "Ref": [
                    [
                      9952,
                      9956
                    ],
                    "dict"
                  ]
                }
              ]
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
                "Dict",
                [
                  {
                    "Var": "k"
                  },
                  {
                    "Var": "v"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Dict",
                [
                  {
                    "Var": "k"
                  },
                  {
                    "Var": "v"
                  }
                ]
              ]
            }
          ]
        },
        "name": "moveRedRight",
        "patterns": [
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Case": [
            [
              10017,
              10973
            ],
            {
              "Ref": [
                [
                  10022,
                  10026
                ],
                "dict"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      {
                        "Var": "clr"
                      },
                      {
                        "Var": "k"
                      },
                      {
                        "Var": "v"
                      },
                      {
                        "Adt": [
                          "RBNode_elm_builtin",
                          [
                            {
                              "Var": "lClr"
                            },
                            {
                              "Var": "lK"
                            },
                            {
                              "Var": "lV"
                            },
                            {
                              "Adt": [
                                "RBNode_elm_builtin",
                                [
                                  {
                                    "Adt": [
                                      "Red",
                                      [
                                        {
                                          "Var": "llK"
                                        },
                                        {
                                          "Var": "llV"
                                        },
                                        {
                                          "Var": "llLeft"
                                        },
                                        {
                                          "Var": "llRight"
                                        }
                                      ]
                                    ]
                                  }
                                ]
                              ]
                            },
                            {
                              "Var": "lRight"
                            }
                          ]
                        ]
                      },
                      {
                        "Adt": [
                          "RBNode_elm_builtin",
                          [
                            {
                              "Var": "rClr"
                            },
                            {
                              "Var": "rK"
                            },
                            {
                              "Var": "rV"
                            },
                            {
                              "Var": "rLeft"
                            },
                            {
                              "Var": "rRight"
                            }
                          ]
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      10202,
                      10402
                    ],
                    {
                      "Application": [
                        [
                          10202,
                          10402
                        ],
                        {
                          "Application": [
                            [
                              10202,
                              10402
                            ],
                            {
                              "Application": [
                                [
                                  10202,
                                  10402
                                ],
                                {
                                  "Application": [
                                    [
                                      10202,
                                      10402
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          10202,
                                          10229
                                        ],
                                        "RBNode_elm_builtin"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          10229,
                                          10241
                                        ],
                                        "Red"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      10241,
                                      10243
                                    ],
                                    "lK"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  10252,
                                  10254
                                ],
                                "lV"
                              ]
                            }
                          ]
                        },
                        {
                          "Application": [
                            [
                              10264,
                              10311
                            ],
                            {
                              "Application": [
                                [
                                  10264,
                                  10311
                                ],
                                {
                                  "Application": [
                                    [
                                      10264,
                                      10311
                                    ],
                                    {
                                      "Application": [
                                        [
                                          10264,
                                          10311
                                        ],
                                        {
                                          "Application": [
                                            [
                                              10264,
                                              10311
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  10264,
                                                  10283
                                                ],
                                                "RBNode_elm_builtin"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  10283,
                                                  10289
                                                ],
                                                "Black"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              10289,
                                              10292
                                            ],
                                            "llK"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          10293,
                                          10296
                                        ],
                                        "llV"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      10297,
                                      10303
                                    ],
                                    "llLeft"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  10304,
                                  10311
                                ],
                                "llRight"
                              ]
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "Application": [
                        [
                          10322,
                          10401
                        ],
                        {
                          "Application": [
                            [
                              10322,
                              10401
                            ],
                            {
                              "Application": [
                                [
                                  10322,
                                  10401
                                ],
                                {
                                  "Application": [
                                    [
                                      10322,
                                      10401
                                    ],
                                    {
                                      "Application": [
                                        [
                                          10322,
                                          10401
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              10322,
                                              10341
                                            ],
                                            "RBNode_elm_builtin"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              10341,
                                              10347
                                            ],
                                            "Black"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          10347,
                                          10348
                                        ],
                                        "k"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      10349,
                                      10350
                                    ],
                                    "v"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  10351,
                                  10357
                                ],
                                "lRight"
                              ]
                            }
                          ]
                        },
                        {
                          "Application": [
                            [
                              10359,
                              10400
                            ],
                            {
                              "Application": [
                                [
                                  10359,
                                  10400
                                ],
                                {
                                  "Application": [
                                    [
                                      10359,
                                      10400
                                    ],
                                    {
                                      "Application": [
                                        [
                                          10359,
                                          10400
                                        ],
                                        {
                                          "Application": [
                                            [
                                              10359,
                                              10400
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  10359,
                                                  10378
                                                ],
                                                "RBNode_elm_builtin"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  10378,
                                                  10382
                                                ],
                                                "Red"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              10382,
                                              10384
                                            ],
                                            "rK"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          10385,
                                          10387
                                        ],
                                        "rV"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      10388,
                                      10393
                                    ],
                                    "rLeft"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  10394,
                                  10400
                                ],
                                "rRight"
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      {
                        "Var": "clr"
                      },
                      {
                        "Var": "k"
                      },
                      {
                        "Var": "v"
                      },
                      {
                        "Adt": [
                          "RBNode_elm_builtin",
                          [
                            {
                              "Var": "lClr"
                            },
                            {
                              "Var": "lK"
                            },
                            {
                              "Var": "lV"
                            },
                            {
                              "Var": "lLeft"
                            },
                            {
                              "Var": "lRight"
                            }
                          ]
                        ]
                      },
                      {
                        "Adt": [
                          "RBNode_elm_builtin",
                          [
                            {
                              "Var": "rClr"
                            },
                            {
                              "Var": "rK"
                            },
                            {
                              "Var": "rV"
                            },
                            {
                              "Var": "rLeft"
                            },
                            {
                              "Var": "rRight"
                            }
                          ]
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      10534,
                      10952
                    ],
                    {
                      "Ref": [
                        [
                          10539,
                          10542
                        ],
                        "clr"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            "Black",
                            []
                          ]
                        },
                        {
                          "Application": [
                            [
                              10573,
                              10749
                            ],
                            {
                              "Application": [
                                [
                                  10573,
                                  10749
                                ],
                                {
                                  "Application": [
                                    [
                                      10573,
                                      10749
                                    ],
                                    {
                                      "Application": [
                                        [
                                          10573,
                                          10749
                                        ],
                                        {
                                          "Application": [
                                            [
                                              10573,
                                              10749
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  10573,
                                                  10604
                                                ],
                                                "RBNode_elm_builtin"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  10604,
                                                  10622
                                                ],
                                                "Black"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              10622,
                                              10623
                                            ],
                                            "k"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          10636,
                                          10637
                                        ],
                                        "v"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      10651,
                                      10692
                                    ],
                                    {
                                      "Application": [
                                        [
                                          10651,
                                          10692
                                        ],
                                        {
                                          "Application": [
                                            [
                                              10651,
                                              10692
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  10651,
                                                  10692
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      10651,
                                                      10692
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          10651,
                                                          10670
                                                        ],
                                                        "RBNode_elm_builtin"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          10670,
                                                          10674
                                                        ],
                                                        "Red"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      10674,
                                                      10676
                                                    ],
                                                    "lK"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  10677,
                                                  10679
                                                ],
                                                "lV"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              10680,
                                              10685
                                            ],
                                            "lLeft"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          10686,
                                          10692
                                        ],
                                        "lRight"
                                      ]
                                    }
                                  ]
                                }
                              ]
                            },
                            {
                              "Application": [
                                [
                                  10707,
                                  10748
                                ],
                                {
                                  "Application": [
                                    [
                                      10707,
                                      10748
                                    ],
                                    {
                                      "Application": [
                                        [
                                          10707,
                                          10748
                                        ],
                                        {
                                          "Application": [
                                            [
                                              10707,
                                              10748
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  10707,
                                                  10748
                                                ],
                                                {
                                                  "Ref": [
                                                    [
                                                      10707,
                                                      10726
                                                    ],
                                                    "RBNode_elm_builtin"
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      10726,
                                                      10730
                                                    ],
                                                    "Red"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  10730,
                                                  10732
                                                ],
                                                "rK"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              10733,
                                              10735
                                            ],
                                            "rV"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          10736,
                                          10741
                                        ],
                                        "rLeft"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      10742,
                                      10748
                                    ],
                                    "rRight"
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            "Red",
                            []
                          ]
                        },
                        {
                          "Application": [
                            [
                              10776,
                              10952
                            ],
                            {
                              "Application": [
                                [
                                  10776,
                                  10952
                                ],
                                {
                                  "Application": [
                                    [
                                      10776,
                                      10952
                                    ],
                                    {
                                      "Application": [
                                        [
                                          10776,
                                          10952
                                        ],
                                        {
                                          "Application": [
                                            [
                                              10776,
                                              10952
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  10776,
                                                  10807
                                                ],
                                                "RBNode_elm_builtin"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  10807,
                                                  10825
                                                ],
                                                "Black"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              10825,
                                              10826
                                            ],
                                            "k"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          10839,
                                          10840
                                        ],
                                        "v"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      10854,
                                      10895
                                    ],
                                    {
                                      "Application": [
                                        [
                                          10854,
                                          10895
                                        ],
                                        {
                                          "Application": [
                                            [
                                              10854,
                                              10895
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  10854,
                                                  10895
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      10854,
                                                      10895
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          10854,
                                                          10873
                                                        ],
                                                        "RBNode_elm_builtin"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          10873,
                                                          10877
                                                        ],
                                                        "Red"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      10877,
                                                      10879
                                                    ],
                                                    "lK"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  10880,
                                                  10882
                                                ],
                                                "lV"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              10883,
                                              10888
                                            ],
                                            "lLeft"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          10889,
                                          10895
                                        ],
                                        "lRight"
                                      ]
                                    }
                                  ]
                                }
                              ]
                            },
                            {
                              "Application": [
                                [
                                  10910,
                                  10951
                                ],
                                {
                                  "Application": [
                                    [
                                      10910,
                                      10951
                                    ],
                                    {
                                      "Application": [
                                        [
                                          10910,
                                          10951
                                        ],
                                        {
                                          "Application": [
                                            [
                                              10910,
                                              10951
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  10910,
                                                  10951
                                                ],
                                                {
                                                  "Ref": [
                                                    [
                                                      10910,
                                                      10929
                                                    ],
                                                    "RBNode_elm_builtin"
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      10929,
                                                      10933
                                                    ],
                                                    "Red"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  10933,
                                                  10935
                                                ],
                                                "rK"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              10936,
                                              10938
                                            ],
                                            "rV"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          10939,
                                          10944
                                        ],
                                        "rLeft"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      10945,
                                      10951
                                    ],
                                    "rRight"
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ],
              [
                "Wildcard",
                {
                  "Ref": [
                    [
                      10969,
                      10973
                    ],
                    "dict"
                  ]
                }
              ]
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
              "Var": "comparable"
            },
            {
              "Fun": [
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Maybe",
                        [
                          {
                            "Var": "v"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "Maybe",
                        [
                          {
                            "Var": "v"
                          }
                        ]
                      ]
                    }
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Dict",
                        [
                          {
                            "Var": "comparable"
                          },
                          {
                            "Var": "v"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "Dict",
                        [
                          {
                            "Var": "comparable"
                          },
                          {
                            "Var": "v"
                          }
                        ]
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "update",
        "patterns": [
          {
            "Var": "targetKey"
          },
          {
            "Var": "alter"
          },
          {
            "Var": "dictionary"
          }
        ],
        "expr": {
          "Case": [
            [
              11182,
              11330
            ],
            {
              "Application": [
                [
                  11187,
                  11219
                ],
                {
                  "Ref": [
                    [
                      11187,
                      11192
                    ],
                    "alter"
                  ]
                },
                {
                  "Application": [
                    [
                      11194,
                      11218
                    ],
                    {
                      "Application": [
                        [
                          11194,
                          11218
                        ],
                        {
                          "Ref": [
                            [
                              11194,
                              11197
                            ],
                            "get"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              11198,
                              11207
                            ],
                            "targetKey"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          11208,
                          11218
                        ],
                        "dictionary"
                      ]
                    }
                  ]
                }
              ]
            },
            [
              [
                {
                  "Adt": [
                    "Just",
                    [
                      {
                        "Var": "value"
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      11247,
                      11280
                    ],
                    {
                      "Application": [
                        [
                          11247,
                          11280
                        ],
                        {
                          "Application": [
                            [
                              11247,
                              11280
                            ],
                            {
                              "Ref": [
                                [
                                  11247,
                                  11253
                                ],
                                "insert"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  11254,
                                  11263
                                ],
                                "targetKey"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              11264,
                              11269
                            ],
                            "value"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          11270,
                          11280
                        ],
                        "dictionary"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "Nothing",
                    []
                  ]
                },
                {
                  "Application": [
                    [
                      11303,
                      11330
                    ],
                    {
                      "Application": [
                        [
                          11303,
                          11330
                        ],
                        {
                          "Ref": [
                            [
                              11303,
                              11309
                            ],
                            "remove"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              11310,
                              11319
                            ],
                            "targetKey"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          11320,
                          11330
                        ],
                        "dictionary"
                      ]
                    }
                  ]
                }
              ]
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
              "Var": "comparable"
            },
            {
              "Fun": [
                {
                  "Var": "v"
                },
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "singleton",
        "patterns": [
          {
            "Var": "key"
          },
          {
            "Var": "value"
          }
        ],
        "expr": {
          "Application": [
            [
              11489,
              11563
            ],
            {
              "Application": [
                [
                  11489,
                  11563
                ],
                {
                  "Application": [
                    [
                      11489,
                      11563
                    ],
                    {
                      "Application": [
                        [
                          11489,
                          11563
                        ],
                        {
                          "Application": [
                            [
                              11489,
                              11563
                            ],
                            {
                              "Ref": [
                                [
                                  11489,
                                  11508
                                ],
                                "RBNode_elm_builtin"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  11508,
                                  11514
                                ],
                                "Black"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              11514,
                              11517
                            ],
                            "key"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          11518,
                          11523
                        ],
                        "value"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      11524,
                      11544
                    ],
                    "RBEmpty_elm_builtin"
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  11544,
                  11563
                ],
                "RBEmpty_elm_builtin"
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
                "Dict",
                [
                  {
                    "Var": "comparable"
                  },
                  {
                    "Var": "v"
                  }
                ]
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "union",
        "patterns": [
          {
            "Var": "t1"
          },
          {
            "Var": "t2"
          }
        ],
        "expr": {
          "Application": [
            [
              11766,
              11784
            ],
            {
              "Application": [
                [
                  11766,
                  11784
                ],
                {
                  "Application": [
                    [
                      11766,
                      11784
                    ],
                    {
                      "Ref": [
                        [
                          11766,
                          11771
                        ],
                        "foldl"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          11772,
                          11778
                        ],
                        "insert"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      11779,
                      11781
                    ],
                    "t2"
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  11782,
                  11784
                ],
                "t1"
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
                "Dict",
                [
                  {
                    "Var": "comparable"
                  },
                  {
                    "Var": "v"
                  }
                ]
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "intersect",
        "patterns": [
          {
            "Var": "t1"
          },
          {
            "Var": "t2"
          }
        ],
        "expr": {
          "Application": [
            [
              12010,
              12041
            ],
            {
              "Application": [
                [
                  12010,
                  12041
                ],
                {
                  "Ref": [
                    [
                      12010,
                      12016
                    ],
                    "filter"
                  ]
                },
                {
                  "Lambda": [
                    [
                      12018,
                      12037
                    ],
                    [
                      {
                        "Var": "k"
                      },
                      "Wildcard"
                    ],
                    {
                      "Application": [
                        [
                          12026,
                          12037
                        ],
                        {
                          "Application": [
                            [
                              12026,
                              12037
                            ],
                            {
                              "Ref": [
                                [
                                  12026,
                                  12032
                                ],
                                "member"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  12033,
                                  12034
                                ],
                                "k"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              12035,
                              12037
                            ],
                            "t2"
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  12039,
                  12041
                ],
                "t1"
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
                "Dict",
                [
                  {
                    "Var": "comparable"
                  },
                  {
                    "Var": "a"
                  }
                ]
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "diff",
        "patterns": [
          {
            "Var": "t1"
          },
          {
            "Var": "t2"
          }
        ],
        "expr": {
          "Application": [
            [
              12210,
              12244
            ],
            {
              "Application": [
                [
                  12210,
                  12244
                ],
                {
                  "Application": [
                    [
                      12210,
                      12244
                    ],
                    {
                      "Ref": [
                        [
                          12210,
                          12215
                        ],
                        "foldl"
                      ]
                    },
                    {
                      "Lambda": [
                        [
                          12217,
                          12237
                        ],
                        [
                          {
                            "Var": "k"
                          },
                          {
                            "Var": "v"
                          },
                          {
                            "Var": "t"
                          }
                        ],
                        {
                          "Application": [
                            [
                              12227,
                              12237
                            ],
                            {
                              "Application": [
                                [
                                  12227,
                                  12237
                                ],
                                {
                                  "Ref": [
                                    [
                                      12227,
                                      12233
                                    ],
                                    "remove"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      12234,
                                      12235
                                    ],
                                    "k"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  12236,
                                  12237
                                ],
                                "t"
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      12239,
                      12241
                    ],
                    "t1"
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  12242,
                  12244
                ],
                "t2"
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
                  "Var": "comparable"
                },
                {
                  "Fun": [
                    {
                      "Var": "a"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "result"
                        },
                        {
                          "Var": "result"
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Fun": [
                    {
                      "Var": "comparable"
                    },
                    {
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
                              "Fun": [
                                {
                                  "Var": "result"
                                },
                                {
                                  "Var": "result"
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "Fun": [
                    {
                      "Fun": [
                        {
                          "Var": "comparable"
                        },
                        {
                          "Fun": [
                            {
                              "Var": "b"
                            },
                            {
                              "Fun": [
                                {
                                  "Var": "result"
                                },
                                {
                                  "Var": "result"
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "Dict",
                            [
                              {
                                "Var": "comparable"
                              },
                              {
                                "Var": "a"
                              }
                            ]
                          ]
                        },
                        {
                          "Fun": [
                            {
                              "Tag": [
                                "Dict",
                                [
                                  {
                                    "Var": "comparable"
                                  },
                                  {
                                    "Var": "b"
                                  }
                                ]
                              ]
                            },
                            {
                              "Fun": [
                                {
                                  "Var": "result"
                                },
                                {
                                  "Var": "result"
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "merge",
        "patterns": [
          {
            "Var": "leftStep"
          },
          {
            "Var": "bothStep"
          },
          {
            "Var": "rightStep"
          },
          {
            "Var": "leftDict"
          },
          {
            "Var": "rightDict"
          },
          {
            "Var": "initialResult"
          }
        ],
        "expr": {
          "Let": [
            [
              12832,
              13441
            ],
            [
              {
                "Def": {
                  "header": null,
                  "name": "stepState",
                  "patterns": [
                    {
                      "Var": "rKey"
                    },
                    {
                      "Var": "rValue"
                    },
                    {
                      "Tuple": [
                        {
                          "Var": "list"
                        },
                        {
                          "Var": "result"
                        }
                      ]
                    }
                  ],
                  "expr": {
                    "Case": [
                      [
                        12885,
                        13249
                      ],
                      {
                        "Ref": [
                          [
                            12890,
                            12894
                          ],
                          "list"
                        ]
                      },
                      [
                        [
                          {
                            "List": []
                          },
                          {
                            "Tuple": [
                              [
                                12922,
                                12958
                              ],
                              [
                                {
                                  "Ref": [
                                    [
                                      12923,
                                      12927
                                    ],
                                    "list"
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      12929,
                                      12957
                                    ],
                                    {
                                      "Application": [
                                        [
                                          12929,
                                          12957
                                        ],
                                        {
                                          "Application": [
                                            [
                                              12929,
                                              12957
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  12929,
                                                  12938
                                                ],
                                                "rightStep"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  12939,
                                                  12943
                                                ],
                                                "rKey"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              12944,
                                              12950
                                            ],
                                            "rValue"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          12951,
                                          12957
                                        ],
                                        "result"
                                      ]
                                    }
                                  ]
                                }
                              ]
                            ]
                          }
                        ],
                        [
                          {
                            "BinaryOp": [
                              "::",
                              {
                                "Tuple": [
                                  {
                                    "Var": "lKey"
                                  },
                                  {
                                    "Var": "lValue"
                                  }
                                ]
                              },
                              {
                                "Var": "rest"
                              }
                            ]
                          },
                          {
                            "If": [
                              [
                                13004,
                                13249
                              ],
                              {
                                "OpChain": [
                                  [
                                    13007,
                                    13018
                                  ],
                                  [
                                    {
                                      "Ref": [
                                        [
                                          13007,
                                          13011
                                        ],
                                        "lKey"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          13014,
                                          13018
                                        ],
                                        "rKey"
                                      ]
                                    }
                                  ],
                                  [
                                    "<"
                                  ]
                                ]
                              },
                              {
                                "Application": [
                                  [
                                    13036,
                                    13093
                                  ],
                                  {
                                    "Application": [
                                      [
                                        13036,
                                        13093
                                      ],
                                      {
                                        "Application": [
                                          [
                                            13036,
                                            13093
                                          ],
                                          {
                                            "Ref": [
                                              [
                                                13036,
                                                13045
                                              ],
                                              "stepState"
                                            ]
                                          },
                                          {
                                            "Ref": [
                                              [
                                                13046,
                                                13050
                                              ],
                                              "rKey"
                                            ]
                                          }
                                        ]
                                      },
                                      {
                                        "Ref": [
                                          [
                                            13051,
                                            13057
                                          ],
                                          "rValue"
                                        ]
                                      }
                                    ]
                                  },
                                  {
                                    "Tuple": [
                                      [
                                        13058,
                                        13105
                                      ],
                                      [
                                        {
                                          "Ref": [
                                            [
                                              13059,
                                              13063
                                            ],
                                            "rest"
                                          ]
                                        },
                                        {
                                          "Application": [
                                            [
                                              13065,
                                              13092
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  13065,
                                                  13092
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      13065,
                                                      13092
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          13065,
                                                          13073
                                                        ],
                                                        "leftStep"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          13074,
                                                          13078
                                                        ],
                                                        "lKey"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      13079,
                                                      13085
                                                    ],
                                                    "lValue"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  13086,
                                                  13092
                                                ],
                                                "result"
                                              ]
                                            }
                                          ]
                                        }
                                      ]
                                    ]
                                  }
                                ]
                              },
                              {
                                "If": [
                                  [
                                    13110,
                                    13249
                                  ],
                                  {
                                    "OpChain": [
                                      [
                                        13113,
                                        13124
                                      ],
                                      [
                                        {
                                          "Ref": [
                                            [
                                              13113,
                                              13117
                                            ],
                                            "lKey"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              13120,
                                              13124
                                            ],
                                            "rKey"
                                          ]
                                        }
                                      ],
                                      [
                                        ">"
                                      ]
                                    ]
                                  },
                                  {
                                    "Tuple": [
                                      [
                                        13142,
                                        13190
                                      ],
                                      [
                                        {
                                          "Ref": [
                                            [
                                              13143,
                                              13147
                                            ],
                                            "list"
                                          ]
                                        },
                                        {
                                          "Application": [
                                            [
                                              13149,
                                              13177
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  13149,
                                                  13177
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      13149,
                                                      13177
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          13149,
                                                          13158
                                                        ],
                                                        "rightStep"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          13159,
                                                          13163
                                                        ],
                                                        "rKey"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      13164,
                                                      13170
                                                    ],
                                                    "rValue"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  13171,
                                                  13177
                                                ],
                                                "result"
                                              ]
                                            }
                                          ]
                                        }
                                      ]
                                    ]
                                  },
                                  {
                                    "Tuple": [
                                      [
                                        13207,
                                        13249
                                      ],
                                      [
                                        {
                                          "Ref": [
                                            [
                                              13208,
                                              13212
                                            ],
                                            "rest"
                                          ]
                                        },
                                        {
                                          "Application": [
                                            [
                                              13214,
                                              13248
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  13214,
                                                  13248
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      13214,
                                                      13248
                                                    ],
                                                    {
                                                      "Application": [
                                                        [
                                                          13214,
                                                          13248
                                                        ],
                                                        {
                                                          "Ref": [
                                                            [
                                                              13214,
                                                              13222
                                                            ],
                                                            "bothStep"
                                                          ]
                                                        },
                                                        {
                                                          "Ref": [
                                                            [
                                                              13223,
                                                              13227
                                                            ],
                                                            "lKey"
                                                          ]
                                                        }
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          13228,
                                                          13234
                                                        ],
                                                        "lValue"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      13235,
                                                      13241
                                                    ],
                                                    "rValue"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  13242,
                                                  13248
                                                ],
                                                "result"
                                              ]
                                            }
                                          ]
                                        }
                                      ]
                                    ]
                                  }
                                ]
                              }
                            ]
                          }
                        ]
                      ]
                    ]
                  }
                }
              },
              {
                "Pattern": [
                  {
                    "Tuple": [
                      {
                        "Var": "leftovers"
                      },
                      {
                        "Var": "intermediateResult"
                      }
                    ]
                  },
                  {
                    "Application": [
                      [
                        13295,
                        13353
                      ],
                      {
                        "Application": [
                          [
                            13295,
                            13353
                          ],
                          {
                            "Application": [
                              [
                                13295,
                                13353
                              ],
                              {
                                "Ref": [
                                  [
                                    13295,
                                    13300
                                  ],
                                  "foldl"
                                ]
                              },
                              {
                                "Ref": [
                                  [
                                    13301,
                                    13310
                                  ],
                                  "stepState"
                                ]
                              }
                            ]
                          },
                          {
                            "Tuple": [
                              [
                                13311,
                                13344
                              ],
                              [
                                {
                                  "Application": [
                                    [
                                      13312,
                                      13327
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          13312,
                                          13318
                                        ],
                                        "toList"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          13319,
                                          13327
                                        ],
                                        "leftDict"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      13329,
                                      13342
                                    ],
                                    "initialResult"
                                  ]
                                }
                              ]
                            ]
                          }
                        ]
                      },
                      {
                        "Ref": [
                          [
                            13344,
                            13353
                          ],
                          "rightDict"
                        ]
                      }
                    ]
                  }
                ]
              }
            ],
            {
              "Application": [
                [
                  13363,
                  13441
                ],
                {
                  "Application": [
                    [
                      13363,
                      13441
                    ],
                    {
                      "Application": [
                        [
                          13363,
                          13441
                        ],
                        {
                          "QualifiedRef": [
                            [
                              13363,
                              13374
                            ],
                            [
                              "List"
                            ],
                            "foldl"
                          ]
                        },
                        {
                          "Lambda": [
                            [
                              13375,
                              13411
                            ],
                            [
                              {
                                "Tuple": [
                                  {
                                    "Var": "k"
                                  },
                                  {
                                    "Var": "v"
                                  }
                                ]
                              },
                              {
                                "Var": "result"
                              }
                            ],
                            {
                              "Application": [
                                [
                                  13392,
                                  13411
                                ],
                                {
                                  "Application": [
                                    [
                                      13392,
                                      13411
                                    ],
                                    {
                                      "Application": [
                                        [
                                          13392,
                                          13411
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              13392,
                                              13400
                                            ],
                                            "leftStep"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              13401,
                                              13402
                                            ],
                                            "k"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          13403,
                                          13404
                                        ],
                                        "v"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      13405,
                                      13411
                                    ],
                                    "result"
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          13413,
                          13431
                        ],
                        "intermediateResult"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      13432,
                      13441
                    ],
                    "leftovers"
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
                  "Var": "k"
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
            {
              "Fun": [
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "k"
                      },
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "k"
                      },
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "map",
        "patterns": [
          {
            "Var": "func"
          },
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Case": [
            [
              13577,
              13781
            ],
            {
              "Ref": [
                [
                  13582,
                  13586
                ],
                "dict"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "RBEmpty_elm_builtin",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      13623,
                      13642
                    ],
                    "RBEmpty_elm_builtin"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      {
                        "Var": "color"
                      },
                      {
                        "Var": "key"
                      },
                      {
                        "Var": "value"
                      },
                      {
                        "Var": "left"
                      },
                      {
                        "Var": "right"
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      13703,
                      13781
                    ],
                    {
                      "Application": [
                        [
                          13703,
                          13781
                        ],
                        {
                          "Application": [
                            [
                              13703,
                              13781
                            ],
                            {
                              "Application": [
                                [
                                  13703,
                                  13781
                                ],
                                {
                                  "Application": [
                                    [
                                      13703,
                                      13781
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          13703,
                                          13722
                                        ],
                                        "RBNode_elm_builtin"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          13722,
                                          13727
                                        ],
                                        "color"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      13728,
                                      13731
                                    ],
                                    "key"
                                  ]
                                }
                              ]
                            },
                            {
                              "Application": [
                                [
                                  13733,
                                  13747
                                ],
                                {
                                  "Application": [
                                    [
                                      13733,
                                      13747
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          13733,
                                          13737
                                        ],
                                        "func"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          13738,
                                          13741
                                        ],
                                        "key"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      13742,
                                      13747
                                    ],
                                    "value"
                                  ]
                                }
                              ]
                            }
                          ]
                        },
                        {
                          "Application": [
                            [
                              13750,
                              13763
                            ],
                            {
                              "Application": [
                                [
                                  13750,
                                  13763
                                ],
                                {
                                  "Ref": [
                                    [
                                      13750,
                                      13753
                                    ],
                                    "map"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      13754,
                                      13758
                                    ],
                                    "func"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  13759,
                                  13763
                                ],
                                "left"
                              ]
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "Application": [
                        [
                          13766,
                          13780
                        ],
                        {
                          "Application": [
                            [
                              13766,
                              13780
                            ],
                            {
                              "Ref": [
                                [
                                  13766,
                                  13769
                                ],
                                "map"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  13770,
                                  13774
                                ],
                                "func"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              13775,
                              13780
                            ],
                            "right"
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
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
              "Fun": [
                {
                  "Var": "k"
                },
                {
                  "Fun": [
                    {
                      "Var": "v"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "b"
                        },
                        {
                          "Var": "b"
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Var": "b"
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Dict",
                        [
                          {
                            "Var": "k"
                          },
                          {
                            "Var": "v"
                          }
                        ]
                      ]
                    },
                    {
                      "Var": "b"
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "foldl",
        "patterns": [
          {
            "Var": "func"
          },
          {
            "Var": "acc"
          },
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Case": [
            [
              14218,
              14379
            ],
            {
              "Ref": [
                [
                  14223,
                  14227
                ],
                "dict"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "RBEmpty_elm_builtin",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      14264,
                      14267
                    ],
                    "acc"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      "Wildcard",
                      {
                        "Var": "key"
                      },
                      {
                        "Var": "value"
                      },
                      {
                        "Var": "left"
                      },
                      {
                        "Var": "right"
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      14324,
                      14379
                    ],
                    {
                      "Application": [
                        [
                          14324,
                          14379
                        ],
                        {
                          "Application": [
                            [
                              14324,
                              14379
                            ],
                            {
                              "Ref": [
                                [
                                  14324,
                                  14329
                                ],
                                "foldl"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  14330,
                                  14334
                                ],
                                "func"
                              ]
                            }
                          ]
                        },
                        {
                          "Application": [
                            [
                              14336,
                              14372
                            ],
                            {
                              "Application": [
                                [
                                  14336,
                                  14372
                                ],
                                {
                                  "Application": [
                                    [
                                      14336,
                                      14372
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          14336,
                                          14340
                                        ],
                                        "func"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          14341,
                                          14344
                                        ],
                                        "key"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      14345,
                                      14350
                                    ],
                                    "value"
                                  ]
                                }
                              ]
                            },
                            {
                              "Application": [
                                [
                                  14352,
                                  14371
                                ],
                                {
                                  "Application": [
                                    [
                                      14352,
                                      14371
                                    ],
                                    {
                                      "Application": [
                                        [
                                          14352,
                                          14371
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              14352,
                                              14357
                                            ],
                                            "foldl"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              14358,
                                              14362
                                            ],
                                            "func"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          14363,
                                          14366
                                        ],
                                        "acc"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      14367,
                                      14371
                                    ],
                                    "left"
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          14374,
                          14379
                        ],
                        "right"
                      ]
                    }
                  ]
                }
              ]
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
              "Fun": [
                {
                  "Var": "k"
                },
                {
                  "Fun": [
                    {
                      "Var": "v"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "b"
                        },
                        {
                          "Var": "b"
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Var": "b"
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Dict",
                        [
                          {
                            "Var": "k"
                          },
                          {
                            "Var": "v"
                          }
                        ]
                      ]
                    },
                    {
                      "Var": "b"
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "foldr",
        "patterns": [
          {
            "Var": "func"
          },
          {
            "Var": "acc"
          },
          {
            "Var": "t"
          }
        ],
        "expr": {
          "Case": [
            [
              14813,
              14971
            ],
            {
              "Ref": [
                [
                  14818,
                  14819
                ],
                "t"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "RBEmpty_elm_builtin",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      14856,
                      14859
                    ],
                    "acc"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "RBNode_elm_builtin",
                    [
                      "Wildcard",
                      {
                        "Var": "key"
                      },
                      {
                        "Var": "value"
                      },
                      {
                        "Var": "left"
                      },
                      {
                        "Var": "right"
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      14916,
                      14971
                    ],
                    {
                      "Application": [
                        [
                          14916,
                          14971
                        ],
                        {
                          "Application": [
                            [
                              14916,
                              14971
                            ],
                            {
                              "Ref": [
                                [
                                  14916,
                                  14921
                                ],
                                "foldr"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  14922,
                                  14926
                                ],
                                "func"
                              ]
                            }
                          ]
                        },
                        {
                          "Application": [
                            [
                              14928,
                              14965
                            ],
                            {
                              "Application": [
                                [
                                  14928,
                                  14965
                                ],
                                {
                                  "Application": [
                                    [
                                      14928,
                                      14965
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          14928,
                                          14932
                                        ],
                                        "func"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          14933,
                                          14936
                                        ],
                                        "key"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      14937,
                                      14942
                                    ],
                                    "value"
                                  ]
                                }
                              ]
                            },
                            {
                              "Application": [
                                [
                                  14944,
                                  14964
                                ],
                                {
                                  "Application": [
                                    [
                                      14944,
                                      14964
                                    ],
                                    {
                                      "Application": [
                                        [
                                          14944,
                                          14964
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              14944,
                                              14949
                                            ],
                                            "foldr"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              14950,
                                              14954
                                            ],
                                            "func"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          14955,
                                          14958
                                        ],
                                        "acc"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      14959,
                                      14964
                                    ],
                                    "right"
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          14967,
                          14971
                        ],
                        "left"
                      ]
                    }
                  ]
                }
              ]
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
              "Fun": [
                {
                  "Var": "comparable"
                },
                {
                  "Fun": [
                    {
                      "Var": "v"
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
            {
              "Fun": [
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "filter",
        "patterns": [
          {
            "Var": "isGood"
          },
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Application": [
            [
              15137,
              15204
            ],
            {
              "Application": [
                [
                  15137,
                  15204
                ],
                {
                  "Application": [
                    [
                      15137,
                      15204
                    ],
                    {
                      "Ref": [
                        [
                          15137,
                          15142
                        ],
                        "foldl"
                      ]
                    },
                    {
                      "Lambda": [
                        [
                          15144,
                          15192
                        ],
                        [
                          {
                            "Var": "k"
                          },
                          {
                            "Var": "v"
                          },
                          {
                            "Var": "d"
                          }
                        ],
                        {
                          "If": [
                            [
                              15154,
                              15192
                            ],
                            {
                              "Application": [
                                [
                                  15157,
                                  15167
                                ],
                                {
                                  "Application": [
                                    [
                                      15157,
                                      15167
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          15157,
                                          15163
                                        ],
                                        "isGood"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          15164,
                                          15165
                                        ],
                                        "k"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      15166,
                                      15167
                                    ],
                                    "v"
                                  ]
                                }
                              ]
                            },
                            {
                              "Application": [
                                [
                                  15173,
                                  15185
                                ],
                                {
                                  "Application": [
                                    [
                                      15173,
                                      15185
                                    ],
                                    {
                                      "Application": [
                                        [
                                          15173,
                                          15185
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              15173,
                                              15179
                                            ],
                                            "insert"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              15180,
                                              15181
                                            ],
                                            "k"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          15182,
                                          15183
                                        ],
                                        "v"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      15184,
                                      15185
                                    ],
                                    "d"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  15191,
                                  15192
                                ],
                                "d"
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      15194,
                      15199
                    ],
                    "empty"
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  15200,
                  15204
                ],
                "dict"
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
                  "Var": "comparable"
                },
                {
                  "Fun": [
                    {
                      "Var": "v"
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
            {
              "Fun": [
                {
                  "Tag": [
                    "Dict",
                    [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  ]
                },
                {
                  "Tuple": [
                    {
                      "Tag": [
                        "Dict",
                        [
                          {
                            "Var": "comparable"
                          },
                          {
                            "Var": "v"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "Dict",
                        [
                          {
                            "Var": "comparable"
                          },
                          {
                            "Var": "v"
                          }
                        ]
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "partition",
        "patterns": [
          {
            "Var": "isGood"
          },
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Let": [
            [
              15509,
              15691
            ],
            [
              {
                "Def": {
                  "header": null,
                  "name": "add",
                  "patterns": [
                    {
                      "Var": "key"
                    },
                    {
                      "Var": "value"
                    },
                    {
                      "Tuple": [
                        {
                          "Var": "t1"
                        },
                        {
                          "Var": "t2"
                        }
                      ]
                    }
                  ],
                  "expr": {
                    "If": [
                      [
                        15548,
                        15655
                      ],
                      {
                        "Application": [
                          [
                            15551,
                            15567
                          ],
                          {
                            "Application": [
                              [
                                15551,
                                15567
                              ],
                              {
                                "Ref": [
                                  [
                                    15551,
                                    15557
                                  ],
                                  "isGood"
                                ]
                              },
                              {
                                "Ref": [
                                  [
                                    15558,
                                    15561
                                  ],
                                  "key"
                                ]
                              }
                            ]
                          },
                          {
                            "Ref": [
                              [
                                15562,
                                15567
                              ],
                              "value"
                            ]
                          }
                        ]
                      },
                      {
                        "Tuple": [
                          [
                            15581,
                            15614
                          ],
                          [
                            {
                              "Application": [
                                [
                                  15582,
                                  15601
                                ],
                                {
                                  "Application": [
                                    [
                                      15582,
                                      15601
                                    ],
                                    {
                                      "Application": [
                                        [
                                          15582,
                                          15601
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              15582,
                                              15588
                                            ],
                                            "insert"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              15589,
                                              15592
                                            ],
                                            "key"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          15593,
                                          15598
                                        ],
                                        "value"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      15599,
                                      15601
                                    ],
                                    "t1"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  15603,
                                  15605
                                ],
                                "t2"
                              ]
                            }
                          ]
                        ]
                      },
                      {
                        "Tuple": [
                          [
                            15627,
                            15655
                          ],
                          [
                            {
                              "Ref": [
                                [
                                  15628,
                                  15630
                                ],
                                "t1"
                              ]
                            },
                            {
                              "Application": [
                                [
                                  15632,
                                  15651
                                ],
                                {
                                  "Application": [
                                    [
                                      15632,
                                      15651
                                    ],
                                    {
                                      "Application": [
                                        [
                                          15632,
                                          15651
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              15632,
                                              15638
                                            ],
                                            "insert"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              15639,
                                              15642
                                            ],
                                            "key"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          15643,
                                          15648
                                        ],
                                        "value"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      15649,
                                      15651
                                    ],
                                    "t2"
                                  ]
                                }
                              ]
                            }
                          ]
                        ]
                      }
                    ]
                  }
                }
              }
            ],
            {
              "Application": [
                [
                  15662,
                  15691
                ],
                {
                  "Application": [
                    [
                      15662,
                      15691
                    ],
                    {
                      "Application": [
                        [
                          15662,
                          15691
                        ],
                        {
                          "Ref": [
                            [
                              15662,
                              15667
                            ],
                            "foldl"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              15668,
                              15671
                            ],
                            "add"
                          ]
                        }
                      ]
                    },
                    {
                      "Tuple": [
                        [
                          15672,
                          15687
                        ],
                        [
                          {
                            "Ref": [
                              [
                                15673,
                                15678
                              ],
                              "empty"
                            ]
                          },
                          {
                            "Ref": [
                              [
                                15680,
                                15685
                              ],
                              "empty"
                            ]
                          }
                        ]
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      15687,
                      15691
                    ],
                    "dict"
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
                "Dict",
                [
                  {
                    "Var": "k"
                  },
                  {
                    "Var": "v"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "k"
                  }
                ]
              ]
            }
          ]
        },
        "name": "keys",
        "patterns": [
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Application": [
            [
              15874,
              15926
            ],
            {
              "Application": [
                [
                  15874,
                  15926
                ],
                {
                  "Application": [
                    [
                      15874,
                      15926
                    ],
                    {
                      "Ref": [
                        [
                          15874,
                          15879
                        ],
                        "foldr"
                      ]
                    },
                    {
                      "Lambda": [
                        [
                          15881,
                          15917
                        ],
                        [
                          {
                            "Var": "key"
                          },
                          {
                            "Var": "value"
                          },
                          {
                            "Var": "keyList"
                          }
                        ],
                        {
                          "OpChain": [
                            [
                              15903,
                              15917
                            ],
                            [
                              {
                                "Ref": [
                                  [
                                    15903,
                                    15906
                                  ],
                                  "key"
                                ]
                              },
                              {
                                "Ref": [
                                  [
                                    15910,
                                    15917
                                  ],
                                  "keyList"
                                ]
                              }
                            ],
                            [
                              "::"
                            ]
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "List": [
                    [
                      15919,
                      15922
                    ],
                    []
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  15922,
                  15926
                ],
                "dict"
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
                "Dict",
                [
                  {
                    "Var": "k"
                  },
                  {
                    "Var": "v"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "v"
                  }
                ]
              ]
            }
          ]
        },
        "name": "values",
        "patterns": [
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Application": [
            [
              16114,
              16172
            ],
            {
              "Application": [
                [
                  16114,
                  16172
                ],
                {
                  "Application": [
                    [
                      16114,
                      16172
                    ],
                    {
                      "Ref": [
                        [
                          16114,
                          16119
                        ],
                        "foldr"
                      ]
                    },
                    {
                      "Lambda": [
                        [
                          16121,
                          16163
                        ],
                        [
                          {
                            "Var": "key"
                          },
                          {
                            "Var": "value"
                          },
                          {
                            "Var": "valueList"
                          }
                        ],
                        {
                          "OpChain": [
                            [
                              16145,
                              16163
                            ],
                            [
                              {
                                "Ref": [
                                  [
                                    16145,
                                    16150
                                  ],
                                  "value"
                                ]
                              },
                              {
                                "Ref": [
                                  [
                                    16154,
                                    16163
                                  ],
                                  "valueList"
                                ]
                              }
                            ],
                            [
                              "::"
                            ]
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "List": [
                    [
                      16165,
                      16168
                    ],
                    []
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  16168,
                  16172
                ],
                "dict"
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
                "Dict",
                [
                  {
                    "Var": "k"
                  },
                  {
                    "Var": "v"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "List",
                [
                  {
                    "Tuple": [
                      {
                        "Var": "k"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  }
                ]
              ]
            }
          ]
        },
        "name": "toList",
        "patterns": [
          {
            "Var": "dict"
          }
        ],
        "expr": {
          "Application": [
            [
              16312,
              16366
            ],
            {
              "Application": [
                [
                  16312,
                  16366
                ],
                {
                  "Application": [
                    [
                      16312,
                      16366
                    ],
                    {
                      "Ref": [
                        [
                          16312,
                          16317
                        ],
                        "foldr"
                      ]
                    },
                    {
                      "Lambda": [
                        [
                          16319,
                          16357
                        ],
                        [
                          {
                            "Var": "key"
                          },
                          {
                            "Var": "value"
                          },
                          {
                            "Var": "list"
                          }
                        ],
                        {
                          "OpChain": [
                            [
                              16338,
                              16357
                            ],
                            [
                              {
                                "Tuple": [
                                  [
                                    16338,
                                    16350
                                  ],
                                  [
                                    {
                                      "Ref": [
                                        [
                                          16339,
                                          16342
                                        ],
                                        "key"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          16343,
                                          16348
                                        ],
                                        "value"
                                      ]
                                    }
                                  ]
                                ]
                              },
                              {
                                "Ref": [
                                  [
                                    16353,
                                    16357
                                  ],
                                  "list"
                                ]
                              }
                            ],
                            [
                              "::"
                            ]
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "List": [
                    [
                      16359,
                      16362
                    ],
                    []
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  16362,
                  16366
                ],
                "dict"
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
                "List",
                [
                  {
                    "Tuple": [
                      {
                        "Var": "comparable"
                      },
                      {
                        "Var": "v"
                      }
                    ]
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Dict",
                [
                  {
                    "Var": "comparable"
                  },
                  {
                    "Var": "v"
                  }
                ]
              ]
            }
          ]
        },
        "name": "fromList",
        "patterns": [
          {
            "Var": "assocs"
          }
        ],
        "expr": {
          "Application": [
            [
              16495,
              16563
            ],
            {
              "Application": [
                [
                  16495,
                  16563
                ],
                {
                  "Application": [
                    [
                      16495,
                      16563
                    ],
                    {
                      "QualifiedRef": [
                        [
                          16495,
                          16506
                        ],
                        [
                          "List"
                        ],
                        "foldl"
                      ]
                    },
                    {
                      "Lambda": [
                        [
                          16507,
                          16549
                        ],
                        [
                          {
                            "Tuple": [
                              {
                                "Var": "key"
                              },
                              {
                                "Var": "value"
                              }
                            ]
                          },
                          {
                            "Var": "dict"
                          }
                        ],
                        {
                          "Application": [
                            [
                              16528,
                              16549
                            ],
                            {
                              "Application": [
                                [
                                  16528,
                                  16549
                                ],
                                {
                                  "Application": [
                                    [
                                      16528,
                                      16549
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          16528,
                                          16534
                                        ],
                                        "insert"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          16535,
                                          16538
                                        ],
                                        "key"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      16539,
                                      16544
                                    ],
                                    "value"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  16545,
                                  16549
                                ],
                                "dict"
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      16551,
                      16556
                    ],
                    "empty"
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  16557,
                  16563
                ],
                "assocs"
              ]
            }
          ]
        }
      }
    }
  ]
}
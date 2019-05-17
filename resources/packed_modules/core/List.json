{
  "header": {
    "name": "List",
    "exposing": {
      "Just": [
        {
          "Definition": "singleton"
        },
        {
          "Definition": "repeat"
        },
        {
          "Definition": "range"
        },
        {
          "BinaryOperator": "::"
        },
        {
          "Definition": "map"
        },
        {
          "Definition": "indexedMap"
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
          "Definition": "filterMap"
        },
        {
          "Definition": "length"
        },
        {
          "Definition": "reverse"
        },
        {
          "Definition": "member"
        },
        {
          "Definition": "all"
        },
        {
          "Definition": "any"
        },
        {
          "Definition": "maximum"
        },
        {
          "Definition": "minimum"
        },
        {
          "Definition": "sum"
        },
        {
          "Definition": "product"
        },
        {
          "Definition": "append"
        },
        {
          "Definition": "concat"
        },
        {
          "Definition": "concatMap"
        },
        {
          "Definition": "intersperse"
        },
        {
          "Definition": "map2"
        },
        {
          "Definition": "map3"
        },
        {
          "Definition": "map4"
        },
        {
          "Definition": "map5"
        },
        {
          "Definition": "sort"
        },
        {
          "Definition": "sortBy"
        },
        {
          "Definition": "sortWith"
        },
        {
          "Definition": "isEmpty"
        },
        {
          "Definition": "head"
        },
        {
          "Definition": "tail"
        },
        {
          "Definition": "take"
        },
        {
          "Definition": "drop"
        },
        {
          "Definition": "partition"
        },
        {
          "Definition": "unzip"
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
        "Elm",
        "Kernel",
        "List"
      ],
      "alias": null,
      "exposing": null
    },
    {
      "path": [
        "Maybe"
      ],
      "alias": null,
      "exposing": {
        "Just": [
          {
            "Adt": [
              "Maybe",
              "All"
            ]
          }
        ]
      }
    }
  ],
  "statements": [
    {
      "Adt": [
        "List",
        [
          "a"
        ],
        [
          [
            [
              993,
              999
            ],
            "List",
            [
              {
                "Var": "a"
              }
            ]
          ]
        ]
      ]
    },
    {
      "Infix": [
        "right",
        5,
        "::",
        "cons"
      ]
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "a"
            },
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            }
          ]
        },
        "name": "singleton",
        "patterns": [
          {
            "Var": [
              [
                1180,
                1185
              ],
              "value"
            ]
          }
        ],
        "expr": {
          "List": [
            [
              1190,
              1197
            ],
            [
              {
                "Ref": [
                  [
                    1191,
                    1196
                  ],
                  "value"
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
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Tag": [
                    "List",
                    [
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
        "name": "repeat",
        "patterns": [
          {
            "Var": [
              [
                1327,
                1328
              ],
              "n"
            ]
          },
          {
            "Var": [
              [
                1329,
                1334
              ],
              "value"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              1339,
              1360
            ],
            {
              "Application": [
                [
                  1339,
                  1360
                ],
                {
                  "Application": [
                    [
                      1339,
                      1360
                    ],
                    {
                      "Ref": [
                        [
                          1339,
                          1349
                        ],
                        "repeatHelp"
                      ]
                    },
                    {
                      "List": [
                        [
                          1350,
                          1353
                        ],
                        []
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      1353,
                      1354
                    ],
                    "n"
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  1355,
                  1360
                ],
                "value"
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
                    "Var": "a"
                  }
                ]
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
                  "Fun": [
                    {
                      "Var": "a"
                    },
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "a"
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
        "name": "repeatHelp",
        "patterns": [
          {
            "Var": [
              [
                1416,
                1422
              ],
              "result"
            ]
          },
          {
            "Var": [
              [
                1423,
                1424
              ],
              "n"
            ]
          },
          {
            "Var": [
              [
                1425,
                1430
              ],
              "value"
            ]
          }
        ],
        "expr": {
          "If": [
            [
              1435,
              1515
            ],
            {
              "OpChain": [
                [
                  1438,
                  1444
                ],
                [
                  {
                    "Ref": [
                      [
                        1438,
                        1439
                      ],
                      "n"
                    ]
                  },
                  {
                    "Literal": [
                      [
                        1443,
                        1444
                      ],
                      {
                        "Int": 0
                      }
                    ]
                  }
                ],
                [
                  "<="
                ]
              ]
            },
            {
              "Ref": [
                [
                  1454,
                  1460
                ],
                "result"
              ]
            },
            {
              "Application": [
                [
                  1473,
                  1515
                ],
                {
                  "Application": [
                    [
                      1473,
                      1515
                    ],
                    {
                      "Application": [
                        [
                          1473,
                          1515
                        ],
                        {
                          "Ref": [
                            [
                              1473,
                              1483
                            ],
                            "repeatHelp"
                          ]
                        },
                        {
                          "Application": [
                            [
                              1485,
                              1502
                            ],
                            {
                              "Application": [
                                [
                                  1485,
                                  1502
                                ],
                                {
                                  "Ref": [
                                    [
                                      1485,
                                      1489
                                    ],
                                    "cons"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      1490,
                                      1495
                                    ],
                                    "value"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  1496,
                                  1502
                                ],
                                "result"
                              ]
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "OpChain": [
                        [
                          1505,
                          1508
                        ],
                        [
                          {
                            "Ref": [
                              [
                                1505,
                                1506
                              ],
                              "n"
                            ]
                          },
                          {
                            "Literal": [
                              [
                                1507,
                                1508
                              ],
                              {
                                "Int": 1
                              }
                            ]
                          }
                        ],
                        [
                          "-"
                        ]
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      1510,
                      1515
                    ],
                    "value"
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
                    "List",
                    [
                      {
                        "Tag": [
                          "Int",
                          []
                        ]
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "range",
        "patterns": [
          {
            "Var": [
              [
                1760,
                1762
              ],
              "lo"
            ]
          },
          {
            "Var": [
              [
                1763,
                1765
              ],
              "hi"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              1770,
              1788
            ],
            {
              "Application": [
                [
                  1770,
                  1788
                ],
                {
                  "Application": [
                    [
                      1770,
                      1788
                    ],
                    {
                      "Ref": [
                        [
                          1770,
                          1779
                        ],
                        "rangeHelp"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          1780,
                          1782
                        ],
                        "lo"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      1783,
                      1785
                    ],
                    "hi"
                  ]
                }
              ]
            },
            {
              "List": [
                [
                  1786,
                  1788
                ],
                []
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
                    "Int",
                    []
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Tag": [
                              "Int",
                              []
                            ]
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Tag": [
                              "Int",
                              []
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
        },
        "name": "rangeHelp",
        "patterns": [
          {
            "Var": [
              [
                1848,
                1850
              ],
              "lo"
            ]
          },
          {
            "Var": [
              [
                1851,
                1853
              ],
              "hi"
            ]
          },
          {
            "Var": [
              [
                1854,
                1858
              ],
              "list"
            ]
          }
        ],
        "expr": {
          "If": [
            [
              1863,
              1937
            ],
            {
              "OpChain": [
                [
                  1866,
                  1874
                ],
                [
                  {
                    "Ref": [
                      [
                        1866,
                        1868
                      ],
                      "lo"
                    ]
                  },
                  {
                    "Ref": [
                      [
                        1872,
                        1874
                      ],
                      "hi"
                    ]
                  }
                ],
                [
                  "<="
                ]
              ]
            },
            {
              "Application": [
                [
                  1884,
                  1920
                ],
                {
                  "Application": [
                    [
                      1884,
                      1920
                    ],
                    {
                      "Application": [
                        [
                          1884,
                          1920
                        ],
                        {
                          "Ref": [
                            [
                              1884,
                              1893
                            ],
                            "rangeHelp"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              1894,
                              1896
                            ],
                            "lo"
                          ]
                        }
                      ]
                    },
                    {
                      "OpChain": [
                        [
                          1898,
                          1904
                        ],
                        [
                          {
                            "Ref": [
                              [
                                1898,
                                1900
                              ],
                              "hi"
                            ]
                          },
                          {
                            "Literal": [
                              [
                                1903,
                                1904
                              ],
                              {
                                "Int": 1
                              }
                            ]
                          }
                        ],
                        [
                          "-"
                        ]
                      ]
                    }
                  ]
                },
                {
                  "Application": [
                    [
                      1907,
                      1919
                    ],
                    {
                      "Application": [
                        [
                          1907,
                          1919
                        ],
                        {
                          "Ref": [
                            [
                              1907,
                              1911
                            ],
                            "cons"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              1912,
                              1914
                            ],
                            "hi"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          1915,
                          1919
                        ],
                        "list"
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  1933,
                  1937
                ],
                "list"
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
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
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
        "name": "cons",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              2190,
              2210
            ],
            [
              "Elm",
              "Kernel",
              "List"
            ],
            "cons"
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
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
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
            "Var": [
              [
                2477,
                2478
              ],
              "f"
            ]
          },
          {
            "Var": [
              [
                2479,
                2481
              ],
              "xs"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              2486,
              2524
            ],
            {
              "Application": [
                [
                  2486,
                  2524
                ],
                {
                  "Application": [
                    [
                      2486,
                      2524
                    ],
                    {
                      "Ref": [
                        [
                          2486,
                          2491
                        ],
                        "foldr"
                      ]
                    },
                    {
                      "Lambda": [
                        [
                          2493,
                          2517
                        ],
                        [
                          {
                            "Var": [
                              [
                                2494,
                                2495
                              ],
                              "x"
                            ]
                          },
                          {
                            "Var": [
                              [
                                2496,
                                2499
                              ],
                              "acc"
                            ]
                          }
                        ],
                        {
                          "Application": [
                            [
                              2503,
                              2517
                            ],
                            {
                              "Application": [
                                [
                                  2503,
                                  2517
                                ],
                                {
                                  "Ref": [
                                    [
                                      2503,
                                      2507
                                    ],
                                    "cons"
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      2509,
                                      2512
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          2509,
                                          2510
                                        ],
                                        "f"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          2511,
                                          2512
                                        ],
                                        "x"
                                      ]
                                    }
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  2514,
                                  2517
                                ],
                                "acc"
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "List": [
                    [
                      2519,
                      2522
                    ],
                    []
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  2522,
                  2524
                ],
                "xs"
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
                  "Tag": [
                    "Int",
                    []
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
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
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
        "name": "indexedMap",
        "patterns": [
          {
            "Var": [
              [
                2776,
                2777
              ],
              "f"
            ]
          },
          {
            "Var": [
              [
                2778,
                2780
              ],
              "xs"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              2785,
              2820
            ],
            {
              "Application": [
                [
                  2785,
                  2820
                ],
                {
                  "Application": [
                    [
                      2785,
                      2820
                    ],
                    {
                      "Ref": [
                        [
                          2785,
                          2789
                        ],
                        "map2"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          2790,
                          2791
                        ],
                        "f"
                      ]
                    }
                  ]
                },
                {
                  "Application": [
                    [
                      2793,
                      2816
                    ],
                    {
                      "Application": [
                        [
                          2793,
                          2816
                        ],
                        {
                          "Ref": [
                            [
                              2793,
                              2798
                            ],
                            "range"
                          ]
                        },
                        {
                          "Literal": [
                            [
                              2799,
                              2800
                            ],
                            {
                              "Int": 0
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "OpChain": [
                        [
                          2802,
                          2815
                        ],
                        [
                          {
                            "Application": [
                              [
                                2802,
                                2811
                              ],
                              {
                                "Ref": [
                                  [
                                    2802,
                                    2808
                                  ],
                                  "length"
                                ]
                              },
                              {
                                "Ref": [
                                  [
                                    2809,
                                    2811
                                  ],
                                  "xs"
                                ]
                              }
                            ]
                          },
                          {
                            "Literal": [
                              [
                                2814,
                                2815
                              ],
                              {
                                "Int": 1
                              }
                            ]
                          }
                        ],
                        [
                          "-"
                        ]
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  2818,
                  2820
                ],
                "xs"
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
                        "List",
                        [
                          {
                            "Var": "a"
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
            "Var": [
              [
                3082,
                3086
              ],
              "func"
            ]
          },
          {
            "Var": [
              [
                3087,
                3090
              ],
              "acc"
            ]
          },
          {
            "Var": [
              [
                3091,
                3095
              ],
              "list"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              3100,
              3181
            ],
            {
              "Ref": [
                [
                  3105,
                  3109
                ],
                "list"
              ]
            },
            [
              [
                {
                  "List": [
                    [
                      3117,
                      3119
                    ],
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      3129,
                      3132
                    ],
                    "acc"
                  ]
                }
              ],
              [
                {
                  "BinaryOp": [
                    [
                      3138,
                      3145
                    ],
                    "::",
                    {
                      "Var": [
                        [
                          3138,
                          3139
                        ],
                        "x"
                      ]
                    },
                    {
                      "Var": [
                        [
                          3143,
                          3145
                        ],
                        "xs"
                      ]
                    }
                  ]
                },
                {
                  "Application": [
                    [
                      3155,
                      3181
                    ],
                    {
                      "Application": [
                        [
                          3155,
                          3181
                        ],
                        {
                          "Application": [
                            [
                              3155,
                              3181
                            ],
                            {
                              "Ref": [
                                [
                                  3155,
                                  3160
                                ],
                                "foldl"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  3161,
                                  3165
                                ],
                                "func"
                              ]
                            }
                          ]
                        },
                        {
                          "Application": [
                            [
                              3167,
                              3177
                            ],
                            {
                              "Application": [
                                [
                                  3167,
                                  3177
                                ],
                                {
                                  "Ref": [
                                    [
                                      3167,
                                      3171
                                    ],
                                    "func"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      3172,
                                      3173
                                    ],
                                    "x"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  3174,
                                  3177
                                ],
                                "acc"
                              ]
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          3179,
                          3181
                        ],
                        "xs"
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
                  "Var": "a"
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
                        "List",
                        [
                          {
                            "Var": "a"
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
            "Var": [
              [
                3444,
                3446
              ],
              "fn"
            ]
          },
          {
            "Var": [
              [
                3447,
                3450
              ],
              "acc"
            ]
          },
          {
            "Var": [
              [
                3451,
                3453
              ],
              "ls"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              3460,
              3483
            ],
            {
              "Application": [
                [
                  3460,
                  3483
                ],
                {
                  "Application": [
                    [
                      3460,
                      3483
                    ],
                    {
                      "Application": [
                        [
                          3460,
                          3483
                        ],
                        {
                          "Ref": [
                            [
                              3460,
                              3471
                            ],
                            "foldrHelper"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              3472,
                              3474
                            ],
                            "fn"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          3475,
                          3478
                        ],
                        "acc"
                      ]
                    }
                  ]
                },
                {
                  "Literal": [
                    [
                      3479,
                      3480
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
                  3481,
                  3483
                ],
                "ls"
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
                        "Int",
                        []
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "List",
                            [
                              {
                                "Var": "a"
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
            }
          ]
        },
        "name": "foldrHelper",
        "patterns": [
          {
            "Var": [
              [
                3553,
                3555
              ],
              "fn"
            ]
          },
          {
            "Var": [
              [
                3556,
                3559
              ],
              "acc"
            ]
          },
          {
            "Var": [
              [
                3560,
                3563
              ],
              "ctr"
            ]
          },
          {
            "Var": [
              [
                3564,
                3566
              ],
              "ls"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              3573,
              4516
            ],
            {
              "Ref": [
                [
                  3578,
                  3580
                ],
                "ls"
              ]
            },
            [
              [
                {
                  "List": [
                    [
                      3592,
                      3594
                    ],
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      3610,
                      3613
                    ],
                    "acc"
                  ]
                }
              ],
              [
                {
                  "BinaryOp": [
                    [
                      3623,
                      3630
                    ],
                    "::",
                    {
                      "Var": [
                        [
                          3623,
                          3624
                        ],
                        "a"
                      ]
                    },
                    {
                      "Var": [
                        [
                          3628,
                          3630
                        ],
                        "r1"
                      ]
                    }
                  ]
                },
                {
                  "Case": [
                    [
                      3646,
                      4516
                    ],
                    {
                      "Ref": [
                        [
                          3651,
                          3653
                        ],
                        "r1"
                      ]
                    },
                    [
                      [
                        {
                          "List": [
                            [
                              3673,
                              3675
                            ],
                            []
                          ]
                        },
                        {
                          "Application": [
                            [
                              3699,
                              3707
                            ],
                            {
                              "Application": [
                                [
                                  3699,
                                  3707
                                ],
                                {
                                  "Ref": [
                                    [
                                      3699,
                                      3701
                                    ],
                                    "fn"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      3702,
                                      3703
                                    ],
                                    "a"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  3704,
                                  3707
                                ],
                                "acc"
                              ]
                            }
                          ]
                        }
                      ],
                      [
                        {
                          "BinaryOp": [
                            [
                              3725,
                              3732
                            ],
                            "::",
                            {
                              "Var": [
                                [
                                  3725,
                                  3726
                                ],
                                "b"
                              ]
                            },
                            {
                              "Var": [
                                [
                                  3730,
                                  3732
                                ],
                                "r2"
                              ]
                            }
                          ]
                        },
                        {
                          "Case": [
                            [
                              3756,
                              4516
                            ],
                            {
                              "Ref": [
                                [
                                  3761,
                                  3763
                                ],
                                "r2"
                              ]
                            },
                            [
                              [
                                {
                                  "List": [
                                    [
                                      3791,
                                      3793
                                    ],
                                    []
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      3825,
                                      3840
                                    ],
                                    {
                                      "Application": [
                                        [
                                          3825,
                                          3840
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              3825,
                                              3827
                                            ],
                                            "fn"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              3828,
                                              3829
                                            ],
                                            "a"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Application": [
                                        [
                                          3831,
                                          3839
                                        ],
                                        {
                                          "Application": [
                                            [
                                              3831,
                                              3839
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  3831,
                                                  3833
                                                ],
                                                "fn"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  3834,
                                                  3835
                                                ],
                                                "b"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              3836,
                                              3839
                                            ],
                                            "acc"
                                          ]
                                        }
                                      ]
                                    }
                                  ]
                                }
                              ],
                              [
                                {
                                  "BinaryOp": [
                                    [
                                      3866,
                                      3873
                                    ],
                                    "::",
                                    {
                                      "Var": [
                                        [
                                          3866,
                                          3867
                                        ],
                                        "c"
                                      ]
                                    },
                                    {
                                      "Var": [
                                        [
                                          3871,
                                          3873
                                        ],
                                        "r3"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Case": [
                                    [
                                      3905,
                                      4516
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          3910,
                                          3912
                                        ],
                                        "r3"
                                      ]
                                    },
                                    [
                                      [
                                        {
                                          "List": [
                                            [
                                              3948,
                                              3950
                                            ],
                                            []
                                          ]
                                        },
                                        {
                                          "Application": [
                                            [
                                              3990,
                                              4012
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  3990,
                                                  4012
                                                ],
                                                {
                                                  "Ref": [
                                                    [
                                                      3990,
                                                      3992
                                                    ],
                                                    "fn"
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      3993,
                                                      3994
                                                    ],
                                                    "a"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Application": [
                                                [
                                                  3996,
                                                  4011
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      3996,
                                                      4011
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          3996,
                                                          3998
                                                        ],
                                                        "fn"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          3999,
                                                          4000
                                                        ],
                                                        "b"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Application": [
                                                    [
                                                      4002,
                                                      4010
                                                    ],
                                                    {
                                                      "Application": [
                                                        [
                                                          4002,
                                                          4010
                                                        ],
                                                        {
                                                          "Ref": [
                                                            [
                                                              4002,
                                                              4004
                                                            ],
                                                            "fn"
                                                          ]
                                                        },
                                                        {
                                                          "Ref": [
                                                            [
                                                              4005,
                                                              4006
                                                            ],
                                                            "c"
                                                          ]
                                                        }
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          4007,
                                                          4010
                                                        ],
                                                        "acc"
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
                                          "BinaryOp": [
                                            [
                                              4046,
                                              4053
                                            ],
                                            "::",
                                            {
                                              "Var": [
                                                [
                                                  4046,
                                                  4047
                                                ],
                                                "d"
                                              ]
                                            },
                                            {
                                              "Var": [
                                                [
                                                  4051,
                                                  4053
                                                ],
                                                "r4"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Let": [
                                            [
                                              4093,
                                              4516
                                            ],
                                            [
                                              {
                                                "Def": {
                                                  "header": null,
                                                  "name": "res",
                                                  "patterns": [],
                                                  "expr": {
                                                    "If": [
                                                      [
                                                        4187,
                                                        4444
                                                      ],
                                                      {
                                                        "OpChain": [
                                                          [
                                                            4190,
                                                            4199
                                                          ],
                                                          [
                                                            {
                                                              "Ref": [
                                                                [
                                                                  4190,
                                                                  4193
                                                                ],
                                                                "ctr"
                                                              ]
                                                            },
                                                            {
                                                              "Literal": [
                                                                [
                                                                  4196,
                                                                  4199
                                                                ],
                                                                {
                                                                  "Int": 500
                                                                }
                                                              ]
                                                            }
                                                          ],
                                                          [
                                                            ">"
                                                          ]
                                                        ]
                                                      },
                                                      {
                                                        "Application": [
                                                          [
                                                            4253,
                                                            4278
                                                          ],
                                                          {
                                                            "Application": [
                                                              [
                                                                4253,
                                                                4278
                                                              ],
                                                              {
                                                                "Application": [
                                                                  [
                                                                    4253,
                                                                    4278
                                                                  ],
                                                                  {
                                                                    "Ref": [
                                                                      [
                                                                        4253,
                                                                        4258
                                                                      ],
                                                                      "foldl"
                                                                    ]
                                                                  },
                                                                  {
                                                                    "Ref": [
                                                                      [
                                                                        4259,
                                                                        4261
                                                                      ],
                                                                      "fn"
                                                                    ]
                                                                  }
                                                                ]
                                                              },
                                                              {
                                                                "Ref": [
                                                                  [
                                                                    4262,
                                                                    4265
                                                                  ],
                                                                  "acc"
                                                                ]
                                                              }
                                                            ]
                                                          },
                                                          {
                                                            "Application": [
                                                              [
                                                                4267,
                                                                4277
                                                              ],
                                                              {
                                                                "Ref": [
                                                                  [
                                                                    4267,
                                                                    4274
                                                                  ],
                                                                  "reverse"
                                                                ]
                                                              },
                                                              {
                                                                "Ref": [
                                                                  [
                                                                    4275,
                                                                    4277
                                                                  ],
                                                                  "r4"
                                                                ]
                                                              }
                                                            ]
                                                          }
                                                        ]
                                                      },
                                                      {
                                                        "Application": [
                                                          [
                                                            4376,
                                                            4407
                                                          ],
                                                          {
                                                            "Application": [
                                                              [
                                                                4376,
                                                                4407
                                                              ],
                                                              {
                                                                "Application": [
                                                                  [
                                                                    4376,
                                                                    4407
                                                                  ],
                                                                  {
                                                                    "Application": [
                                                                      [
                                                                        4376,
                                                                        4407
                                                                      ],
                                                                      {
                                                                        "Ref": [
                                                                          [
                                                                            4376,
                                                                            4387
                                                                          ],
                                                                          "foldrHelper"
                                                                        ]
                                                                      },
                                                                      {
                                                                        "Ref": [
                                                                          [
                                                                            4388,
                                                                            4390
                                                                          ],
                                                                          "fn"
                                                                        ]
                                                                      }
                                                                    ]
                                                                  },
                                                                  {
                                                                    "Ref": [
                                                                      [
                                                                        4391,
                                                                        4394
                                                                      ],
                                                                      "acc"
                                                                    ]
                                                                  }
                                                                ]
                                                              },
                                                              {
                                                                "OpChain": [
                                                                  [
                                                                    4396,
                                                                    4403
                                                                  ],
                                                                  [
                                                                    {
                                                                      "Ref": [
                                                                        [
                                                                          4396,
                                                                          4399
                                                                        ],
                                                                        "ctr"
                                                                      ]
                                                                    },
                                                                    {
                                                                      "Literal": [
                                                                        [
                                                                          4402,
                                                                          4403
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
                                                                4405,
                                                                4407
                                                              ],
                                                              "r4"
                                                            ]
                                                          }
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
                                                  4487,
                                                  4516
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      4487,
                                                      4516
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          4487,
                                                          4489
                                                        ],
                                                        "fn"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          4490,
                                                          4491
                                                        ],
                                                        "a"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Application": [
                                                    [
                                                      4493,
                                                      4515
                                                    ],
                                                    {
                                                      "Application": [
                                                        [
                                                          4493,
                                                          4515
                                                        ],
                                                        {
                                                          "Ref": [
                                                            [
                                                              4493,
                                                              4495
                                                            ],
                                                            "fn"
                                                          ]
                                                        },
                                                        {
                                                          "Ref": [
                                                            [
                                                              4496,
                                                              4497
                                                            ],
                                                            "b"
                                                          ]
                                                        }
                                                      ]
                                                    },
                                                    {
                                                      "Application": [
                                                        [
                                                          4499,
                                                          4514
                                                        ],
                                                        {
                                                          "Application": [
                                                            [
                                                              4499,
                                                              4514
                                                            ],
                                                            {
                                                              "Ref": [
                                                                [
                                                                  4499,
                                                                  4501
                                                                ],
                                                                "fn"
                                                              ]
                                                            },
                                                            {
                                                              "Ref": [
                                                                [
                                                                  4502,
                                                                  4503
                                                                ],
                                                                "c"
                                                              ]
                                                            }
                                                          ]
                                                        },
                                                        {
                                                          "Application": [
                                                            [
                                                              4505,
                                                              4513
                                                            ],
                                                            {
                                                              "Application": [
                                                                [
                                                                  4505,
                                                                  4513
                                                                ],
                                                                {
                                                                  "Ref": [
                                                                    [
                                                                      4505,
                                                                      4507
                                                                    ],
                                                                    "fn"
                                                                  ]
                                                                },
                                                                {
                                                                  "Ref": [
                                                                    [
                                                                      4508,
                                                                      4509
                                                                    ],
                                                                    "d"
                                                                  ]
                                                                }
                                                              ]
                                                            },
                                                            {
                                                              "Ref": [
                                                                [
                                                                  4510,
                                                                  4513
                                                                ],
                                                                "res"
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
                                    ]
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
                  "Var": "a"
                },
                {
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
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
        "name": "filter",
        "patterns": [
          {
            "Var": [
              [
                4655,
                4661
              ],
              "isGood"
            ]
          },
          {
            "Var": [
              [
                4662,
                4666
              ],
              "list"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              4671,
              4730
            ],
            {
              "Application": [
                [
                  4671,
                  4730
                ],
                {
                  "Application": [
                    [
                      4671,
                      4730
                    ],
                    {
                      "Ref": [
                        [
                          4671,
                          4676
                        ],
                        "foldr"
                      ]
                    },
                    {
                      "Lambda": [
                        [
                          4678,
                          4721
                        ],
                        [
                          {
                            "Var": [
                              [
                                4679,
                                4680
                              ],
                              "x"
                            ]
                          },
                          {
                            "Var": [
                              [
                                4681,
                                4683
                              ],
                              "xs"
                            ]
                          }
                        ],
                        {
                          "If": [
                            [
                              4687,
                              4721
                            ],
                            {
                              "Application": [
                                [
                                  4690,
                                  4698
                                ],
                                {
                                  "Ref": [
                                    [
                                      4690,
                                      4696
                                    ],
                                    "isGood"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      4697,
                                      4698
                                    ],
                                    "x"
                                  ]
                                }
                              ]
                            },
                            {
                              "Application": [
                                [
                                  4704,
                                  4713
                                ],
                                {
                                  "Application": [
                                    [
                                      4704,
                                      4713
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          4704,
                                          4708
                                        ],
                                        "cons"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          4709,
                                          4710
                                        ],
                                        "x"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      4711,
                                      4713
                                    ],
                                    "xs"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  4719,
                                  4721
                                ],
                                "xs"
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "List": [
                    [
                      4723,
                      4726
                    ],
                    []
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  4726,
                  4730
                ],
                "list"
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
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "b"
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
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
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
        "name": "filterMap",
        "patterns": [
          {
            "Var": [
              [
                5063,
                5064
              ],
              "f"
            ]
          },
          {
            "Var": [
              [
                5065,
                5067
              ],
              "xs"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              5072,
              5097
            ],
            {
              "Application": [
                [
                  5072,
                  5097
                ],
                {
                  "Application": [
                    [
                      5072,
                      5097
                    ],
                    {
                      "Ref": [
                        [
                          5072,
                          5077
                        ],
                        "foldr"
                      ]
                    },
                    {
                      "Application": [
                        [
                          5079,
                          5090
                        ],
                        {
                          "Ref": [
                            [
                              5079,
                              5088
                            ],
                            "maybeCons"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              5089,
                              5090
                            ],
                            "f"
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "List": [
                    [
                      5092,
                      5095
                    ],
                    []
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  5095,
                  5097
                ],
                "xs"
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
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "b"
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
        "name": "maybeCons",
        "patterns": [
          {
            "Var": [
              [
                5162,
                5163
              ],
              "f"
            ]
          },
          {
            "Var": [
              [
                5164,
                5166
              ],
              "mx"
            ]
          },
          {
            "Var": [
              [
                5167,
                5169
              ],
              "xs"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              5174,
              5241
            ],
            {
              "Application": [
                [
                  5179,
                  5183
                ],
                {
                  "Ref": [
                    [
                      5179,
                      5180
                    ],
                    "f"
                  ]
                },
                {
                  "Ref": [
                    [
                      5181,
                      5183
                    ],
                    "mx"
                  ]
                }
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      5191,
                      5197
                    ],
                    "Just",
                    [
                      {
                        "Var": [
                          [
                            5196,
                            5197
                          ],
                          "x"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      5207,
                      5216
                    ],
                    {
                      "Application": [
                        [
                          5207,
                          5216
                        ],
                        {
                          "Ref": [
                            [
                              5207,
                              5211
                            ],
                            "cons"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              5212,
                              5213
                            ],
                            "x"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          5214,
                          5216
                        ],
                        "xs"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      5222,
                      5229
                    ],
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      5239,
                      5241
                    ],
                    "xs"
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
                "List",
                [
                  {
                    "Var": "a"
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
        "name": "length",
        "patterns": [
          {
            "Var": [
              [
                5353,
                5355
              ],
              "xs"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              5360,
              5386
            ],
            {
              "Application": [
                [
                  5360,
                  5386
                ],
                {
                  "Application": [
                    [
                      5360,
                      5386
                    ],
                    {
                      "Ref": [
                        [
                          5360,
                          5365
                        ],
                        "foldl"
                      ]
                    },
                    {
                      "Lambda": [
                        [
                          5367,
                          5380
                        ],
                        [
                          {
                            "Wildcard": [
                              5368,
                              5369
                            ]
                          },
                          {
                            "Var": [
                              [
                                5370,
                                5371
                              ],
                              "i"
                            ]
                          }
                        ],
                        {
                          "OpChain": [
                            [
                              5375,
                              5380
                            ],
                            [
                              {
                                "Ref": [
                                  [
                                    5375,
                                    5376
                                  ],
                                  "i"
                                ]
                              },
                              {
                                "Literal": [
                                  [
                                    5379,
                                    5380
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
                    }
                  ]
                },
                {
                  "Literal": [
                    [
                      5382,
                      5383
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
                  5384,
                  5386
                ],
                "xs"
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
                    "Var": "a"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            }
          ]
        },
        "name": "reverse",
        "patterns": [
          {
            "Var": [
              [
                5483,
                5487
              ],
              "list"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              5492,
              5510
            ],
            {
              "Application": [
                [
                  5492,
                  5510
                ],
                {
                  "Application": [
                    [
                      5492,
                      5510
                    ],
                    {
                      "Ref": [
                        [
                          5492,
                          5497
                        ],
                        "foldl"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          5498,
                          5502
                        ],
                        "cons"
                      ]
                    }
                  ]
                },
                {
                  "List": [
                    [
                      5503,
                      5506
                    ],
                    []
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  5506,
                  5510
                ],
                "list"
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
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
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
            "Var": [
              [
                5664,
                5665
              ],
              "x"
            ]
          },
          {
            "Var": [
              [
                5666,
                5668
              ],
              "xs"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              5673,
              5694
            ],
            {
              "Application": [
                [
                  5673,
                  5694
                ],
                {
                  "Ref": [
                    [
                      5673,
                      5676
                    ],
                    "any"
                  ]
                },
                {
                  "Lambda": [
                    [
                      5678,
                      5690
                    ],
                    [
                      {
                        "Var": [
                          [
                            5679,
                            5680
                          ],
                          "a"
                        ]
                      }
                    ],
                    {
                      "OpChain": [
                        [
                          5684,
                          5690
                        ],
                        [
                          {
                            "Ref": [
                              [
                                5684,
                                5685
                              ],
                              "a"
                            ]
                          },
                          {
                            "Ref": [
                              [
                                5689,
                                5690
                              ],
                              "x"
                            ]
                          }
                        ],
                        [
                          "=="
                        ]
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  5692,
                  5694
                ],
                "xs"
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
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
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
        "name": "all",
        "patterns": [
          {
            "Var": [
              [
                5875,
                5881
              ],
              "isOkay"
            ]
          },
          {
            "Var": [
              [
                5882,
                5886
              ],
              "list"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              5891,
              5921
            ],
            {
              "Ref": [
                [
                  5891,
                  5894
                ],
                "not"
              ]
            },
            {
              "Application": [
                [
                  5896,
                  5920
                ],
                {
                  "Application": [
                    [
                      5896,
                      5920
                    ],
                    {
                      "Ref": [
                        [
                          5896,
                          5899
                        ],
                        "any"
                      ]
                    },
                    {
                      "OpChain": [
                        [
                          5901,
                          5914
                        ],
                        [
                          {
                            "Ref": [
                              [
                                5901,
                                5904
                              ],
                              "not"
                            ]
                          },
                          {
                            "Ref": [
                              [
                                5908,
                                5914
                              ],
                              "isOkay"
                            ]
                          }
                        ],
                        [
                          "<<"
                        ]
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      5916,
                      5920
                    ],
                    "list"
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
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
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
        "name": "any",
        "patterns": [
          {
            "Var": [
              [
                6103,
                6109
              ],
              "isOkay"
            ]
          },
          {
            "Var": [
              [
                6110,
                6114
              ],
              "list"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              6119,
              6300
            ],
            {
              "Ref": [
                [
                  6124,
                  6128
                ],
                "list"
              ]
            },
            [
              [
                {
                  "List": [
                    [
                      6136,
                      6138
                    ],
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      6148,
                      6153
                    ],
                    "False"
                  ]
                }
              ],
              [
                {
                  "BinaryOp": [
                    [
                      6159,
                      6166
                    ],
                    "::",
                    {
                      "Var": [
                        [
                          6159,
                          6160
                        ],
                        "x"
                      ]
                    },
                    {
                      "Var": [
                        [
                          6164,
                          6166
                        ],
                        "xs"
                      ]
                    }
                  ]
                },
                {
                  "If": [
                    [
                      6237,
                      6300
                    ],
                    {
                      "Application": [
                        [
                          6240,
                          6248
                        ],
                        {
                          "Ref": [
                            [
                              6240,
                              6246
                            ],
                            "isOkay"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              6247,
                              6248
                            ],
                            "x"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          6262,
                          6274
                        ],
                        "True"
                      ]
                    },
                    {
                      "Application": [
                        [
                          6287,
                          6300
                        ],
                        {
                          "Application": [
                            [
                              6287,
                              6300
                            ],
                            {
                              "Ref": [
                                [
                                  6287,
                                  6290
                                ],
                                "any"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  6291,
                                  6297
                                ],
                                "isOkay"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              6298,
                              6300
                            ],
                            "xs"
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
              "Tag": [
                "List",
                [
                  {
                    "Var": "comparable"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Maybe",
                [
                  {
                    "Var": "comparable"
                  }
                ]
              ]
            }
          ]
        },
        "name": "maximum",
        "patterns": [
          {
            "Var": [
              [
                6472,
                6476
              ],
              "list"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              6481,
              6560
            ],
            {
              "Ref": [
                [
                  6486,
                  6490
                ],
                "list"
              ]
            },
            [
              [
                {
                  "BinaryOp": [
                    [
                      6498,
                      6505
                    ],
                    "::",
                    {
                      "Var": [
                        [
                          6498,
                          6499
                        ],
                        "x"
                      ]
                    },
                    {
                      "Var": [
                        [
                          6503,
                          6505
                        ],
                        "xs"
                      ]
                    }
                  ]
                },
                {
                  "Application": [
                    [
                      6515,
                      6536
                    ],
                    {
                      "Ref": [
                        [
                          6515,
                          6520
                        ],
                        "Just"
                      ]
                    },
                    {
                      "Application": [
                        [
                          6521,
                          6535
                        ],
                        {
                          "Application": [
                            [
                              6521,
                              6535
                            ],
                            {
                              "Application": [
                                [
                                  6521,
                                  6535
                                ],
                                {
                                  "Ref": [
                                    [
                                      6521,
                                      6526
                                    ],
                                    "foldl"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      6527,
                                      6530
                                    ],
                                    "max"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  6531,
                                  6532
                                ],
                                "x"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              6533,
                              6535
                            ],
                            "xs"
                          ]
                        }
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Wildcard": [
                    6542,
                    6543
                  ]
                },
                {
                  "Ref": [
                    [
                      6553,
                      6560
                    ],
                    "Nothing"
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
                "List",
                [
                  {
                    "Var": "comparable"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Maybe",
                [
                  {
                    "Var": "comparable"
                  }
                ]
              ]
            }
          ]
        },
        "name": "minimum",
        "patterns": [
          {
            "Var": [
              [
                6732,
                6736
              ],
              "list"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              6741,
              6820
            ],
            {
              "Ref": [
                [
                  6746,
                  6750
                ],
                "list"
              ]
            },
            [
              [
                {
                  "BinaryOp": [
                    [
                      6758,
                      6765
                    ],
                    "::",
                    {
                      "Var": [
                        [
                          6758,
                          6759
                        ],
                        "x"
                      ]
                    },
                    {
                      "Var": [
                        [
                          6763,
                          6765
                        ],
                        "xs"
                      ]
                    }
                  ]
                },
                {
                  "Application": [
                    [
                      6775,
                      6796
                    ],
                    {
                      "Ref": [
                        [
                          6775,
                          6780
                        ],
                        "Just"
                      ]
                    },
                    {
                      "Application": [
                        [
                          6781,
                          6795
                        ],
                        {
                          "Application": [
                            [
                              6781,
                              6795
                            ],
                            {
                              "Application": [
                                [
                                  6781,
                                  6795
                                ],
                                {
                                  "Ref": [
                                    [
                                      6781,
                                      6786
                                    ],
                                    "foldl"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      6787,
                                      6790
                                    ],
                                    "min"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  6791,
                                  6792
                                ],
                                "x"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              6793,
                              6795
                            ],
                            "xs"
                          ]
                        }
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Wildcard": [
                    6802,
                    6803
                  ]
                },
                {
                  "Ref": [
                    [
                      6813,
                      6820
                    ],
                    "Nothing"
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
                "List",
                [
                  {
                    "Var": "number"
                  }
                ]
              ]
            },
            {
              "Var": "number"
            }
          ]
        },
        "name": "sum",
        "patterns": [
          {
            "Var": [
              [
                6921,
                6928
              ],
              "numbers"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              6933,
              6952
            ],
            {
              "Application": [
                [
                  6933,
                  6952
                ],
                {
                  "Application": [
                    [
                      6933,
                      6952
                    ],
                    {
                      "Ref": [
                        [
                          6933,
                          6938
                        ],
                        "foldl"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          6939,
                          6943
                        ],
                        "+"
                      ]
                    }
                  ]
                },
                {
                  "Literal": [
                    [
                      6943,
                      6944
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
                  6945,
                  6952
                ],
                "numbers"
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
                    "Var": "number"
                  }
                ]
              ]
            },
            {
              "Var": "number"
            }
          ]
        },
        "name": "product",
        "patterns": [
          {
            "Var": [
              [
                7069,
                7076
              ],
              "numbers"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              7081,
              7100
            ],
            {
              "Application": [
                [
                  7081,
                  7100
                ],
                {
                  "Application": [
                    [
                      7081,
                      7100
                    ],
                    {
                      "Ref": [
                        [
                          7081,
                          7086
                        ],
                        "foldl"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          7087,
                          7091
                        ],
                        "*"
                      ]
                    }
                  ]
                },
                {
                  "Literal": [
                    [
                      7091,
                      7092
                    ],
                    {
                      "Int": 1
                    }
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  7093,
                  7100
                ],
                "numbers"
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
                    "Var": "a"
                  }
                ]
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
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
        "name": "append",
        "patterns": [
          {
            "Var": [
              [
                7348,
                7350
              ],
              "xs"
            ]
          },
          {
            "Var": [
              [
                7351,
                7353
              ],
              "ys"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              7358,
              7420
            ],
            {
              "Ref": [
                [
                  7363,
                  7365
                ],
                "ys"
              ]
            },
            [
              [
                {
                  "List": [
                    [
                      7373,
                      7375
                    ],
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      7385,
                      7387
                    ],
                    "xs"
                  ]
                }
              ],
              [
                {
                  "Wildcard": [
                    7393,
                    7394
                  ]
                },
                {
                  "Application": [
                    [
                      7404,
                      7420
                    ],
                    {
                      "Application": [
                        [
                          7404,
                          7420
                        ],
                        {
                          "Application": [
                            [
                              7404,
                              7420
                            ],
                            {
                              "Ref": [
                                [
                                  7404,
                                  7409
                                ],
                                "foldr"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  7410,
                                  7414
                                ],
                                "cons"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              7415,
                              7417
                            ],
                            "ys"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          7418,
                          7420
                        ],
                        "xs"
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
                "List",
                [
                  {
                    "Tag": [
                      "List",
                      [
                        {
                          "Var": "a"
                        }
                      ]
                    ]
                  }
                ]
              ]
            },
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            }
          ]
        },
        "name": "concat",
        "patterns": [
          {
            "Var": [
              [
                7564,
                7569
              ],
              "lists"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              7574,
              7595
            ],
            {
              "Application": [
                [
                  7574,
                  7595
                ],
                {
                  "Application": [
                    [
                      7574,
                      7595
                    ],
                    {
                      "Ref": [
                        [
                          7574,
                          7579
                        ],
                        "foldr"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          7580,
                          7586
                        ],
                        "append"
                      ]
                    }
                  ]
                },
                {
                  "List": [
                    [
                      7587,
                      7590
                    ],
                    []
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  7590,
                  7595
                ],
                "lists"
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
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "b"
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
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
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
        "name": "concatMap",
        "patterns": [
          {
            "Var": [
              [
                7768,
                7769
              ],
              "f"
            ]
          },
          {
            "Var": [
              [
                7770,
                7774
              ],
              "list"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              7779,
              7798
            ],
            {
              "Ref": [
                [
                  7779,
                  7785
                ],
                "concat"
              ]
            },
            {
              "Application": [
                [
                  7787,
                  7797
                ],
                {
                  "Application": [
                    [
                      7787,
                      7797
                    ],
                    {
                      "Ref": [
                        [
                          7787,
                          7790
                        ],
                        "map"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          7791,
                          7792
                        ],
                        "f"
                      ]
                    }
                  ]
                },
                {
                  "Ref": [
                    [
                      7793,
                      7797
                    ],
                    "list"
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
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
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
        "name": "intersperse",
        "patterns": [
          {
            "Var": [
              [
                8017,
                8020
              ],
              "sep"
            ]
          },
          {
            "Var": [
              [
                8021,
                8023
              ],
              "xs"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              8028,
              8218
            ],
            {
              "Ref": [
                [
                  8033,
                  8035
                ],
                "xs"
              ]
            },
            [
              [
                {
                  "List": [
                    [
                      8043,
                      8045
                    ],
                    []
                  ]
                },
                {
                  "List": [
                    [
                      8055,
                      8057
                    ],
                    []
                  ]
                }
              ],
              [
                {
                  "BinaryOp": [
                    [
                      8063,
                      8071
                    ],
                    "::",
                    {
                      "Var": [
                        [
                          8063,
                          8065
                        ],
                        "hd"
                      ]
                    },
                    {
                      "Var": [
                        [
                          8069,
                          8071
                        ],
                        "tl"
                      ]
                    }
                  ]
                },
                {
                  "Let": [
                    [
                      8081,
                      8218
                    ],
                    [
                      {
                        "Def": {
                          "header": null,
                          "name": "step",
                          "patterns": [
                            {
                              "Var": [
                                [
                                  8098,
                                  8099
                                ],
                                "x"
                              ]
                            },
                            {
                              "Var": [
                                [
                                  8100,
                                  8104
                                ],
                                "rest"
                              ]
                            }
                          ],
                          "expr": {
                            "Application": [
                              [
                                8117,
                                8139
                              ],
                              {
                                "Application": [
                                  [
                                    8117,
                                    8139
                                  ],
                                  {
                                    "Ref": [
                                      [
                                        8117,
                                        8121
                                      ],
                                      "cons"
                                    ]
                                  },
                                  {
                                    "Ref": [
                                      [
                                        8122,
                                        8125
                                      ],
                                      "sep"
                                    ]
                                  }
                                ]
                              },
                              {
                                "Application": [
                                  [
                                    8127,
                                    8138
                                  ],
                                  {
                                    "Application": [
                                      [
                                        8127,
                                        8138
                                      ],
                                      {
                                        "Ref": [
                                          [
                                            8127,
                                            8131
                                          ],
                                          "cons"
                                        ]
                                      },
                                      {
                                        "Ref": [
                                          [
                                            8132,
                                            8133
                                          ],
                                          "x"
                                        ]
                                      }
                                    ]
                                  },
                                  {
                                    "Ref": [
                                      [
                                        8134,
                                        8138
                                      ],
                                      "rest"
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
                          "header": null,
                          "name": "spersed",
                          "patterns": [],
                          "expr": {
                            "Application": [
                              [
                                8169,
                                8185
                              ],
                              {
                                "Application": [
                                  [
                                    8169,
                                    8185
                                  ],
                                  {
                                    "Application": [
                                      [
                                        8169,
                                        8185
                                      ],
                                      {
                                        "Ref": [
                                          [
                                            8169,
                                            8174
                                          ],
                                          "foldr"
                                        ]
                                      },
                                      {
                                        "Ref": [
                                          [
                                            8175,
                                            8179
                                          ],
                                          "step"
                                        ]
                                      }
                                    ]
                                  },
                                  {
                                    "List": [
                                      [
                                        8180,
                                        8183
                                      ],
                                      []
                                    ]
                                  }
                                ]
                              },
                              {
                                "Ref": [
                                  [
                                    8183,
                                    8185
                                  ],
                                  "tl"
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
                          8203,
                          8218
                        ],
                        {
                          "Application": [
                            [
                              8203,
                              8218
                            ],
                            {
                              "Ref": [
                                [
                                  8203,
                                  8207
                                ],
                                "cons"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  8208,
                                  8210
                                ],
                                "hd"
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              8211,
                              8218
                            ],
                            "spersed"
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
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Var": "result"
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
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
                        "List",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "result"
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
        "name": "map2",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              8738,
              8758
            ],
            [
              "Elm",
              "Kernel",
              "List"
            ],
            "map2"
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
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "c"
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
                    "List",
                    [
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
                        "List",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "List",
                            [
                              {
                                "Var": "c"
                              }
                            ]
                          ]
                        },
                        {
                          "Tag": [
                            "List",
                            [
                              {
                                "Var": "result"
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
        },
        "name": "map3",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              8852,
              8872
            ],
            [
              "Elm",
              "Kernel",
              "List"
            ],
            "map3"
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
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "c"
                        },
                        {
                          "Fun": [
                            {
                              "Var": "d"
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
                  "Tag": [
                    "List",
                    [
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
                        "List",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "List",
                            [
                              {
                                "Var": "c"
                              }
                            ]
                          ]
                        },
                        {
                          "Fun": [
                            {
                              "Tag": [
                                "List",
                                [
                                  {
                                    "Var": "d"
                                  }
                                ]
                              ]
                            },
                            {
                              "Tag": [
                                "List",
                                [
                                  {
                                    "Var": "result"
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
        "name": "map4",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              8981,
              9001
            ],
            [
              "Elm",
              "Kernel",
              "List"
            ],
            "map4"
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
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "c"
                        },
                        {
                          "Fun": [
                            {
                              "Var": "d"
                            },
                            {
                              "Fun": [
                                {
                                  "Var": "e"
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
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
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
                        "List",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "List",
                            [
                              {
                                "Var": "c"
                              }
                            ]
                          ]
                        },
                        {
                          "Fun": [
                            {
                              "Tag": [
                                "List",
                                [
                                  {
                                    "Var": "d"
                                  }
                                ]
                              ]
                            },
                            {
                              "Fun": [
                                {
                                  "Tag": [
                                    "List",
                                    [
                                      {
                                        "Var": "e"
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Tag": [
                                    "List",
                                    [
                                      {
                                        "Var": "result"
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
        },
        "name": "map5",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              9125,
              9145
            ],
            [
              "Elm",
              "Kernel",
              "List"
            ],
            "map5"
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
                    "Var": "comparable"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "List",
                [
                  {
                    "Var": "comparable"
                  }
                ]
              ]
            }
          ]
        },
        "name": "sort",
        "patterns": [
          {
            "Var": [
              [
                9277,
                9279
              ],
              "xs"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              9284,
              9302
            ],
            {
              "Application": [
                [
                  9284,
                  9302
                ],
                {
                  "Ref": [
                    [
                      9284,
                      9290
                    ],
                    "sortBy"
                  ]
                },
                {
                  "Ref": [
                    [
                      9291,
                      9299
                    ],
                    "identity"
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  9300,
                  9302
                ],
                "xs"
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
                  "Var": "comparable"
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
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
        "name": "sortBy",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              9711,
              9733
            ],
            [
              "Elm",
              "Kernel",
              "List"
            ],
            "sortBy"
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
                  "Fun": [
                    {
                      "Var": "a"
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
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
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
        "name": "sortWith",
        "patterns": [],
        "expr": {
          "QualifiedRef": [
            [
              10133,
              10157
            ],
            [
              "Elm",
              "Kernel",
              "List"
            ],
            "sortWith"
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
                    "Var": "a"
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
            "Var": [
              [
                10398,
                10400
              ],
              "xs"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              10405,
              10458
            ],
            {
              "Ref": [
                [
                  10410,
                  10412
                ],
                "xs"
              ]
            },
            [
              [
                {
                  "List": [
                    [
                      10420,
                      10422
                    ],
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      10432,
                      10436
                    ],
                    "True"
                  ]
                }
              ],
              [
                {
                  "Wildcard": [
                    10442,
                    10443
                  ]
                },
                {
                  "Ref": [
                    [
                      10453,
                      10458
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
                "List",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Maybe",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            }
          ]
        },
        "name": "head",
        "patterns": [
          {
            "Var": [
              [
                10732,
                10736
              ],
              "list"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              10741,
              10806
            ],
            {
              "Ref": [
                [
                  10746,
                  10750
                ],
                "list"
              ]
            },
            [
              [
                {
                  "BinaryOp": [
                    [
                      10758,
                      10765
                    ],
                    "::",
                    {
                      "Var": [
                        [
                          10758,
                          10759
                        ],
                        "x"
                      ]
                    },
                    {
                      "Var": [
                        [
                          10763,
                          10765
                        ],
                        "xs"
                      ]
                    }
                  ]
                },
                {
                  "Application": [
                    [
                      10775,
                      10781
                    ],
                    {
                      "Ref": [
                        [
                          10775,
                          10780
                        ],
                        "Just"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          10780,
                          10781
                        ],
                        "x"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "List": [
                    [
                      10787,
                      10789
                    ],
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      10799,
                      10806
                    ],
                    "Nothing"
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
                "List",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Maybe",
                [
                  {
                    "Tag": [
                      "List",
                      [
                        {
                          "Var": "a"
                        }
                      ]
                    ]
                  }
                ]
              ]
            }
          ]
        },
        "name": "tail",
        "patterns": [
          {
            "Var": [
              [
                11084,
                11088
              ],
              "list"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              11093,
              11159
            ],
            {
              "Ref": [
                [
                  11098,
                  11102
                ],
                "list"
              ]
            },
            [
              [
                {
                  "BinaryOp": [
                    [
                      11110,
                      11117
                    ],
                    "::",
                    {
                      "Var": [
                        [
                          11110,
                          11111
                        ],
                        "x"
                      ]
                    },
                    {
                      "Var": [
                        [
                          11115,
                          11117
                        ],
                        "xs"
                      ]
                    }
                  ]
                },
                {
                  "Application": [
                    [
                      11127,
                      11134
                    ],
                    {
                      "Ref": [
                        [
                          11127,
                          11132
                        ],
                        "Just"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          11132,
                          11134
                        ],
                        "xs"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "List": [
                    [
                      11140,
                      11142
                    ],
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      11152,
                      11159
                    ],
                    "Nothing"
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
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
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
        "name": "take",
        "patterns": [
          {
            "Var": [
              [
                11274,
                11275
              ],
              "n"
            ]
          },
          {
            "Var": [
              [
                11276,
                11280
              ],
              "list"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              11285,
              11302
            ],
            {
              "Application": [
                [
                  11285,
                  11302
                ],
                {
                  "Application": [
                    [
                      11285,
                      11302
                    ],
                    {
                      "Ref": [
                        [
                          11285,
                          11293
                        ],
                        "takeFast"
                      ]
                    },
                    {
                      "Literal": [
                        [
                          11294,
                          11295
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
                      11296,
                      11297
                    ],
                    "n"
                  ]
                }
              ]
            },
            {
              "Ref": [
                [
                  11298,
                  11302
                ],
                "list"
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
                    "Int",
                    []
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "a"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "a"
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
        "name": "takeFast",
        "patterns": [
          {
            "Var": [
              [
                11356,
                11359
              ],
              "ctr"
            ]
          },
          {
            "Var": [
              [
                11360,
                11361
              ],
              "n"
            ]
          },
          {
            "Var": [
              [
                11362,
                11366
              ],
              "list"
            ]
          }
        ],
        "expr": {
          "If": [
            [
              11371,
              11843
            ],
            {
              "OpChain": [
                [
                  11374,
                  11380
                ],
                [
                  {
                    "Ref": [
                      [
                        11374,
                        11375
                      ],
                      "n"
                    ]
                  },
                  {
                    "Literal": [
                      [
                        11379,
                        11380
                      ],
                      {
                        "Int": 0
                      }
                    ]
                  }
                ],
                [
                  "<="
                ]
              ]
            },
            {
              "List": [
                [
                  11390,
                  11395
                ],
                []
              ]
            },
            {
              "Case": [
                [
                  11404,
                  11843
                ],
                {
                  "Tuple": [
                    [
                      11409,
                      11421
                    ],
                    [
                      {
                        "Ref": [
                          [
                            11411,
                            11412
                          ],
                          "n"
                        ]
                      },
                      {
                        "Ref": [
                          [
                            11414,
                            11418
                          ],
                          "list"
                        ]
                      }
                    ]
                  ]
                },
                [
                  [
                    {
                      "Tuple": [
                        [
                          11430,
                          11439
                        ],
                        [
                          {
                            "Wildcard": [
                              11432,
                              11433
                            ]
                          },
                          {
                            "List": [
                              [
                                11435,
                                11437
                              ],
                              []
                            ]
                          }
                        ]
                      ]
                    },
                    {
                      "Ref": [
                        [
                          11451,
                          11455
                        ],
                        "list"
                      ]
                    }
                  ],
                  [
                    {
                      "Tuple": [
                        [
                          11463,
                          11476
                        ],
                        [
                          {
                            "LitInt": [
                              [
                                11465,
                                11466
                              ],
                              1
                            ]
                          },
                          {
                            "BinaryOp": [
                              [
                                11468,
                                11474
                              ],
                              "::",
                              {
                                "Var": [
                                  [
                                    11468,
                                    11469
                                  ],
                                  "x"
                                ]
                              },
                              {
                                "Wildcard": [
                                  11473,
                                  11474
                                ]
                              }
                            ]
                          }
                        ]
                      ]
                    },
                    {
                      "List": [
                        [
                          11488,
                          11493
                        ],
                        [
                          {
                            "Ref": [
                              [
                                11490,
                                11491
                              ],
                              "x"
                            ]
                          }
                        ]
                      ]
                    }
                  ],
                  [
                    {
                      "Tuple": [
                        [
                          11501,
                          11519
                        ],
                        [
                          {
                            "LitInt": [
                              [
                                11503,
                                11504
                              ],
                              2
                            ]
                          },
                          {
                            "BinaryOp": [
                              [
                                11506,
                                11517
                              ],
                              "::",
                              {
                                "Var": [
                                  [
                                    11506,
                                    11507
                                  ],
                                  "x"
                                ]
                              },
                              {
                                "BinaryOp": [
                                  [
                                    11511,
                                    11517
                                  ],
                                  "::",
                                  {
                                    "Var": [
                                      [
                                        11511,
                                        11512
                                      ],
                                      "y"
                                    ]
                                  },
                                  {
                                    "Wildcard": [
                                      11516,
                                      11517
                                    ]
                                  }
                                ]
                              }
                            ]
                          }
                        ]
                      ]
                    },
                    {
                      "List": [
                        [
                          11531,
                          11539
                        ],
                        [
                          {
                            "Ref": [
                              [
                                11533,
                                11534
                              ],
                              "x"
                            ]
                          },
                          {
                            "Ref": [
                              [
                                11536,
                                11537
                              ],
                              "y"
                            ]
                          }
                        ]
                      ]
                    }
                  ],
                  [
                    {
                      "Tuple": [
                        [
                          11547,
                          11570
                        ],
                        [
                          {
                            "LitInt": [
                              [
                                11549,
                                11550
                              ],
                              3
                            ]
                          },
                          {
                            "BinaryOp": [
                              [
                                11552,
                                11568
                              ],
                              "::",
                              {
                                "Var": [
                                  [
                                    11552,
                                    11553
                                  ],
                                  "x"
                                ]
                              },
                              {
                                "BinaryOp": [
                                  [
                                    11557,
                                    11568
                                  ],
                                  "::",
                                  {
                                    "Var": [
                                      [
                                        11557,
                                        11558
                                      ],
                                      "y"
                                    ]
                                  },
                                  {
                                    "BinaryOp": [
                                      [
                                        11562,
                                        11568
                                      ],
                                      "::",
                                      {
                                        "Var": [
                                          [
                                            11562,
                                            11563
                                          ],
                                          "z"
                                        ]
                                      },
                                      {
                                        "Wildcard": [
                                          11567,
                                          11568
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
                    },
                    {
                      "List": [
                        [
                          11582,
                          11593
                        ],
                        [
                          {
                            "Ref": [
                              [
                                11584,
                                11585
                              ],
                              "x"
                            ]
                          },
                          {
                            "Ref": [
                              [
                                11587,
                                11588
                              ],
                              "y"
                            ]
                          },
                          {
                            "Ref": [
                              [
                                11590,
                                11591
                              ],
                              "z"
                            ]
                          }
                        ]
                      ]
                    }
                  ],
                  [
                    {
                      "Tuple": [
                        [
                          11601,
                          11630
                        ],
                        [
                          {
                            "Wildcard": [
                              11603,
                              11604
                            ]
                          },
                          {
                            "BinaryOp": [
                              [
                                11606,
                                11628
                              ],
                              "::",
                              {
                                "Var": [
                                  [
                                    11606,
                                    11607
                                  ],
                                  "x"
                                ]
                              },
                              {
                                "BinaryOp": [
                                  [
                                    11611,
                                    11628
                                  ],
                                  "::",
                                  {
                                    "Var": [
                                      [
                                        11611,
                                        11612
                                      ],
                                      "y"
                                    ]
                                  },
                                  {
                                    "BinaryOp": [
                                      [
                                        11616,
                                        11628
                                      ],
                                      "::",
                                      {
                                        "Var": [
                                          [
                                            11616,
                                            11617
                                          ],
                                          "z"
                                        ]
                                      },
                                      {
                                        "BinaryOp": [
                                          [
                                            11621,
                                            11628
                                          ],
                                          "::",
                                          {
                                            "Var": [
                                              [
                                                11621,
                                                11622
                                              ],
                                              "w"
                                            ]
                                          },
                                          {
                                            "Var": [
                                              [
                                                11626,
                                                11628
                                              ],
                                              "tl"
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
                      ]
                    },
                    {
                      "If": [
                        [
                          11642,
                          11818
                        ],
                        {
                          "OpChain": [
                            [
                              11645,
                              11655
                            ],
                            [
                              {
                                "Ref": [
                                  [
                                    11645,
                                    11648
                                  ],
                                  "ctr"
                                ]
                              },
                              {
                                "Literal": [
                                  [
                                    11651,
                                    11655
                                  ],
                                  {
                                    "Int": 1000
                                  }
                                ]
                              }
                            ],
                            [
                              ">"
                            ]
                          ]
                        },
                        {
                          "Application": [
                            [
                              11671,
                              11729
                            ],
                            {
                              "Application": [
                                [
                                  11671,
                                  11729
                                ],
                                {
                                  "Ref": [
                                    [
                                      11671,
                                      11675
                                    ],
                                    "cons"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      11676,
                                      11677
                                    ],
                                    "x"
                                  ]
                                }
                              ]
                            },
                            {
                              "Application": [
                                [
                                  11679,
                                  11728
                                ],
                                {
                                  "Application": [
                                    [
                                      11679,
                                      11728
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          11679,
                                          11683
                                        ],
                                        "cons"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          11684,
                                          11685
                                        ],
                                        "y"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      11687,
                                      11727
                                    ],
                                    {
                                      "Application": [
                                        [
                                          11687,
                                          11727
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              11687,
                                              11691
                                            ],
                                            "cons"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              11692,
                                              11693
                                            ],
                                            "z"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Application": [
                                        [
                                          11695,
                                          11726
                                        ],
                                        {
                                          "Application": [
                                            [
                                              11695,
                                              11726
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  11695,
                                                  11699
                                                ],
                                                "cons"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  11700,
                                                  11701
                                                ],
                                                "w"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Application": [
                                            [
                                              11703,
                                              11725
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  11703,
                                                  11725
                                                ],
                                                {
                                                  "Ref": [
                                                    [
                                                      11703,
                                                      11714
                                                    ],
                                                    "takeTailRec"
                                                  ]
                                                },
                                                {
                                                  "OpChain": [
                                                    [
                                                      11716,
                                                      11721
                                                    ],
                                                    [
                                                      {
                                                        "Ref": [
                                                          [
                                                            11716,
                                                            11717
                                                          ],
                                                          "n"
                                                        ]
                                                      },
                                                      {
                                                        "Literal": [
                                                          [
                                                            11720,
                                                            11721
                                                          ],
                                                          {
                                                            "Int": 4
                                                          }
                                                        ]
                                                      }
                                                    ],
                                                    [
                                                      "-"
                                                    ]
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  11723,
                                                  11725
                                                ],
                                                "tl"
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
                        {
                          "Application": [
                            [
                              11753,
                              11818
                            ],
                            {
                              "Application": [
                                [
                                  11753,
                                  11818
                                ],
                                {
                                  "Ref": [
                                    [
                                      11753,
                                      11757
                                    ],
                                    "cons"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      11758,
                                      11759
                                    ],
                                    "x"
                                  ]
                                }
                              ]
                            },
                            {
                              "Application": [
                                [
                                  11761,
                                  11817
                                ],
                                {
                                  "Application": [
                                    [
                                      11761,
                                      11817
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          11761,
                                          11765
                                        ],
                                        "cons"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          11766,
                                          11767
                                        ],
                                        "y"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      11769,
                                      11816
                                    ],
                                    {
                                      "Application": [
                                        [
                                          11769,
                                          11816
                                        ],
                                        {
                                          "Ref": [
                                            [
                                              11769,
                                              11773
                                            ],
                                            "cons"
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              11774,
                                              11775
                                            ],
                                            "z"
                                          ]
                                        }
                                      ]
                                    },
                                    {
                                      "Application": [
                                        [
                                          11777,
                                          11815
                                        ],
                                        {
                                          "Application": [
                                            [
                                              11777,
                                              11815
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  11777,
                                                  11781
                                                ],
                                                "cons"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  11782,
                                                  11783
                                                ],
                                                "w"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Application": [
                                            [
                                              11785,
                                              11814
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  11785,
                                                  11814
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      11785,
                                                      11814
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          11785,
                                                          11793
                                                        ],
                                                        "takeFast"
                                                      ]
                                                    },
                                                    {
                                                      "OpChain": [
                                                        [
                                                          11795,
                                                          11802
                                                        ],
                                                        [
                                                          {
                                                            "Ref": [
                                                              [
                                                                11795,
                                                                11798
                                                              ],
                                                              "ctr"
                                                            ]
                                                          },
                                                          {
                                                            "Literal": [
                                                              [
                                                                11801,
                                                                11802
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
                                                  "OpChain": [
                                                    [
                                                      11805,
                                                      11810
                                                    ],
                                                    [
                                                      {
                                                        "Ref": [
                                                          [
                                                            11805,
                                                            11806
                                                          ],
                                                          "n"
                                                        ]
                                                      },
                                                      {
                                                        "Literal": [
                                                          [
                                                            11809,
                                                            11810
                                                          ],
                                                          {
                                                            "Int": 4
                                                          }
                                                        ]
                                                      }
                                                    ],
                                                    [
                                                      "-"
                                                    ]
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  11812,
                                                  11814
                                                ],
                                                "tl"
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
                  ],
                  [
                    {
                      "Wildcard": [
                        11826,
                        11827
                      ]
                    },
                    {
                      "Ref": [
                        [
                          11839,
                          11843
                        ],
                        "list"
                      ]
                    }
                  ]
                ]
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
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
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
        "name": "takeTailRec",
        "patterns": [
          {
            "Var": [
              [
                11895,
                11896
              ],
              "n"
            ]
          },
          {
            "Var": [
              [
                11897,
                11901
              ],
              "list"
            ]
          }
        ],
        "expr": {
          "Application": [
            [
              11906,
              11937
            ],
            {
              "Ref": [
                [
                  11906,
                  11913
                ],
                "reverse"
              ]
            },
            {
              "Application": [
                [
                  11915,
                  11936
                ],
                {
                  "Application": [
                    [
                      11915,
                      11936
                    ],
                    {
                      "Application": [
                        [
                          11915,
                          11936
                        ],
                        {
                          "Ref": [
                            [
                              11915,
                              11926
                            ],
                            "takeReverse"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              11927,
                              11928
                            ],
                            "n"
                          ]
                        }
                      ]
                    },
                    {
                      "Ref": [
                        [
                          11929,
                          11933
                        ],
                        "list"
                      ]
                    }
                  ]
                },
                {
                  "List": [
                    [
                      11934,
                      11936
                    ],
                    []
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
                "Int",
                []
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
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
                        "List",
                        [
                          {
                            "Var": "a"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "a"
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
        "name": "takeReverse",
        "patterns": [
          {
            "Var": [
              [
                12000,
                12001
              ],
              "n"
            ]
          },
          {
            "Var": [
              [
                12002,
                12006
              ],
              "list"
            ]
          },
          {
            "Var": [
              [
                12007,
                12011
              ],
              "kept"
            ]
          }
        ],
        "expr": {
          "If": [
            [
              12016,
              12151
            ],
            {
              "OpChain": [
                [
                  12019,
                  12025
                ],
                [
                  {
                    "Ref": [
                      [
                        12019,
                        12020
                      ],
                      "n"
                    ]
                  },
                  {
                    "Literal": [
                      [
                        12024,
                        12025
                      ],
                      {
                        "Int": 0
                      }
                    ]
                  }
                ],
                [
                  "<="
                ]
              ]
            },
            {
              "Ref": [
                [
                  12035,
                  12039
                ],
                "kept"
              ]
            },
            {
              "Case": [
                [
                  12051,
                  12151
                ],
                {
                  "Ref": [
                    [
                      12056,
                      12060
                    ],
                    "list"
                  ]
                },
                [
                  [
                    {
                      "List": [
                        [
                          12070,
                          12072
                        ],
                        []
                      ]
                    },
                    {
                      "Ref": [
                        [
                          12084,
                          12088
                        ],
                        "kept"
                      ]
                    }
                  ],
                  [
                    {
                      "BinaryOp": [
                        [
                          12096,
                          12103
                        ],
                        "::",
                        {
                          "Var": [
                            [
                              12096,
                              12097
                            ],
                            "x"
                          ]
                        },
                        {
                          "Var": [
                            [
                              12101,
                              12103
                            ],
                            "xs"
                          ]
                        }
                      ]
                    },
                    {
                      "Application": [
                        [
                          12115,
                          12151
                        ],
                        {
                          "Application": [
                            [
                              12115,
                              12151
                            ],
                            {
                              "Application": [
                                [
                                  12115,
                                  12151
                                ],
                                {
                                  "Ref": [
                                    [
                                      12115,
                                      12126
                                    ],
                                    "takeReverse"
                                  ]
                                },
                                {
                                  "OpChain": [
                                    [
                                      12128,
                                      12133
                                    ],
                                    [
                                      {
                                        "Ref": [
                                          [
                                            12128,
                                            12129
                                          ],
                                          "n"
                                        ]
                                      },
                                      {
                                        "Literal": [
                                          [
                                            12132,
                                            12133
                                          ],
                                          {
                                            "Int": 1
                                          }
                                        ]
                                      }
                                    ],
                                    [
                                      "-"
                                    ]
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  12135,
                                  12137
                                ],
                                "xs"
                              ]
                            }
                          ]
                        },
                        {
                          "Application": [
                            [
                              12139,
                              12150
                            ],
                            {
                              "Application": [
                                [
                                  12139,
                                  12150
                                ],
                                {
                                  "Ref": [
                                    [
                                      12139,
                                      12143
                                    ],
                                    "cons"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      12144,
                                      12145
                                    ],
                                    "x"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  12146,
                                  12150
                                ],
                                "kept"
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
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
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
        "name": "drop",
        "patterns": [
          {
            "Var": [
              [
                12266,
                12267
              ],
              "n"
            ]
          },
          {
            "Var": [
              [
                12268,
                12272
              ],
              "list"
            ]
          }
        ],
        "expr": {
          "If": [
            [
              12277,
              12390
            ],
            {
              "OpChain": [
                [
                  12280,
                  12286
                ],
                [
                  {
                    "Ref": [
                      [
                        12280,
                        12281
                      ],
                      "n"
                    ]
                  },
                  {
                    "Literal": [
                      [
                        12285,
                        12286
                      ],
                      {
                        "Int": 0
                      }
                    ]
                  }
                ],
                [
                  "<="
                ]
              ]
            },
            {
              "Ref": [
                [
                  12296,
                  12300
                ],
                "list"
              ]
            },
            {
              "Case": [
                [
                  12313,
                  12390
                ],
                {
                  "Ref": [
                    [
                      12318,
                      12322
                    ],
                    "list"
                  ]
                },
                [
                  [
                    {
                      "List": [
                        [
                          12332,
                          12334
                        ],
                        []
                      ]
                    },
                    {
                      "Ref": [
                        [
                          12346,
                          12350
                        ],
                        "list"
                      ]
                    }
                  ],
                  [
                    {
                      "BinaryOp": [
                        [
                          12358,
                          12365
                        ],
                        "::",
                        {
                          "Var": [
                            [
                              12358,
                              12359
                            ],
                            "x"
                          ]
                        },
                        {
                          "Var": [
                            [
                              12363,
                              12365
                            ],
                            "xs"
                          ]
                        }
                      ]
                    },
                    {
                      "Application": [
                        [
                          12377,
                          12390
                        ],
                        {
                          "Application": [
                            [
                              12377,
                              12390
                            ],
                            {
                              "Ref": [
                                [
                                  12377,
                                  12381
                                ],
                                "drop"
                              ]
                            },
                            {
                              "OpChain": [
                                [
                                  12383,
                                  12386
                                ],
                                [
                                  {
                                    "Ref": [
                                      [
                                        12383,
                                        12384
                                      ],
                                      "n"
                                    ]
                                  },
                                  {
                                    "Literal": [
                                      [
                                        12385,
                                        12386
                                      ],
                                      {
                                        "Int": 1
                                      }
                                    ]
                                  }
                                ],
                                [
                                  "-"
                                ]
                              ]
                            }
                          ]
                        },
                        {
                          "Ref": [
                            [
                              12388,
                              12390
                            ],
                            "xs"
                          ]
                        }
                      ]
                    }
                  ]
                ]
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
                  "Tag": [
                    "Bool",
                    []
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tuple": [
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "a"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "List",
                        [
                          {
                            "Var": "a"
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
            "Var": [
              [
                12744,
                12748
              ],
              "pred"
            ]
          },
          {
            "Var": [
              [
                12749,
                12753
              ],
              "list"
            ]
          }
        ],
        "expr": {
          "Let": [
            [
              12758,
              12918
            ],
            [
              {
                "Def": {
                  "header": null,
                  "name": "step",
                  "patterns": [
                    {
                      "Var": [
                        [
                          12771,
                          12772
                        ],
                        "x"
                      ]
                    },
                    {
                      "Tuple": [
                        [
                          12773,
                          12788
                        ],
                        [
                          {
                            "Var": [
                              [
                                12774,
                                12779
                              ],
                              "trues"
                            ]
                          },
                          {
                            "Var": [
                              [
                                12781,
                                12787
                              ],
                              "falses"
                            ]
                          }
                        ]
                      ]
                    }
                  ],
                  "expr": {
                    "If": [
                      [
                        12797,
                        12888
                      ],
                      {
                        "Application": [
                          [
                            12800,
                            12806
                          ],
                          {
                            "Ref": [
                              [
                                12800,
                                12804
                              ],
                              "pred"
                            ]
                          },
                          {
                            "Ref": [
                              [
                                12805,
                                12806
                              ],
                              "x"
                            ]
                          }
                        ]
                      },
                      {
                        "Tuple": [
                          [
                            12820,
                            12850
                          ],
                          [
                            {
                              "Application": [
                                [
                                  12821,
                                  12833
                                ],
                                {
                                  "Application": [
                                    [
                                      12821,
                                      12833
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          12821,
                                          12825
                                        ],
                                        "cons"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          12826,
                                          12827
                                        ],
                                        "x"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      12828,
                                      12833
                                    ],
                                    "trues"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  12835,
                                  12841
                                ],
                                "falses"
                              ]
                            }
                          ]
                        ]
                      },
                      {
                        "Tuple": [
                          [
                            12863,
                            12888
                          ],
                          [
                            {
                              "Ref": [
                                [
                                  12864,
                                  12869
                                ],
                                "trues"
                              ]
                            },
                            {
                              "Application": [
                                [
                                  12871,
                                  12884
                                ],
                                {
                                  "Application": [
                                    [
                                      12871,
                                      12884
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          12871,
                                          12875
                                        ],
                                        "cons"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          12876,
                                          12877
                                        ],
                                        "x"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      12878,
                                      12884
                                    ],
                                    "falses"
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
                  12895,
                  12918
                ],
                {
                  "Application": [
                    [
                      12895,
                      12918
                    ],
                    {
                      "Application": [
                        [
                          12895,
                          12918
                        ],
                        {
                          "Ref": [
                            [
                              12895,
                              12900
                            ],
                            "foldr"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              12901,
                              12905
                            ],
                            "step"
                          ]
                        }
                      ]
                    },
                    {
                      "Tuple": [
                        [
                          12906,
                          12914
                        ],
                        [
                          {
                            "List": [
                              [
                                12907,
                                12909
                              ],
                              []
                            ]
                          },
                          {
                            "List": [
                              [
                                12910,
                                12912
                              ],
                              []
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
                      12914,
                      12918
                    ],
                    "list"
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
                "List",
                [
                  {
                    "Tuple": [
                      {
                        "Var": "a"
                      },
                      {
                        "Var": "b"
                      }
                    ]
                  }
                ]
              ]
            },
            {
              "Tuple": [
                {
                  "Tag": [
                    "List",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "List",
                    [
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
        "name": "unzip",
        "patterns": [
          {
            "Var": [
              [
                13109,
                13114
              ],
              "pairs"
            ]
          }
        ],
        "expr": {
          "Let": [
            [
              13119,
              13211
            ],
            [
              {
                "Def": {
                  "header": null,
                  "name": "step",
                  "patterns": [
                    {
                      "Tuple": [
                        [
                          13132,
                          13137
                        ],
                        [
                          {
                            "Var": [
                              [
                                13133,
                                13134
                              ],
                              "x"
                            ]
                          },
                          {
                            "Var": [
                              [
                                13135,
                                13136
                              ],
                              "y"
                            ]
                          }
                        ]
                      ]
                    },
                    {
                      "Tuple": [
                        [
                          13138,
                          13145
                        ],
                        [
                          {
                            "Var": [
                              [
                                13139,
                                13141
                              ],
                              "xs"
                            ]
                          },
                          {
                            "Var": [
                              [
                                13142,
                                13144
                              ],
                              "ys"
                            ]
                          }
                        ]
                      ]
                    }
                  ],
                  "expr": {
                    "Tuple": [
                      [
                        13154,
                        13179
                      ],
                      [
                        {
                          "Application": [
                            [
                              13155,
                              13164
                            ],
                            {
                              "Application": [
                                [
                                  13155,
                                  13164
                                ],
                                {
                                  "Ref": [
                                    [
                                      13155,
                                      13159
                                    ],
                                    "cons"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      13160,
                                      13161
                                    ],
                                    "x"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  13162,
                                  13164
                                ],
                                "xs"
                              ]
                            }
                          ]
                        },
                        {
                          "Application": [
                            [
                              13166,
                              13175
                            ],
                            {
                              "Application": [
                                [
                                  13166,
                                  13175
                                ],
                                {
                                  "Ref": [
                                    [
                                      13166,
                                      13170
                                    ],
                                    "cons"
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      13171,
                                      13172
                                    ],
                                    "y"
                                  ]
                                }
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  13173,
                                  13175
                                ],
                                "ys"
                              ]
                            }
                          ]
                        }
                      ]
                    ]
                  }
                }
              }
            ],
            {
              "Application": [
                [
                  13186,
                  13211
                ],
                {
                  "Application": [
                    [
                      13186,
                      13211
                    ],
                    {
                      "Application": [
                        [
                          13186,
                          13211
                        ],
                        {
                          "Ref": [
                            [
                              13186,
                              13191
                            ],
                            "foldr"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              13192,
                              13196
                            ],
                            "step"
                          ]
                        }
                      ]
                    },
                    {
                      "Tuple": [
                        [
                          13197,
                          13206
                        ],
                        [
                          {
                            "List": [
                              [
                                13198,
                                13200
                              ],
                              []
                            ]
                          },
                          {
                            "List": [
                              [
                                13202,
                                13204
                              ],
                              []
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
                      13206,
                      13211
                    ],
                    "pairs"
                  ]
                }
              ]
            }
          ]
        }
      }
    }
  ]
}